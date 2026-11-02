//! STATS (`stats_menu.xml`, the `StatsMenu` class): five pages along its
//! tail line (Status, S.P.E.C.I.A.L., Skills, Perks, General), the page
//! shown being the menu's `user0`. Filled as `007da2c0` (setup), `007dcfc0`
//! / `007dd090` (values) and `007dff40` (page changes) fill it.

use super::{by_id, text, trait_id, Action, Key, PipboyInput, StatLine};
use crate::listbox::ListBox;
use crate::names::t;
use crate::tile::{TileId, Ui};

/// The limbs' pictures by actor value 25 to 30 (the names at `011a0ae4`:
/// `Interface\Stats\<name>.dds`, `<name>_broken.dds` at 0 condition).
pub const LIMB_PICTURES: [&str; 6] = [
    "head",
    "torso",
    "left_arm",
    "right_arm",
    "left_leg",
    "right_leg",
];

/// The limb pictures' tiles and their meters (`id`s 12/13 head, 14/15
/// torso, 16/17 left arm, 18/19 right arm, 20/21 left leg, 22/23 right
/// leg).
const LIMB_IDS: [(i32, i32); 6] = [(12, 13), (14, 15), (16, 17), (18, 19), (20, 21), (22, 23)];

/// The karma pictures by band (`007dd090`, `0047e040`'s order: good,
/// neutral, bad, very good, very evil).
pub const KARMA_PICTURES: [&str; 5] = [
    "Interface\\Icons\\PipboyImages\\Karma icons\\karma_good.dds",
    "Interface\\Icons\\PipboyImages\\Karma icons\\karma_neutral.dds",
    "Interface\\Icons\\PipboyImages\\Karma icons\\karma_bad.dds",
    "Interface\\Icons\\PipboyImages\\Karma icons\\karma_saintly.dds",
    "Interface\\Icons\\PipboyImages\\Karma icons\\karma_evil.dds",
];

/// The STATS menu.
pub struct StatsMenu {
    pub menu: TileId,
    /// The page shown (0 Status .. 4 General).
    pub page: usize,
    /// The Status page's sub-page (0 CND, 1 RAD, 2 EFF).
    pub status_mode: usize,
    pub special: ListBox,
    pub skills: ListBox,
    pub perks: ListBox,
    pub general: ListBox,
    pub reputation: ListBox,
    pub effects: ListBox,
    status: Option<TileId>,
    /// The General page's holder: its `user0` 0 shows the misc
    /// statistics and karma, 1 (the file's start) the reputations.
    genrep: Option<TileId>,
    icon: Option<TileId>,
    badge: Option<TileId>,
    description: Option<TileId>,
    /// What the lists were last filled from (refilled only when it
    /// changes, so choices and scrolling stay).
    filled: Option<ListSources>,
    /// Per page, the last row the pointer was over (`007dfd20`'s
    /// `011dad60` .. `011dad70`).
    pub(crate) hovered: [Option<usize>; 5],
    /// Healing mode (`+0x2a0`, `StatsMenu::ToggleHealingMode` (Xbox PDB)
    /// `007e0230`) and the limb it aims at (the body part controller,
    /// `007db1c0` / `007dbfa0`; by `LIMB_IDS`' order).
    pub healing: bool,
    pub part: Option<usize>,
}

/// What the STATS lists are made from: the rest of the game's state
/// (health, the date, ...) changes without the rows being made again, so
/// the pointer's row and the scrolling stay.
#[derive(Debug, Clone, PartialEq)]
struct ListSources {
    special: Vec<StatLine>,
    skills: Vec<StatLine>,
    perks: Vec<StatLine>,
    general: Vec<StatLine>,
    reputations: Vec<super::ReputationLine>,
    effects: Vec<(String, String)>,
}

impl ListSources {
    fn of(input: &PipboyInput) -> ListSources {
        ListSources {
            special: input.special.clone(),
            skills: input.skills.clone(),
            perks: input.perks.clone(),
            general: input.general.clone(),
            reputations: input.reputations.clone(),
            effects: input.effects.clone(),
        }
    }
}

impl StatsMenu {
    /// Reads the menu and sets the texts its maker sets (`007da2c0`'s end:
    /// the title, the cards' titles, the CND and RAD buttons, `user9`
    /// crippled, `user10` effects, the rad resistance label, "no effects",
    /// the tail line's five buttons).
    pub fn load(
        ui: &mut Ui,
        read: &mut dyn FnMut(&str) -> Option<Vec<u8>>,
    ) -> Result<StatsMenu, String> {
        let menu = super::load_menu(ui, super::STATS_FILE, read)?;
        let list = |ui: &mut Ui, id: i32, template: &str| {
            let tile = by_id(ui, menu, id).unwrap_or(menu);
            ListBox::new(menu, tile, template)
        };
        let special = list(ui, 6, "stats_list_template");
        let skills = list(ui, 7, "stats_list_template");
        let perks = list(ui, 34, "stats_list_template");
        let general = list(ui, 8, "stats_list_template");
        let reputation = list(ui, 46, "stats_list_template");
        let effects = list(ui, 33, "stats_effects_template");
        let mut s = StatsMenu {
            menu,
            page: 0,
            status_mode: 0,
            special,
            skills,
            perks,
            general,
            reputation,
            effects,
            status: by_id(ui, menu, 5),
            genrep: by_id(ui, menu, 50),
            icon: by_id(ui, menu, 25),
            badge: by_id(ui, menu, 52),
            description: ui.find(menu, "stats_description"),
            filled: None,
            hovered: [None; 5],
            healing: false,
            part: None,
        };
        let set = |ui: &mut Ui, tile: Option<TileId>, trait_id: i32, setting: &str| {
            if let Some(tile) = tile {
                let v = text(ui, setting);
                ui.set_string(tile, trait_id, &v);
            }
        };
        let title = trait_id(ui, "_Title");
        set(ui, by_id(ui, menu, 39), t::STRING, "sStats");
        set(ui, by_id(ui, menu, 40), title, "sStatsLVLAbbrev");
        set(ui, by_id(ui, menu, 41), title, "sStatsHP");
        set(ui, by_id(ui, menu, 42), title, "sStatsAP");
        set(ui, by_id(ui, menu, 43), title, "sStatsXP");
        set(ui, by_id(ui, menu, 28), t::STRING, "sStatsCNDAbbrev");
        set(ui, by_id(ui, menu, 29), t::STRING, "sStatsRADAbbrev");
        set(ui, Some(menu), t::USER0 + 9, "sStatsCrippled");
        set(ui, Some(menu), t::USER0 + 10, "sStatsEFFAbbrev");
        set(ui, by_id(ui, menu, 44), t::STRING, "sStatsRadResist");
        set(ui, by_id(ui, menu, 45), t::STRING, "sStatsNoEffects");
        // The hardcore needs' labels (`_h20`, `_hunger`, `_sleep`) and the
        // rads' (`_rads`, `sHUDRads`) on the menu.
        for (name, setting) in [
            ("_rads", "sHUDRads"),
            ("_h20", "sStatsH20Abbrev"),
            ("_hunger", "sStatsFODAbbrev"),
            ("_sleep", "sStatsSLPAbbrev"),
        ] {
            let id = trait_id(ui, name);
            set(ui, Some(menu), id, setting);
        }
        for (i, setting) in [
            "sStatsStatus",
            "sStatsSpecial",
            "sStatsSkills",
            "sStatsPerks",
            "sStatsGeneral",
        ]
        .iter()
        .enumerate()
        {
            set(ui, by_id(ui, menu, i as i32), t::STRING, setting);
        }
        s.set_page(ui, 0, None);
        Ok(s)
    }

    /// Fills the values (`007dcfc0`, `007dd090`) and, when the game's state
    /// changed, the lists.
    pub fn fill(&mut self, ui: &mut Ui, input: &PipboyInput) {
        let m = self.menu;
        // The headline: level (`user8`), "%d/%d" health (`user5`) and action
        // points (`user6`), "%d/%d" experience (`user7`) or `sStatsXPMax`.
        ui.set_number(m, t::USER0 + 8, input.level as f32);
        let pair = |v: (f32, f32)| format!("{}/{}", v.0 as i32, v.1 as i32);
        ui.set_string(m, t::USER0 + 5, &pair(input.health));
        ui.set_string(m, t::USER0 + 6, &pair(input.action_points));
        let xp = match input.xp {
            Some((now, next)) => format!("{now}/{next}"),
            None => text(ui, "sStatsXPMax"),
        };
        ui.set_string(m, t::USER0 + 7, &xp);
        // The name line (`007df4e0`: "%s - %s %hd", the name, `sMenuDisplay
        // LevelString`, the level).
        if let Some(name) = by_id(ui, m, 27) {
            let level = text(ui, "sMenuDisplayLevelString");
            ui.set_string(
                name,
                t::STRING,
                &format!("{} - {level} {}", input.name, input.level),
            );
        }
        // The limbs (`007dd090`, actor values 25 .. 30): their pictures,
        // broken at 0, and meters at condition / 100.
        let value = trait_id(ui, "_Value");
        for (i, &(picture, meter)) in LIMB_IDS.iter().enumerate() {
            let condition = input.limbs[i];
            if let Some(tile) = by_id(ui, m, picture) {
                let file = if condition <= 0.0 {
                    format!("Interface\\Stats\\{}_broken.dds", LIMB_PICTURES[i])
                } else {
                    format!("Interface\\Stats\\{}.dds", LIMB_PICTURES[i])
                };
                ui.set_string(tile, t::FILENAME, &file);
            }
            if let Some(tile) = by_id(ui, m, meter) {
                ui.set_number(tile, value, condition / 100.0);
            }
        }
        // The Vault Boy's face (`007dfe40`): trunc((most − health) / most ×
        // 5), 0 to 4. (An active effect of archetype 10 gives face 10
        // instead: not done.)
        if let Some(face) = by_id(ui, m, 24) {
            let share = if input.health.1 > 0.0 {
                (input.health.0 / input.health.1).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let index = (((1.0 - share) * 5.0) as i32).clamp(0, 4);
            ui.set_string(
                face,
                t::FILENAME,
                &format!("Interface\\Stats\\face_{index:02}.dds"),
            );
        }
        // Rads (`007ddb00`): `user1` the player's rounded to a whole number
        // (halves up), but 1 between 0 and 1 (at least 0); rad resistance
        // "%d%%" (`010774c4`). (The Pip-Boy's needle turning with them isn't
        // done.)
        let rads = input.rads.max(0.0);
        let shown = if rads > 0.0 && rads < 1.0 {
            1.0
        } else {
            let whole = rads.trunc();
            whole + if rads - whole >= 0.5 { 1.0 } else { 0.0 }
        };
        ui.set_number(m, t::USER0 + 1, shown);
        if let Some(level) = by_id(ui, m, 38) {
            ui.set_string(
                level,
                t::STRING,
                &format!("{}%", input.rad_resistance as i32),
            );
        }
        // The aid buttons (`007df230`): the item's name (the button's
        // `user0`), "(%d) %s" with the count when there are any; clickable
        // (`target`) with some (for the Stimpak and the Doctor's Bag
        // outside hardcore, as the code sets; for RadAway and Rad-X the
        // code asks its own tests, `007e07a0` rads above 0 for RadAway:
        // taken as having some).
        for (slot, &id) in AID_BUTTONS.iter().enumerate() {
            let Some(tile) = by_id(ui, m, id) else {
                continue;
            };
            let name = &input.aid_names[slot];
            let count = aid_count(input, slot);
            ui.set_string(tile, t::USER0, name);
            let text = if count > 0 {
                format!("({count}) {name}")
            } else {
                name.clone()
            };
            ui.set_string(tile, t::STRING, &text);
            ui.set_number(tile, t::TARGET, if count > 0 { 1.0 } else { 0.0 });
        }
        // The hardcore needs' buttons show only in hardcore (`007dc4f0`).
        for id in [53, 55, 57] {
            if let Some(tile) = by_id(ui, m, id) {
                let on = if input.hardcore { 1.0 } else { 0.0 };
                ui.set_number(tile, t::VISIBLE, on);
                ui.set_number(tile, t::TARGET, on);
            }
        }
        // Karma (`007dd090` case 0x17, `007df5d0`).
        if let Some(image) = by_id(ui, m, 36) {
            let band = usize::from(input.karma_band).min(4);
            ui.set_string(image, t::FILENAME, KARMA_PICTURES[band]);
        }
        if let Some(a) = by_id(ui, m, 35) {
            ui.set_string(a, t::STRING, &input.alignment);
        }
        if let Some(title) = by_id(ui, m, 37) {
            ui.set_string(title, t::STRING, &input.karma_title);
        }
        let sources = ListSources::of(input);
        if self.filled.as_ref() != Some(&sources) {
            self.fill_lists(ui, input);
            self.filled = Some(sources);
        }
    }

    fn fill_lists(&mut self, ui: &mut Ui, input: &PipboyInput) {
        let fill = |ui: &mut Ui, list: &mut ListBox, lines: &[StatLine]| {
            let keep = list.selected;
            list.clear(ui);
            for line in lines {
                let Some(row) = list.add(ui, Some(&line.name)) else {
                    continue;
                };
                // `user1` the number (-1 hides it).
                ui.set_number(row, t::USER0 + 1, line.value.map_or(-1.0, |v| v as f32));
            }
            if !lines.is_empty() {
                list.select(ui, Some(keep.unwrap_or(0).min(lines.len() - 1)));
            }
        };
        fill(ui, &mut self.special, &input.special);
        fill(ui, &mut self.skills, &input.skills);
        fill(ui, &mut self.perks, &input.perks);
        fill(ui, &mut self.general, &input.general);
        let reps: Vec<StatLine> = input
            .reputations
            .iter()
            .map(|r| StatLine {
                name: r.name.clone(),
                ..StatLine::default()
            })
            .collect();
        fill(ui, &mut self.reputation, &reps);
        // Effects: `user0` the name, `user1` what it does, `user2` the
        // divider (not under the last).
        self.effects.clear(ui);
        let n = input.effects.len();
        for (i, (name, what)) in input.effects.iter().enumerate() {
            if let Some(row) = self.effects.add(ui, None) {
                ui.set_string(row, t::USER0, name);
                ui.set_string(row, t::USER0 + 1, what);
                ui.set_number(row, t::USER0 + 2, if i + 1 < n { 1.0 } else { 0.0 });
            }
        }
        self.show_selected(ui, input);
    }

    /// The picture and description of what's chosen on the page
    /// (`007dc7e0` SPECIAL and skills: the actor value's `ICON` and `DESC`;
    /// `007dc9b0` perks; `007dcb40` reputations), or none.
    fn show_selected(&mut self, ui: &mut Ui, input: &PipboyInput) {
        let line: Option<StatLine> = match self.page {
            1 => pick(&self.special, &input.special),
            2 => pick(&self.skills, &input.skills),
            3 => pick(&self.perks, &input.perks),
            _ => None,
        };
        let Some(icon) = self.icon else {
            return;
        };
        match line {
            Some(line) => {
                ui.set_number(icon, t::VISIBLE, 1.0);
                ui.set_string(icon, t::FILENAME, line.icon.as_deref().unwrap_or(""));
                if let Some(d) = self.description {
                    ui.set_string(d, t::STRING, &line.description);
                }
            }
            None => ui.set_number(icon, t::VISIBLE, 0.0),
        }
        if let Some(b) = self.badge {
            ui.set_number(b, t::VISIBLE, 0.0);
        }
        // The reputation page's picture and titles.
        if self.page == 4 {
            if let Some(r) = self
                .reputation
                .selected
                .and_then(|i| input.reputations.get(i))
            {
                if let Some(image) = by_id(ui, self.menu, 47) {
                    ui.set_string(image, t::FILENAME, r.icon.as_deref().unwrap_or(""));
                }
                if let Some(title) = by_id(ui, self.menu, 48) {
                    ui.set_string(title, t::STRING, &r.name);
                }
                if let Some(title) = by_id(ui, self.menu, 51) {
                    ui.set_string(title, t::STRING, &r.title);
                }
            }
        }
    }

    /// Shows a page (0 Status .. 4 General).
    pub fn show_page(&mut self, ui: &mut Ui, page: usize, input: &PipboyInput) {
        self.set_page(ui, page, Some(input));
    }

    /// Whether the General page shows reputations (else the misc
    /// statistics).
    fn showing_reputations(&self, ui: &mut Ui) -> bool {
        match self.genrep {
            Some(g) => ui.number(g, t::USER0) != 0.0,
            None => true,
        }
    }

    /// The list on the page shown.
    fn page_list(&mut self, ui: &mut Ui) -> Option<&mut ListBox> {
        let reputations = self.showing_reputations(ui);
        match self.page {
            1 => Some(&mut self.special),
            2 => Some(&mut self.skills),
            3 => Some(&mut self.perks),
            4 if reputations => Some(&mut self.reputation),
            4 => Some(&mut self.general),
            _ => None,
        }
    }

    /// Shows a page (`007dff40`): the menu's `user0`, a burst on the screen
    /// (the caller's [`super::screen::ScreenEffects::tab_changed`]) and the
    /// tab knob turning (`007dfc90`, `UIPipBoyTab`).
    fn set_page(&mut self, ui: &mut Ui, page: usize, input: Option<&PipboyInput>) {
        self.page = page.min(4);
        // The pages' lists show and take keys by this (their `visible` and
        // `_enabled` read it in the file).
        ui.set_number(self.menu, t::USER0, self.page as f32);
        if let Some(input) = input {
            self.show_selected(ui, input);
        }
    }

    /// A key (`007db680`): left and right change page (wrapping), up and
    /// down move in the page's list (the Status page's CND / RAD / EFF
    /// with them), the pad's buttons press the Status page's aid buttons.
    /// (Letters go through the menu's `_PCButton_` traits: [`super::Pipboy::key`].)
    pub fn key(&mut self, ui: &mut Ui, key: Key, input: &PipboyInput) -> Vec<Action> {
        let mut out = Vec::new();
        match key {
            Key::Left | Key::Right => {
                let page = if key == Key::Right {
                    (self.page + 1) % 5
                } else {
                    (self.page + 4) % 5
                };
                self.set_page(ui, page, Some(input));
                out.push(Action::Sound("UIPipBoyTab".into()));
            }
            Key::Up | Key::Down => {
                let by = if key == Key::Down { 1 } else { -1 };
                if self.page == 0 {
                    // CND, RAD, EFF (`007db680` keys 1 and 2 on the
                    // Status page; the hardcore needs' three after them).
                    let modes = if input.hardcore { 6 } else { 3 };
                    let next = (self.status_mode as i32 + by).clamp(0, modes - 1) as usize;
                    if next != self.status_mode {
                        self.status_mode = next;
                        if let Some(s) = self.status {
                            ui.set_number(s, t::USER0, next as f32);
                        }
                        out.push(Action::Sound("UIPipBoySelect".into()));
                    }
                } else if let Some(list) = self.page_list(ui) {
                    let before = list.selected;
                    list.step(ui, by);
                    if list.selected != before {
                        out.push(Action::Sound("UIPipBoyScroll".into()));
                    }
                    self.show_selected(ui, input);
                }
            }
            // On the Status page (`007db680`): the A button presses the
            // Stimpak's button under CND, RadAway's under RAD; Y the Doctor's
            // Bag's under CND; X and Y Rad-X's under RAD. (X under CND
            // chooses a limb to heal: not here yet.)
            Key::Activate | Key::ButtonX | Key::ButtonY if self.page == 0 => {
                let button = match (key, self.status_mode) {
                    (Key::Activate, 0) => Some(10),
                    (Key::Activate, 1) => Some(11),
                    (Key::ButtonY, 0) => Some(59),
                    (Key::ButtonX | Key::ButtonY, 1) => Some(31),
                    _ => None,
                };
                if let Some(id) = button {
                    out.extend(self.click(ui, id, input));
                }
            }
            _ => {}
        }
        out
    }

    /// A button pressed (`007db380`): an aid button uses its item when
    /// there's one; `stats_genrep_button` (id 49, `007e02b0`) switches the
    /// General page between the misc statistics and the reputations; the
    /// tail line's buttons (ids 0–4) turn to their page.
    pub fn click(&mut self, ui: &mut Ui, id: i32, input: &PipboyInput) -> Vec<Action> {
        let mut out = Vec::new();
        // A limb clicked in healing mode (`007db380`'s end): the Stimpak's
        // button (10), in hardcore the Doctor's Bag's (0x3b), pressed for
        // it.
        let limb = (12..=24).contains(&id) && id % 2 == 0;
        if limb && self.page == 0 {
            if self.healing {
                let button = if input.hardcore { 0x3b } else { 10 };
                return self.click(ui, button, input);
            }
            return out;
        }
        if let Some(slot) = AID_BUTTONS.iter().position(|&b| b == id) {
            if aid_count(input, slot) > 0 {
                if let Some(form) = input.aid[slot] {
                    out.push(Action::Use(form));
                }
            }
        } else if id == 49 {
            if let Some(g) = self.genrep {
                let now = ui.number(g, t::USER0);
                ui.set_number(g, t::USER0, if now != 0.0 { 0.0 } else { 1.0 });
            }
        } else if (0..5).contains(&id) {
            self.set_page(ui, id as usize, Some(input));
            out.push(Action::Sound("UIPipBoyTab".into()));
        } else if let Some(mode) = status_mode_of(id) {
            // `007e0060`: a mode the page has (CND, RAD, EFF; the three
            // needs only in hardcore) other than the one shown becomes the
            // status tile's `user0`. (The buttons' own `clicksound`,
            // `UIPipboySelect`, is the file's: the interface plays it.)
            let last = if input.hardcore { 5 } else { 2 };
            if mode <= last && mode != self.status_mode {
                self.status_mode = mode;
                if let Some(s) = self.status {
                    ui.set_number(s, t::USER0, mode as f32);
                }
            }
        }
        out
    }

    /// The pointer onto a tile (`007dc1b0`, slot 0x10): on the
    /// S.P.E.C.I.A.L., Skills, Perks and General pages a row of the page's
    /// list becomes the chosen one and its picture and description show;
    /// the knob clicks (`007dfd20` → `007f8610`, `UIPipBoyScroll`) when its
    /// `listindex` differs from the last one the pointer was over on that
    /// page. On the Status page a damaged limb aims healing mode at it.
    /// (The aimed limb's picture blinking, `007dbfe0` animating its alpha,
    /// isn't here.)
    pub fn mouseover(&mut self, ui: &mut Ui, tile: TileId, input: &PipboyInput) -> Vec<Action> {
        let mut out = Vec::new();
        // The Status page (`007dc1b0` case 0): the pointer onto a limb
        // (only damaged ones are targets, the file's `target`) with a
        // Stimpak to use (a Doctor's Bag in hardcore, `004d1360`) aims the
        // body part controller at it with `UIPipBoyHighlight` and turns
        // healing mode on when it's off.
        if self.page == 0 {
            if let Some(part) = limb_of(ui, tile) {
                let have = if input.hardcore {
                    input.doctors_bags
                } else {
                    input.stimpaks
                };
                if have > 0 {
                    self.part = Some(part);
                    out.push(Action::Sound("UIPipBoyHighlight".into()));
                    if !self.healing {
                        self.healing = true;
                    }
                }
            }
            return out;
        }
        let page = self.page;
        let Some(list) = self.page_list(ui) else {
            return out;
        };
        let Some(index) = list.index_of(tile) else {
            return out;
        };
        list.choose(ui, Some(index));
        let last = &mut self.hovered[page.min(4)];
        if *last != Some(index) {
            *last = Some(index);
            out.push(Action::Sound("UIPipBoyScroll".into()));
        }
        self.show_selected(ui, input);
        out
    }
}

impl StatsMenu {
    /// The pointer off a tile (`007dc490`, slot 0x14): leaving a limb in
    /// healing mode turns it off.
    pub fn unmouseover(&mut self, ui: &mut Ui, tile: TileId) {
        if self.healing && limb_of(ui, tile).is_some() {
            self.healing = false;
        }
    }

    /// The limb healing mode aims at, as its condition's actor value (25
    /// head .. 30 right leg, `007e06b0`), while it's on.
    pub fn healing_part(&self) -> Option<u16> {
        self.part
            .filter(|_| self.healing)
            .map(|i| LIMB_CONDITION_AVS[i])
    }
}

/// The limbs' condition actor values in `LIMB_IDS`' order (the meters
/// `007dd090` fills: 25 head, 26 torso, 27 left arm, 28 right arm, 29
/// left leg, 30 right leg).
const LIMB_CONDITION_AVS: [u16; 6] = [25, 26, 27, 28, 29, 30];

/// The limb a Status-page tile is (`007db630` against the controller's
/// tiles): a limb picture's `id`, the face (24, part of the head).
/// The body part controller's tiles for a limb (its picture; the head's
/// face too), which blink while healing mode aims at it (`007dbfe0`).
pub fn limb_tiles(ui: &Ui, menu: TileId, part: usize) -> Vec<TileId> {
    let mut ids = vec![LIMB_IDS[part].0];
    if part == 0 {
        ids.push(24);
    }
    ids.into_iter()
        .filter_map(|id| by_id(ui, menu, id))
        .collect()
}

fn limb_of(ui: &mut Ui, tile: TileId) -> Option<usize> {
    let id = ui.number(tile, t::ID) as i32;
    if id == 24 {
        return Some(0);
    }
    LIMB_IDS.iter().position(|&(picture, _)| picture == id)
}

/// The Status page's mode buttons by `id` (`007e0160`): 0x1c CND, 0x1d
/// RAD, 0x1e EFF, then the hardcore needs 0x35, 0x37, 0x39.
fn status_mode_of(id: i32) -> Option<usize> {
    Some(match id {
        0x1c => 0,
        0x1d => 1,
        0x1e => 2,
        0x35 => 3,
        0x37 => 4,
        0x39 => 5,
        _ => return None,
    })
}

/// The aid buttons' `id`s by slot: Stimpak, Doctor's Bag, RadAway, Rad-X
/// (`007da2c0` keeps the tiles of ids 10, 59, 11, 31 with default objects
/// 0, 21, 3, 2).
const AID_BUTTONS: [i32; 4] = [10, 59, 11, 31];

/// How many of a slot's aid item the player carries.
fn aid_count(input: &PipboyInput, slot: usize) -> i32 {
    [
        input.stimpaks,
        input.doctors_bags,
        input.radaway,
        input.radx,
    ][slot]
}

/// The chosen line of a list.
fn pick(list: &ListBox, lines: &[StatLine]) -> Option<StatLine> {
    list.selected.and_then(|i| lines.get(i)).cloned()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// `stats_menu.xml` cut down to the tiles the code finds by `id` and
    /// name.
    pub(crate) const MENU: &str = r#"<menu name="StatsMenu"><locus>&true;</locus>
      <_PCButton_S> stats_stimpak_button </_PCButton_S>
      <_PCButton_A> stats_radaway_button </_PCButton_A>
      <text name="title"><id>39</id></text>
      <text name="name_line"><id>27</id></text>
      <image name="head"><id>12</id></image><rect name="head_meter"><id>13</id></rect>
      <image name="torso"><id>14</id></image><rect name="torso_meter"><id>15</id></rect>
      <image name="face"><id>24</id></image>
      <text name="rad_resist"><id>38</id></text>
      <text name="stats_stimpak_button"><id>10</id><visible>&true;</visible><target>&true;</target></text>
      <text name="stats_radaway_button"><id>11</id><visible>&true;</visible><target>&true;</target></text>
      <rect name="status"><id>5</id></rect>
      <hotrect name="special"><id>6</id><x>0</x><y>100</y><width>300</width><height>400</height>
        <_scroll_delta>0</_scroll_delta><_highlight_y>-1</_highlight_y><_selected_height>0</_selected_height>
        <image name="lb_scrollbar"><_number_of_items>1</_number_of_items><_current_value>0</_current_value></image>
      </hotrect>
      <template name="stats_list_template"><hotrect name="row"><height>30</height></hotrect></template>
      <image name="stats_icon"><id>25</id></image>
      <text name="stats_description"></text>
    </menu>"#;

    pub(crate) fn input() -> PipboyInput {
        let line = |name: &str, value: i32| StatLine {
            name: name.into(),
            value: Some(value),
            description: format!("{name} is a measure."),
            icon: Some(format!("{name}.dds")),
        };
        PipboyInput {
            name: "Courier".into(),
            level: 3,
            xp: None,
            health: (150.0, 200.0),
            action_points: (80.0, 80.0),
            limbs: [0.0, 50.0, 100.0, 100.0, 100.0, 100.0],
            rad_resistance: 25.0,
            stimpaks: 2,
            aid: [Some(0x15169), Some(0xCB05C), Some(0x15167), Some(0x15168)],
            aid_names: [
                "Stimpak".into(),
                "Doctor's Bag".into(),
                "RadAway".into(),
                "Rad-X".into(),
            ],
            special: vec![line("Strength", 6), line("Perception", 4)],
            ..PipboyInput::default()
        }
    }

    #[test]
    fn the_status_page_and_the_lists_are_filled_as_the_code_fills_them() {
        let mut ui = crate::pipboy::tests::ui();
        let mut read = |p: &str| (p == crate::pipboy::STATS_FILE).then(|| MENU.as_bytes().to_vec());
        let mut s = StatsMenu::load(&mut ui, &mut read).unwrap();
        let input = input();
        s.fill(&mut ui, &input);
        ui.refresh();
        let m = s.menu;
        let string = |ui: &mut Ui, id: i32, tr: i32| {
            let tile = by_id(ui, m, id).unwrap();
            ui.string(tile, tr).unwrap_or_default()
        };
        assert_eq!(string(&mut ui, 39, t::STRING), "STATS");
        // "%s - %s %d" with `sMenuDisplayLevelString`.
        assert_eq!(string(&mut ui, 27, t::STRING), "Courier - Level 3");
        // A limb at 0 shows broken; meters at condition / 100.
        assert_eq!(
            string(&mut ui, 12, t::FILENAME),
            "Interface\\Stats\\head_broken.dds"
        );
        assert_eq!(
            string(&mut ui, 14, t::FILENAME),
            "Interface\\Stats\\torso.dds"
        );
        let value = ui.names.lookup("_Value").unwrap();
        let meter = by_id(&ui, m, 15).unwrap();
        assert_eq!(ui.number(meter, value), 0.5);
        assert_eq!(string(&mut ui, 38, t::STRING), "25%");
        // "(%d) %s" with some, the name alone without (and then not
        // clickable).
        assert_eq!(string(&mut ui, 10, t::STRING), "(2) Stimpak");
        assert_eq!(string(&mut ui, 11, t::STRING), "RadAway");
        let radaway = by_id(&ui, m, 11).unwrap();
        assert_eq!(ui.number(radaway, t::TARGET), 0.0);
        // At the top level the XP reads `sStatsXPMax`.
        assert_eq!(ui.string(m, t::USER0 + 7).unwrap(), "MAX");
        assert_eq!(ui.string(m, t::USER0 + 5).unwrap(), "150/200");
        // The SPECIAL list: a row each, the number in `user1`.
        assert_eq!(s.special.rows.len(), 2);
        assert_eq!(ui.number(s.special.rows[1], t::USER0 + 1), 4.0);
        // Page 1 shows the chosen one's picture and description.
        s.show_page(&mut ui, 1, &input);
        let icon = by_id(&ui, m, 25).unwrap();
        assert_eq!(ui.string(icon, t::FILENAME).unwrap(), "Strength.dds");
        assert_eq!(
            s.key(&mut ui, Key::Down, &input),
            [Action::Sound("UIPipBoyScroll".into())]
        );
        assert_eq!(ui.string(icon, t::FILENAME).unwrap(), "Perception.dds");
        let d = ui.find(m, "stats_description").unwrap();
        assert_eq!(ui.string(d, t::STRING).unwrap(), "Perception is a measure.");
    }

    /// `007dc1b0` / `007dc490` / `007db380`: the pointer onto a limb with
    /// Stimpaks turns healing mode on aimed at it; a click on it uses a
    /// Stimpak (its effects then aimed at that limb's condition); leaving
    /// it turns healing mode off; without Stimpaks nothing happens.
    #[test]
    fn healing_mode_aims_a_stimpak_at_a_limb() {
        let mut ui = crate::pipboy::tests::ui();
        let mut read = |p: &str| (p == crate::pipboy::STATS_FILE).then(|| MENU.as_bytes().to_vec());
        let mut s = StatsMenu::load(&mut ui, &mut read).unwrap();
        let input = input();
        s.fill(&mut ui, &input);
        let torso = by_id(&ui, s.menu, 14).unwrap();
        assert_eq!(
            s.mouseover(&mut ui, torso, &input),
            [Action::Sound("UIPipBoyHighlight".into())]
        );
        assert!(s.healing);
        assert_eq!(s.healing_part(), Some(26));
        assert_eq!(s.click(&mut ui, 14, &input), [Action::Use(0x15169)]);
        // The face is the head's.
        let face = by_id(&ui, s.menu, 24).unwrap();
        s.mouseover(&mut ui, face, &input);
        assert_eq!(s.healing_part(), Some(25));
        s.unmouseover(&mut ui, face);
        assert!(!s.healing);
        assert_eq!(s.healing_part(), None);
        // Not in healing mode, a limb click does nothing.
        assert!(s.click(&mut ui, 14, &input).is_empty());
        // No Stimpaks: no healing mode.
        let none = PipboyInput {
            stimpaks: 0,
            ..input.clone()
        };
        assert!(s.mouseover(&mut ui, torso, &none).is_empty());
        assert!(!s.healing);
    }

    #[test]
    fn pages_wrap_and_the_status_buttons_use_aid() {
        let mut ui = crate::pipboy::tests::ui();
        let mut read = |p: &str| (p == crate::pipboy::STATS_FILE).then(|| MENU.as_bytes().to_vec());
        let mut s = StatsMenu::load(&mut ui, &mut read).unwrap();
        let input = input();
        s.fill(&mut ui, &input);
        // The A button under CND presses the Stimpak's button; under RAD
        // RadAway's, which does nothing without any.
        assert_eq!(
            s.key(&mut ui, Key::Activate, &input),
            [Action::Use(0x15169)]
        );
        assert!(s.click(&mut ui, 11, &input).is_empty());
        // Down on the Status page: CND to RAD (the status tile's `user0`).
        s.key(&mut ui, Key::Down, &input);
        let status = by_id(&ui, s.menu, 5).unwrap();
        assert_eq!(ui.number(status, t::USER0), 1.0);
        // Left from Status wraps to General; the page is the menu's
        // `user0`.
        s.key(&mut ui, Key::Left, &input);
        assert_eq!(s.page, 4);
        assert_eq!(ui.number(s.menu, t::USER0), 4.0);
        s.key(&mut ui, Key::Right, &input);
        assert_eq!(s.page, 0);
    }
}
