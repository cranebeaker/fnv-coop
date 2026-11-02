//! Crafting (`menus\recipe_menu.xml`, class `RecipeMenu` 1077, vtable
//! `0107048c` in FalloutNV.exe): the recipes of a category on the left, the
//! one under the pointer's ingredients, skill and category on the right,
//! Accept to make it. Read from the code (`00726ff0` and the functions
//! after it; names from the Xbox 360 prototype's symbols, Xbox PDB). The
//! rules (what is listed, how many can be made, making) are
//! `world::crafting`'s; this is the menu's side:
//!
//! * opening (`RecipeMenu::Create` (Xbox PDB), `00726ff0`): list boxes on
//!   ids 3 and 6 (the code asks for `CM_list_template`, which the file lacks,
//!   so the lookup ends on its `RM_list_template`); the title (1)
//!   `sRecipes`, 13 `sSkillRequirement`, 11 `sIngredients`, 12 `sMadeAt`,
//!   Accept (7) `sAccept`, Exit (8) `sExit`, the two buttons then 20 units
//!   further left; then filled.
//! * filling (`00727680`): one line (template id 15) per recipe in the
//!   order given, its alpha 255, or 127 when none can be made; the filter's
//!   subcategories.
//! * the filter (clicks on 0, 1 and 2, `007274b0`; the arrows, special
//!   codes 4 and 3, `00728420`): the index (`0119fbf4`, -1 for all, kept
//!   from one opening to the next) steps down on 0 and up on 1 and 2,
//!   wrapping through -1; the title shows `sRecipes` or the subcategory's
//!   name; lines of other subcategories are filtered out (`00728d60`).
//! * the pointer on a recipe (`00727b10` with id 15): the item picture and
//!   card (9, 10) hidden, then the caller's [`Details`]: the category (4,
//!   alpha 127 when it isn't the menu's), the ingredients heading (11), the
//!   ingredient lines (6, alpha 127 when short), the skill (5, alpha 127
//!   when too low), Accept's `target` (only when it can be made) and the
//!   first product's picture (9).
//! * Accept (7, `007274b0` case 7): the quantity menu for how many (at most
//!   as many as can be made), then the making (`007284f0`), which closes
//!   the menu; Exit (8): closes it (`00727430`).
//!
//! * the item card (10, `item_stats_display.xml`; `00728da0`,
//!   `RecipeMenu::PopulateItemStatsDisplay` (Xbox PDB)), after the details,
//!   for the recipe's first product (the recipe's +0x44 output list, its
//!   first component's item): [`Card`] and [`RecipeMenu::show_card`]; the
//!   card then shown.
//!
//! Not here: the ingredient list's own pointer (an ingredient's card and
//! picture, `00727b10` with the list switched, `011d8ea5`), the
//! controller's list switching (special 0xd, 0xe) and the accept button's
//! cross-fade (`00728a70`).

use crate::list::ListBox;
use crate::menu::{self, MenuCode};
use crate::names::t;
use crate::tile::{TileId, Ui};

/// The menu's file.
pub const FILE: &str = "menus\\recipe_menu.xml";
/// Its class number (`00726f00`).
pub const CLASS: i32 = 1077;
/// A list line's id (the template's).
pub const LINE_ID: i32 = 15;
/// The template the code asks for.
const TEMPLATE: &str = "CM_list_template";

const LEFT_ARROW: usize = 0;
const TITLE: usize = 1;
const RIGHT_ARROW: usize = 2;
const RECIPES: usize = 3;
const MADE_AT: usize = 4;
const SKILL: usize = 5;
const INGREDIENTS: usize = 6;
const ACCEPT: usize = 7;
const EXIT: usize = 8;
const ICON: usize = 9;
const CARD: usize = 10;
const INGREDIENTS_TITLE: usize = 11;
const MADE_AT_TITLE: usize = 12;
const SKILL_TITLE: usize = 13;

/// The alphas the code gives lines and fields (`00700320(0xfa9, ..)`).
const BRIGHT: f32 = 255.0;
const DIM: f32 = 127.0;

/// A recipe line: its text ("name" or "name (n)"), whether it can be made
/// at least once, and its subcategory (a form ID) for the filter.
#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub text: String,
    pub makeable: bool,
    pub subcategory: u32,
}

/// What the right side shows for the recipe under the pointer.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Details {
    /// The recipe's category name, and whether it is dimmed (not the
    /// menu's category).
    pub made_at: String,
    pub made_at_dim: bool,
    /// For a recipe of one ingredient and several products, its
    /// subcategory's name (its products are listed then); `None` for
    /// `sIngredients`.
    pub heading: Option<String>,
    /// The ingredient (or product) lines: text, and whether dimmed.
    pub parts: Vec<(String, bool)>,
    /// "skill (have/need)", empty for none, and whether dimmed.
    pub skill: String,
    pub skill_dim: bool,
    /// Accept can be used.
    pub can_make: bool,
    /// The first product's picture.
    pub icon: Option<String>,
    /// The first product's item card (none without a product).
    pub card: Option<Card>,
}

/// The item card's numbers for a recipe's first product, worked out by
/// the caller (`ui::pipboy::gather::recipe_card`) as `00728da0` does for
/// the base item: no instance, so a full condition and no mods.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Card {
    pub kind: CardKind,
    /// Its value (`0048e8a0`, -1 without one) through `00647c00` with 1
    /// for the condition in percent (the exe passes 1.0 there), as written
    /// on the card.
    pub value: f32,
    /// Its weight (`0048ebc0`, -1 for none).
    pub weight: f32,
    /// Its effects' text (`00406620`; empty for none).
    pub effects: String,
}

/// What kind of card (`00728da0` by the item's form type).
#[derive(Debug, Clone, PartialEq, Default)]
pub enum CardKind {
    /// Anything else: weight and value (0xc).
    #[default]
    Other,
    /// Armour and clothing (form types 0x18, 0x1a): its damage resistance
    /// at full condition (`004be060(1.0)`, whole, rounded up; mask 0x1d).
    Armour { resistance: i32 },
    /// A weapon (0x28, mask 0xc3e): its DPS (`00645380`; none: not worked
    /// out, left empty), damage (`006450f0` at full condition) and
    /// projectiles a shot (`00525b20`), the strength it needs (`00663b60`)
    /// and its ammunition's card title ("--" without).
    Weapon {
        dps: Option<f32>,
        damage: f32,
        projectiles: u32,
        strength: i32,
        ammo: String,
    },
}

/// The card's children by their place in `item_stats_display.xml`
/// (`00728da0` takes the first 12 in order; the 13th, the damage
/// threshold card, is left alone).
mod card {
    pub const DR: usize = 0;
    pub const DPS: usize = 1;
    pub const WEIGHT: usize = 2;
    pub const VALUE: usize = 3;
    pub const CONDITION: usize = 4;
    pub const AMMO: usize = 5;
    pub const EFFECTS: usize = 6;
    pub const MOD_ONE: usize = 7;
    pub const MOD_TWO: usize = 8;
    pub const MOD_THREE: usize = 9;
    pub const STRENGTH: usize = 10;
    pub const DAMAGE: usize = 11;
}

/// A weight or value as the card writes it (`00728da0`): "--" for
/// nothing, `%.1f` below 1, else `%.0f`.
fn amount_text(v: f32) -> String {
    if v <= 0.0 {
        "--".into()
    } else if v < 1.0 {
        format!("{v:.1}")
    } else {
        format!("{v:.0}")
    }
}

/// What the menu asks of the game.
#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    /// The pointer is on this line (its index): give its details.
    Show(usize),
    /// Accept on this line: ask how many and make them.
    Make(usize),
    /// Exit.
    Close,
}

#[derive(Debug)]
pub struct RecipeMenu {
    pub menu: TileId,
    pub tiles: [Option<TileId>; 14],
    /// The recipes (`+0x6c`) and the ingredients (`+0xa0`).
    pub recipes: ListBox,
    pub parts: ListBox,
    pub lines: Vec<Line>,
    /// The filter's subcategories: form ID and name.
    pub filters: Vec<(u32, String)>,
    /// The filter's index (`0119fbf4`): -1 for all.
    pub filter: i32,
    /// The line whose details are shown (`011d8e94`).
    pub shown: Option<usize>,
    pub requests: Vec<Request>,
    pub closed: bool,
}

fn setting(ui: &Ui, name: &str) -> String {
    ui.setting_text(name).unwrap_or_default()
}

/// A line's alpha goes on its first child (`007b5370` reads the new
/// line's child list; the template's first child is `ListItemText`).
fn line_alpha(ui: &mut Ui, line: TileId, dim: bool) {
    let text = ui.tiles[line]
        .children
        .first()
        .copied()
        .or_else(|| ui.find_below(line, "ListItemText"));
    if let Some(text) = text {
        ui.set_number(text, t::ALPHA, if dim { DIM } else { BRIGHT });
    }
}

impl RecipeMenu {
    /// A menu whose filter starts at `filter` (what it was when the menu
    /// last closed; -1 the first time).
    pub fn new(menu: TileId, filter: i32) -> RecipeMenu {
        RecipeMenu {
            menu,
            tiles: [None; 14],
            recipes: ListBox::default(),
            parts: ListBox::default(),
            lines: Vec::new(),
            filters: Vec::new(),
            filter,
            shown: None,
            requests: Vec::new(),
            closed: false,
        }
    }

    fn tile(&self, id: usize) -> Option<TileId> {
        self.tiles.get(id).copied().flatten()
    }

    fn set_text(&self, ui: &mut Ui, id: usize, text: &str) {
        if let Some(tile) = self.tile(id) {
            ui.set_text(tile, t::STRING, text);
        }
    }

    fn set(&self, ui: &mut Ui, id: usize, trait_id: i32, value: f32) {
        if let Some(tile) = self.tile(id) {
            ui.set_number(tile, trait_id, value);
        }
    }

    /// Opens it (`00726ff0`) with the lines (in the order to show them) and
    /// the filter's subcategories; false without its lists.
    pub fn open(&mut self, ui: &mut Ui, lines: Vec<Line>, filters: Vec<(u32, String)>) -> bool {
        let (Some(recipes), Some(parts)) = (self.tile(RECIPES), self.tile(INGREDIENTS)) else {
            return false;
        };
        self.recipes = ListBox::new(ui, recipes, TEMPLATE);
        self.parts = ListBox::new(ui, parts, TEMPLATE);
        for (id, name) in [
            (TITLE, "sRecipes"),
            (SKILL_TITLE, "sSkillRequirement"),
            (INGREDIENTS_TITLE, "sIngredients"),
            (MADE_AT_TITLE, "sMadeAt"),
            (ACCEPT, "sAccept"),
            (EXIT, "sExit"),
        ] {
            let s = setting(ui, name);
            self.set_text(ui, id, &s);
        }
        for id in [ACCEPT, EXIT] {
            if let Some(tile) = self.tile(id) {
                let x = ui.number(tile, t::X);
                ui.set_number(tile, t::X, x - 20.0);
            }
        }
        self.lines = lines;
        self.filters = filters;
        if self.filter >= self.filters.len() as i32 {
            self.filter = -1;
        }
        for (i, line) in self.lines.iter().enumerate() {
            if let Some(tile) = self.recipes.add(ui, self.menu, i as i32, Some(&line.text)) {
                line_alpha(ui, tile, !line.makeable);
            }
        }
        self.apply_filter(ui);
        ui.set_number(self.menu, menu::LEAVE_STACK, 0.0);
        ui.set_number(self.menu, t::VISIBLE, 1.0);
        ui.refresh();
        true
    }

    /// The title and the filtered lines for the filter's index
    /// (`007274b0` cases 0 to 2, `00728d60`).
    fn apply_filter(&mut self, ui: &mut Ui) {
        let title = match usize::try_from(self.filter)
            .ok()
            .and_then(|i| self.filters.get(i))
        {
            Some((_, name)) => name.clone(),
            None => setting(ui, "sRecipes"),
        };
        self.set_text(ui, TITLE, &title);
        let only = usize::try_from(self.filter)
            .ok()
            .and_then(|i| self.filters.get(i))
            .map(|(id, _)| *id);
        let lines = self.lines.clone();
        self.recipes.filter(ui, &|value| {
            only.is_some_and(|sub| {
                lines
                    .get(value as usize)
                    .is_some_and(|l| l.subcategory != sub)
            })
        });
        ui.refresh();
    }

    fn step_filter(&mut self, ui: &mut Ui, up: bool) {
        let n = self.filters.len() as i32;
        self.filter += if up { 1 } else { -1 };
        if self.filter < -1 {
            self.filter = n - 1;
        } else if self.filter >= n {
            self.filter = -1;
        }
        self.apply_filter(ui);
    }

    /// Shows the details of the line under the pointer (`00727b10`).
    pub fn show_details(&mut self, ui: &mut Ui, line: usize, d: &Details) {
        self.shown = Some(line);
        self.set(ui, CARD, t::VISIBLE, 0.0);
        self.set(ui, ICON, t::VISIBLE, 0.0);
        self.set_text(ui, MADE_AT, &d.made_at);
        self.set(
            ui,
            MADE_AT,
            t::ALPHA,
            if d.made_at_dim { DIM } else { BRIGHT },
        );
        let heading = d
            .heading
            .clone()
            .unwrap_or_else(|| setting(ui, "sIngredients"));
        self.set_text(ui, INGREDIENTS_TITLE, &heading);
        self.parts.clear(ui);
        for (i, (text, dim)) in d.parts.iter().enumerate() {
            if let Some(tile) = self.parts.add(ui, self.menu, i as i32, Some(text)) {
                line_alpha(ui, tile, *dim);
            }
        }
        self.set_text(ui, SKILL, &d.skill);
        self.set(ui, SKILL, t::ALPHA, if d.skill_dim { DIM } else { BRIGHT });
        self.set(ui, ACCEPT, t::TARGET, if d.can_make { 1.0 } else { 0.0 });
        if let (Some(card), Some(tile)) = (&d.card, self.tile(CARD)) {
            Self::show_card(ui, tile, card);
            ui.set_number(tile, t::VISIBLE, 1.0);
        }
        if let (Some(icon), Some(tile)) = (&d.icon, self.tile(ICON)) {
            ui.set_string(tile, t::FILENAME, icon);
            ui.set_number(tile, t::VISIBLE, 1.0);
        }
        ui.refresh();
    }

    /// Fills the item card (`00728da0` with the item and the card tile):
    /// the cards' values by the item's kind, their titles (`sInventory…`
    /// settings, the first mod card `sModEffects`), each card shown by its
    /// bit of the mask (DR 0x1, DPS 0x2, weight 0x4, value 0x8, condition
    /// 0x10, ammunition 0x20, effects 0x40, the mods 0x80 .. 0x200, damage
    /// 0x800; the strength card 0x400, made opaque, when the file has
    /// one), and the effects and mod cards' y: half the card's height less
    /// 20, twice that when the condition or ammunition card shows.
    pub fn show_card(ui: &mut Ui, tile: TileId, c: &Card) {
        let children: Vec<TileId> = ui.tiles[tile].children.iter().copied().take(12).collect();
        let at = |i: usize| children.get(i).copied();
        let title = ui.names.lookup_or_add("_Title").unwrap_or(0);
        let value = ui.names.lookup_or_add("_Value").unwrap_or(0);
        let set_value = |ui: &mut Ui, i: usize, text: &str| {
            if let Some(t) = at(i) {
                ui.set_string(t, value, text);
            }
        };
        let mut mask = match &c.kind {
            CardKind::Armour { resistance } => {
                set_value(ui, card::DR, &format!("{resistance}"));
                0x1d
            }
            CardKind::Weapon {
                dps,
                damage,
                projectiles,
                strength,
                ammo,
            } => {
                let dps = dps.map_or(String::new(), |d| {
                    format!("{}", crate::pipboy::items::round_half_up(d))
                });
                set_value(ui, card::DPS, &dps);
                let dam = crate::pipboy::items::damage_text(*damage, *projectiles);
                set_value(ui, card::DAMAGE, &dam);
                // The condition meter reads the card's `user5`: full.
                if let Some(t) = at(card::CONDITION) {
                    ui.set_number(t, t::USER0 + 5, 1.0);
                }
                set_value(ui, card::STRENGTH, &format!("{strength}"));
                if let Some(t) = at(card::AMMO) {
                    ui.set_string(t, title, ammo);
                }
                0xc3e
            }
            CardKind::Other => 0xc,
        };
        if c.effects.is_empty() {
            mask &= !0x40;
        } else {
            set_value(ui, card::EFFECTS, &c.effects);
            mask |= 0x40;
        }
        set_value(ui, card::VALUE, &amount_text(c.value));
        set_value(ui, card::WEIGHT, &amount_text(c.weight));
        for (i, name) in [
            (card::DR, "sInventoryDamageResistance"),
            (card::DPS, "sInventoryDamagePerSecond"),
            (card::DAMAGE, "sInventoryDamage"),
            (card::WEIGHT, "sInventoryWeightUpper"),
            (card::VALUE, "sInventoryValue"),
            (card::CONDITION, "sInventoryCondition"),
            (card::EFFECTS, "sInventoryEffects"),
            (card::MOD_ONE, "sModEffects"),
            (card::STRENGTH, "sInventoryStrReq"),
        ] {
            if let Some(t) = at(i) {
                let s = setting(ui, name);
                ui.set_string(t, title, &s);
            }
        }
        for i in 0..12 {
            let Some(t) = at(i) else { continue };
            if i == card::STRENGTH {
                continue;
            }
            let bit = if i == card::DAMAGE { 0x800 } else { 1 << i };
            ui.set_number(t, t::VISIBLE, if mask & bit != 0 { 1.0 } else { 0.0 });
        }
        if let Some(t) = at(card::STRENGTH) {
            ui.set_number(t, t::VISIBLE, if mask & 0x400 != 0 { 1.0 } else { 0.0 });
            ui.set_number(t, t::ALPHA, 255.0);
        }
        let mut y = ui.number(tile, t::HEIGHT) / 2.0 - 20.0;
        if mask & 0x30 != 0 {
            y += y;
        }
        for i in [card::EFFECTS, card::MOD_ONE, card::MOD_TWO, card::MOD_THREE] {
            if let Some(t) = at(i) {
                ui.set_number(t, t::Y, y);
            }
        }
        ui.refresh();
    }

    /// Exit, and the end of making (`00727430`).
    pub fn close(&mut self, ui: &mut Ui) {
        if self.closed {
            return;
        }
        ui.set_number(self.menu, menu::LEAVE_STACK, 1.0);
        self.closed = true;
    }
}

impl MenuCode for RecipeMenu {
    fn class(&self) -> i32 {
        CLASS
    }

    fn set_tile(&mut self, id: i32, tile: TileId) {
        if (0..14).contains(&id) {
            self.tiles[id as usize] = Some(tile);
        }
    }

    /// The recipes and the ingredients: the pointer chooses lines in them
    /// (`00717e70`).
    fn lists(&mut self) -> Vec<&mut ListBox> {
        vec![&mut self.recipes, &mut self.parts]
    }

    /// `007274b0`.
    fn click(&mut self, ui: &mut Ui, id: i32, _tile: Option<TileId>, _now: f64) {
        match id {
            0 => self.step_filter(ui, false),
            1 | 2 => self.step_filter(ui, true),
            7 => {
                let usable = self
                    .tile(ACCEPT)
                    .is_some_and(|tile| ui.number(tile, t::TARGET) != 0.0);
                if let Some(line) = self.shown.filter(|_| usable) {
                    self.requests.push(Request::Make(line));
                }
            }
            8 => {
                self.requests.push(Request::Close);
                self.close(ui);
            }
            _ => {}
        }
    }

    /// `00727b10` with a recipe line.
    fn mouseover(&mut self, ui: &mut Ui, id: i32, tile: TileId) {
        if id != LINE_ID {
            return;
        }
        if let Some(value) = self
            .recipes
            .items
            .iter()
            .find(|i| i.tile == tile)
            .map(|i| i.value)
        {
            let _ = ui;
            self.requests.push(Request::Show(value as usize));
        }
    }

    /// `00728420`: Left and Right step the filter.
    fn special_key(&mut self, ui: &mut Ui, code: i32, now: f64) -> bool {
        match code {
            menu::special::RIGHT => {
                self.click(ui, RIGHT_ARROW as i32, None, now);
                true
            }
            menu::special::LEFT => {
                self.click(ui, LEFT_ARROW as i32, None, now);
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menus::test_support;

    fn line(text: &str, makeable: bool, subcategory: u32) -> Line {
        Line {
            text: text.into(),
            makeable,
            subcategory,
        }
    }

    fn opened(filter: i32) -> (Ui, RecipeMenu) {
        let mut ui = test_support::ui();
        let mut m = RecipeMenu::new(0, filter);
        m.menu = test_support::load(&mut ui, &test_support::recipe_menu(), &mut m);
        let lines = vec![
            line("Healing Powder", true, 0x19),
            line("Rounds (10)", true, 0x2A),
            line("Stimpak", false, 0x19),
        ];
        let filters = vec![(0x19, "Aid".to_string()), (0x2A, "Ammo".to_string())];
        assert!(m.open(&mut ui, lines, filters));
        (ui, m)
    }

    fn shown(ui: &mut Ui, m: &RecipeMenu) -> Vec<(String, f32)> {
        m.recipes
            .shown_items(ui, 0, i32::MAX)
            .into_iter()
            .map(|l| {
                (
                    ui.string(l.tile, t::STRING).unwrap_or_default(),
                    ui.number(ui.tiles[l.tile].children[0], t::ALPHA),
                )
            })
            .collect()
    }

    /// `00726ff0`, `00727680`, `007274b0`: the settings, the lines with
    /// their alphas, and the filter stepping through -1.
    #[test]
    fn opening_and_the_filter() {
        let (mut ui, mut m) = opened(-1);
        let s = |ui: &mut Ui, m: &RecipeMenu, id: usize| {
            ui.string(m.tiles[id].unwrap(), t::STRING)
                .unwrap_or_default()
        };
        assert_eq!(s(&mut ui, &m, TITLE), "RECIPES");
        assert_eq!(s(&mut ui, &m, ACCEPT), "Accept");
        assert_eq!(s(&mut ui, &m, EXIT), "Exit");
        assert_eq!(
            shown(&mut ui, &m),
            [
                ("Healing Powder".to_string(), 255.0),
                ("Rounds (10)".to_string(), 255.0),
                ("Stimpak".to_string(), 127.0)
            ]
        );
        // Up: Aid, then Ammo, then all again.
        m.click(&mut ui, 2, None, 0.0);
        assert_eq!(s(&mut ui, &m, TITLE), "Aid");
        assert_eq!(shown(&mut ui, &m).len(), 2);
        m.click(&mut ui, 1, None, 0.0);
        assert_eq!(s(&mut ui, &m, TITLE), "Ammo");
        assert_eq!(shown(&mut ui, &m)[0].0, "Rounds (10)");
        m.click(&mut ui, 2, None, 0.0);
        assert_eq!(m.filter, -1);
        assert_eq!(shown(&mut ui, &m).len(), 3);
        // Down from all: the last subcategory.
        m.click(&mut ui, 0, None, 0.0);
        assert_eq!(s(&mut ui, &m, TITLE), "Ammo");
        // Kept: a new menu starts where it was.
        let (mut ui, m) = opened(1);
        assert_eq!(
            ui.string(m.tiles[TITLE].unwrap(), t::STRING).as_deref(),
            Some("Ammo")
        );
    }

    /// `00727b10`, `007274b0` cases 7 and 8: details, Accept only when it
    /// can be made, Exit closes.
    #[test]
    fn details_accept_and_exit() {
        let (mut ui, mut m) = opened(-1);
        let first = m.recipes.items[0].tile;
        m.mouseover(&mut ui, LINE_ID, first);
        assert_eq!(m.requests, [Request::Show(0)]);
        let d = Details {
            made_at: "Campfire".into(),
            made_at_dim: false,
            heading: None,
            parts: vec![
                ("Broc Flower (1/1)".into(), false),
                ("Xander Root (0/1)".into(), true),
            ],
            skill: String::new(),
            skill_dim: false,
            can_make: false,
            icon: Some("Interface\\Icons\\Items\\healingpowder.dds".into()),
            card: None,
        };
        m.show_details(&mut ui, 0, &d);
        let parts: Vec<(String, f32)> = m
            .parts
            .shown_items(&mut ui, 0, i32::MAX)
            .into_iter()
            .map(|l| {
                (
                    ui.string(l.tile, t::STRING).unwrap_or_default(),
                    ui.number(ui.tiles[l.tile].children[0], t::ALPHA),
                )
            })
            .collect();
        assert_eq!(
            parts,
            [
                ("Broc Flower (1/1)".to_string(), 255.0),
                ("Xander Root (0/1)".to_string(), 127.0)
            ]
        );
        assert_eq!(ui.number(m.tiles[ICON].unwrap(), t::VISIBLE), 1.0);
        // Accept does nothing while it can't be made.
        m.requests.clear();
        m.click(&mut ui, 7, None, 0.0);
        assert!(m.requests.is_empty());
        m.show_details(
            &mut ui,
            0,
            &Details {
                can_make: true,
                ..d
            },
        );
        m.click(&mut ui, 7, None, 0.0);
        assert_eq!(m.requests, [Request::Make(0)]);
        m.click(&mut ui, 8, None, 0.0);
        assert!(m.closed);
        assert_eq!(ui.number(m.menu, menu::LEAVE_STACK), 1.0);
    }

    // The Dead Money contributor's menu tests, adapted to this menu: the
    // details come from the caller ([`Details`]), the filter's "all" is -1,
    // and making closes the menu (`007284f0`), so the list is not refreshed
    // in place.

    fn details(can_make: bool, skill: &str) -> Details {
        Details {
            made_at: "Workbench".into(),
            made_at_dim: false,
            heading: None,
            parts: vec![("Scrap (2/2)".into(), false), ("Water (0/1)".into(), true)],
            skill: skill.into(),
            skill_dim: !skill.is_empty(),
            can_make,
            icon: None,
            card: None,
        }
    }

    fn card_texts(ui: &mut Ui, m: &RecipeMenu) -> Vec<(bool, String, String)> {
        let card = m.tiles[CARD].unwrap();
        let (title, value) = (
            ui.names.lookup("_Title").unwrap(),
            ui.names.lookup("_Value").unwrap(),
        );
        ui.tiles[card]
            .children
            .clone()
            .into_iter()
            .map(|c| {
                (
                    ui.number(c, t::VISIBLE) != 0.0,
                    ui.string(c, title).unwrap_or_default(),
                    ui.string(c, value).unwrap_or_default(),
                )
            })
            .collect()
    }

    /// `00727b10` → `00728da0`: the first product's card, by its kind: a
    /// weapon's DPS, weight, value, condition (full), ammunition, strength
    /// and damage (0xc3e); armour's DR, weight, value and condition (0x1d);
    /// anything else weight and value (0xc), "%.1f" below 1, "--" for
    /// none. The damage threshold card (the file's 13th) is left alone.
    #[test]
    fn the_item_card_shows_the_first_product() {
        let (mut ui, mut m) = opened(-1);
        let weapon = Card {
            kind: CardKind::Weapon {
                dps: None,
                damage: 6.5,
                projectiles: 1,
                strength: 2,
                ammo: "9mm (0/40)".into(),
            },
            value: 25.0,
            weight: 2.0,
            effects: String::new(),
        };
        m.show_details(
            &mut ui,
            0,
            &Details {
                card: Some(weapon),
                ..details(true, "")
            },
        );
        let card = m.tiles[CARD].unwrap();
        assert_eq!(ui.number(card, t::VISIBLE), 1.0);
        let shown: Vec<(bool, String, String)> = card_texts(&mut ui, &m);
        let visible: Vec<bool> = shown.iter().map(|s| s.0).collect();
        assert_eq!(
            visible,
            [false, true, true, true, true, true, false, false, false, false, true, true, false]
        );
        assert_eq!(shown[1].1, "DPS");
        assert_eq!(shown[2].2, "2");
        assert_eq!(shown[3].2, "25");
        assert_eq!(shown[5].1, "9mm (0/40)");
        assert_eq!(shown[10].2, "2");
        assert_eq!(shown[11], (true, "DAM".to_string(), "7".to_string()));
        let condition = ui.tiles[card].children[4];
        assert_eq!(ui.number(condition, t::USER0 + 5), 1.0);
        let strength = ui.tiles[card].children[10];
        assert_eq!(ui.number(strength, t::ALPHA), 255.0);
        // The effects card's y: (120 / 2 - 20) × 2 with the condition shown.
        let effects = ui.tiles[card].children[6];
        assert_eq!(ui.number(effects, t::Y), 80.0);

        let armour = Card {
            kind: CardKind::Armour { resistance: 12 },
            value: 0.5,
            weight: -1.0,
            effects: String::new(),
        };
        m.show_details(
            &mut ui,
            0,
            &Details {
                card: Some(armour),
                ..details(true, "")
            },
        );
        let shown = card_texts(&mut ui, &m);
        let visible: Vec<bool> = shown.iter().map(|s| s.0).collect();
        assert_eq!(
            visible,
            [
                true, false, true, true, true, false, false, false, false, false, false, false,
                false
            ]
        );
        assert_eq!(shown[0].2, "12");
        assert_eq!(shown[2].2, "--");
        assert_eq!(shown[3].2, "0.5");

        let other = Card {
            kind: CardKind::Other,
            value: 19.0,
            weight: 0.3,
            effects: "+20 HP".into(),
        };
        m.show_details(
            &mut ui,
            0,
            &Details {
                card: Some(other),
                ..details(true, "")
            },
        );
        let shown = card_texts(&mut ui, &m);
        let visible: Vec<bool> = shown.iter().map(|s| s.0).collect();
        assert_eq!(
            visible,
            [
                false, false, true, true, false, false, true, false, false, false, false, false,
                false
            ]
        );
        assert_eq!(shown[2].2, "0.3");
        assert_eq!(shown[6].2, "+20 HP");
        assert_eq!(ui.number(ui.tiles[card].children[6], t::Y), 40.0);
        // No product: the card stays hidden.
        m.show_details(&mut ui, 0, &details(true, ""));
        assert_eq!(ui.number(card, t::VISIBLE), 0.0);
    }

    fn accept_on(ui: &mut Ui, m: &RecipeMenu) -> f32 {
        ui.number(m.tiles[ACCEPT].unwrap(), t::TARGET)
    }

    #[test]
    fn opening_lists_every_recipe_with_nothing_chosen() {
        let (mut ui, m) = opened(-1);
        assert_eq!(m.recipes.items.len(), 3);
        assert_eq!(m.shown, None);
        assert_eq!(
            ui.string(m.tiles[TITLE].unwrap(), t::STRING).as_deref(),
            Some("RECIPES")
        );
        // A recipe that can't be made yet is dimmed.
        assert_eq!(shown(&mut ui, &m)[2].1, DIM);
        // Accept is off.
        assert_eq!(accept_on(&mut ui, &m), 0.0);
    }

    #[test]
    fn choosing_a_row_shows_its_details_and_turns_accept_on() {
        let (mut ui, mut m) = opened(-1);
        let row = m.recipes.items[1].tile;
        m.mouseover(&mut ui, LINE_ID, row);
        assert_eq!(m.requests, [Request::Show(1)]);
        m.show_details(&mut ui, 1, &details(true, ""));
        assert_eq!(m.shown, Some(1));
        assert_eq!(
            ui.string(m.tiles[MADE_AT].unwrap(), t::STRING).as_deref(),
            Some("Workbench")
        );
        assert_eq!(m.parts.items.len(), 2);
        // The ingredient the player lacks is dimmed.
        let water = m.parts.items[1].tile;
        assert_eq!(ui.string(water, t::STRING).as_deref(), Some("Water (0/1)"));
        assert_eq!(ui.number(ui.tiles[water].children[0], t::ALPHA), DIM);
        assert_eq!(accept_on(&mut ui, &m), 1.0);
        // One that can't be made: its skill shows and Accept goes off.
        m.show_details(&mut ui, 2, &details(false, "Science (39/40)"));
        assert_eq!(
            ui.string(m.tiles[SKILL].unwrap(), t::STRING).as_deref(),
            Some("Science (39/40)")
        );
        assert_eq!(ui.number(m.tiles[SKILL].unwrap(), t::ALPHA), DIM);
        assert_eq!(accept_on(&mut ui, &m), 0.0);
    }

    #[test]
    fn accept_asks_to_make_only_a_makeable_choice() {
        let (mut ui, mut m) = opened(-1);
        m.click(&mut ui, 7, None, 0.0);
        assert!(m.requests.is_empty());
        m.show_details(&mut ui, 2, &details(false, "Science (39/40)"));
        m.click(&mut ui, 7, None, 0.0);
        assert!(m.requests.is_empty());
        m.show_details(&mut ui, 0, &details(true, ""));
        m.click(&mut ui, 7, None, 0.0);
        assert_eq!(m.requests, [Request::Make(0)]);
        assert!(!m.closed);
    }

    #[test]
    fn the_arrows_go_through_the_sub_categories() {
        let (mut ui, mut m) = opened(-1);
        m.click(&mut ui, 2, None, 0.0);
        assert_eq!(m.filter, 0);
        assert_eq!(shown(&mut ui, &m).len(), 2);
        assert!(m.special_key(&mut ui, menu::special::RIGHT, 0.0));
        assert_eq!(m.filter, 1);
        assert_eq!(shown(&mut ui, &m).len(), 1);
        assert!(m.special_key(&mut ui, menu::special::LEFT, 0.0));
        assert_eq!(m.filter, 0);
        // Round to all, and back from all to the last.
        m.click(&mut ui, 2, None, 0.0);
        m.click(&mut ui, 2, None, 0.0);
        assert_eq!((m.filter, shown(&mut ui, &m).len()), (-1, 3));
        m.click(&mut ui, 0, None, 0.0);
        assert_eq!(m.filter, 1);
    }

    #[test]
    fn after_making_the_menu_closes() {
        let (mut ui, mut m) = opened(-1);
        m.show_details(&mut ui, 0, &details(true, ""));
        m.click(&mut ui, 7, None, 0.0);
        assert_eq!(m.requests, [Request::Make(0)]);
        // The making's end (`00727430`).
        m.close(&mut ui);
        assert!(m.closed);
        assert_eq!(ui.number(m.menu, menu::LEAVE_STACK), 1.0);
    }

    #[test]
    fn exit_closes() {
        let (mut ui, mut m) = opened(-1);
        m.click(&mut ui, 8, None, 0.0);
        assert!(m.closed);
        assert_eq!(m.requests, [Request::Close]);
    }
}
