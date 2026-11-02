//! Crafting: recipes (`RCPE`) in categories (`RCCT`), as the game's recipe
//! menu (`ShowRecipeMenu`, class `RecipeMenu`) works them out. Read from
//! `FalloutNV.exe` 1.4.0.525; names and the record layout also in the Xbox
//! 360 prototype's symbols (Xbox PDB).
//!
//! The record (`TESRecipe::Load`, `005a8110`): `EDID`, `OBND`, `FULL`,
//! `CTDA` conditions, `DATA` (`RECIPE_DATA` (Xbox PDB): `ReqSkillId` i32, −1
//! for none, `ReqSkillLevel` i32, `CategoryID` and `SubCategoryID` form IDs),
//! then ingredients (`RCIL` item, `RCQY` count) and outputs (`RCOD` item,
//! `RCQY` count). `RCIL` and `RCOD` start a component; after every
//! subrecord the current component is added to its list if it has an item
//! and a count other than 0, so a component without `RCQY` is never added.
//! In memory (PC offsets; the Xbox PDB's `TESRecipe` layout less 16 bytes
//! of form header): data at +0x24, conditions +0x34, ingredients +0x3c,
//! outputs +0x44, category +0x54, subcategory +0x58.
//!
//! What the menu lists (`00727680`), how many can be made
//! (`RecipeMenu::CanMakeRecipe` (Xbox PDB), `00727920`), the order
//! (`00728c10`) and what making does (`007284f0`, the quantity menu's
//! callback) are below, each with its function.

use std::cmp::Ordering;

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_u32;
use crate::dialogue::{read_condition, Condition};
use crate::scripting::{Facts, GameState};

const RCPE: FourCC = FourCC::new(b"RCPE");
const RCCT: FourCC = FourCC::new(b"RCCT");
const FULL: FourCC = FourCC::new(b"FULL");
const DATA: FourCC = FourCC::new(b"DATA");
const CTDA: FourCC = FourCC::new(b"CTDA");
const RCIL: FourCC = FourCC::new(b"RCIL");
const RCOD: FourCC = FourCC::new(b"RCOD");
const RCQY: FourCC = FourCC::new(b"RCQY");

/// The recipe menu's number (`RecipeMenu` 1077, as `MenuMode` takes it).
pub const RECIPE_MENU: u16 = 1077;

/// The most of one recipe `CanMakeRecipe` counts (the global at `011d8ea8` starts at
/// 100).
pub const MOST: u32 = 100;

/// The miscellaneous statistic crafting adds to (`007284f0` calls
/// `004d5e10(0x20, n)`): "Items Crafted".
pub const ITEMS_CRAFTED: u8 = 0x20;

/// The settings the "added" notice is made of (`007284f0`):
/// `sAddItemtoInventory` (`011d4e7c`) and `sPlural` (`011d3114`), with the
/// values the exe gives them where no plugin sets them (their static
/// initializers `00f6d140`: "added", `00f6d0e0`: "(s)"; the official master
/// sets neither).
const ADDED: (&str, &str) = ("sAddItemtoInventory", "added");
const PLURAL: (&str, &str) = ("sPlural", "(s)");

fn text_setting(order: &LoadOrder, (name, default): (&str, &str)) -> String {
    crate::scripting::game_setting_text(order, name).unwrap_or_else(|| default.to_string())
}

/// The notice's icon (`007284f0`).
pub const NOTICE_ICON: &str = "Interface\\Icons\\Message Icons\\glow_message_giftbox.dds";

/// An ingredient or a product: an item and how many.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Component {
    pub item: FormId,
    pub count: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Recipe {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    /// `FULL`; empty when there is none (the menu shows the empty string,
    /// `00408da0`).
    pub name: String,
    /// The skill it needs (an actor value), if any.
    pub skill: Option<u16>,
    pub skill_level: i32,
    pub category: Option<FormId>,
    pub subcategory: Option<FormId>,
    pub conditions: Vec<Condition>,
    pub ingredients: Vec<Component>,
    pub outputs: Vec<Component>,
}

impl Recipe {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<Recipe> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == RCPE)?;
        if rr.entry.header.is_deleted() {
            return None;
        }
        let record = rr.record().ok()?;
        let form = |raw: u32| Some(rr.plugin.to_global(FormId(raw))).filter(|f| f.0 != 0);
        let mut recipe = Recipe {
            form_id: id,
            editor_id: record.editor_id(),
            name: String::new(),
            skill: None,
            skill_level: 0,
            category: None,
            subcategory: None,
            conditions: Vec::new(),
            ingredients: Vec::new(),
            outputs: Vec::new(),
        };
        // The component being read: (is an output, item, count).
        let mut current: Option<(bool, Option<FormId>, u32)> = None;
        for sub in &record.subrecords {
            let d = &sub.data[..];
            match sub.kind {
                k if k == FULL => recipe.name = sub.zstring(),
                k if k == CTDA => {
                    if let Some(c) = read_condition(&rr, d) {
                        recipe.conditions.push(c);
                    }
                }
                k if k == DATA && d.len() >= 16 => {
                    let skill = le_u32(d, 0) as i32;
                    recipe.skill = (skill >= 0).then_some(skill as u16);
                    recipe.skill_level = le_u32(d, 4) as i32;
                    recipe.category = form(le_u32(d, 8));
                    recipe.subcategory = form(le_u32(d, 12));
                }
                k if (k == RCIL || k == RCOD) && d.len() >= 4 => {
                    current = Some((k == RCOD, form(le_u32(d, 0)), 0));
                }
                k if k == RCQY && d.len() >= 4 => {
                    if let Some(c) = current.as_mut() {
                        c.2 = le_u32(d, 0);
                    }
                }
                _ => {}
            }
            // Added after every subrecord once it has an item and a count.
            if let Some((output, Some(item), count)) = current {
                if count != 0 {
                    let list = if output {
                        &mut recipe.outputs
                    } else {
                        &mut recipe.ingredients
                    };
                    list.push(Component { item, count });
                    // The game keeps the same component current, so a
                    // stray subrecord before the next `RCIL`/`RCOD` would
                    // add it again; that is kept.
                }
            }
        }
        Some(recipe)
    }
}

/// Every recipe (winning, not deleted), in load order.
pub fn recipes(order: &LoadOrder) -> Vec<Recipe> {
    order
        .records_of_type(RCPE)
        .filter_map(|r| Recipe::load(order, r.form_id))
        .collect()
}

/// A recipe category's `FULL` name (`RCCT`), as the menu's filter shows
/// it.
pub fn category_name(order: &LoadOrder, id: FormId) -> String {
    order
        .get(id)
        .filter(|r| r.entry.header.kind == RCCT)
        .and_then(|r| r.record().ok())
        .and_then(|r| r.full_name())
        .unwrap_or_default()
}

/// The actor's skill as the recipe checks read it (`0066ef20`): its value
/// now (the actor value owner's +0xc, the same that `GetActorValue`
/// reads), kept within 0 to 100 for a skill (`0066f190`), then floored.
fn skill_value(facts: &Facts, actor: FormId, skill: u16) -> i32 {
    let v = facts.current_actor_value(actor, skill).unwrap_or(0.0);
    let v = if (32..=45).contains(&skill) {
        v.clamp(0.0, 100.0)
    } else {
        v
    };
    v.floor() as i32
}

/// Whether the actor's skill meets the recipe's (no skill: always).
pub fn has_skill(order: &LoadOrder, state: &GameState, actor: FormId, recipe: &Recipe) -> bool {
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    recipe.skill.map_or(true, |s| {
        skill_value(&facts, actor, s) >= recipe.skill_level
    })
}

/// How many times `actor` can make `recipe` from the menu of `category`
/// (`CanMakeRecipe`, `00727920`): 0 without a category and a subcategory,
/// with too little skill, or when the recipe is in another category;
/// otherwise the fewest times any ingredient's count goes into what the
/// actor has, at most [`MOST`], and 0 if any is short.
pub fn can_make(
    order: &LoadOrder,
    state: &GameState,
    actor: FormId,
    recipe: &Recipe,
    category: FormId,
) -> u32 {
    if recipe.category.is_none() || recipe.subcategory.is_none() {
        return 0;
    }
    if !has_skill(order, state, actor, recipe) {
        return 0;
    }
    if recipe.category != Some(category) {
        return 0;
    }
    let mut most = MOST;
    for c in &recipe.ingredients {
        let have = state.item_count(order, actor, c.item).max(0) as u32;
        if have < c.count {
            return 0;
        }
        most = most.min(have / c.count);
    }
    most
}

/// One line of the recipe list.
#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub recipe: FormId,
    /// The recipe's name (what the order compares).
    pub name: String,
    /// The recipe's name, then " (n)" when its first output comes `n` > 1
    /// at a time (`"%s (%d)"`).
    pub text: String,
    /// How many can be made ([`can_make`]); 0 draws the line dim (alpha
    /// 0x7f instead of 0xff).
    pub makeable: u32,
}

/// What the menu shows for a category (`00727680`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Listing {
    /// Craftable first, then by name (`00728c10`).
    pub lines: Vec<Line>,
    /// The subcategories of the listed recipes, each once, by name
    /// (`007278c0`), for the filter.
    pub subcategories: Vec<FormId>,
    /// How many listed recipes can be made at least once.
    pub makeable: usize,
}

/// The recipes `actor` sees in the menu of `category`: those of that
/// category whose conditions pass on the actor (`00680c30` on the recipe's
/// conditions, the actor as the subject). `subcategory` narrows the lines
/// (the filter's choice; `None` is "all"); the filter's own list always
/// holds every subcategory of the category's listed recipes.
pub fn listing(
    order: &LoadOrder,
    state: &GameState,
    actor: FormId,
    category: FormId,
    subcategory: Option<FormId>,
) -> Listing {
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    let mut out = Listing::default();
    let mut subs: Vec<(String, FormId)> = Vec::new();
    for recipe in recipes(order) {
        if recipe.category != Some(category) {
            continue;
        }
        if !facts.conditions_pass(&recipe.conditions, actor, actor) {
            continue;
        }
        if let Some(sub) = recipe.subcategory {
            if !subs.iter().any(|(_, s)| *s == sub) {
                subs.push((category_name(order, sub), sub));
            }
        }
        if subcategory.is_some() && recipe.subcategory != subcategory {
            continue;
        }
        let text = match recipe.outputs.first() {
            Some(o) if o.count >= 2 => format!("{} ({})", recipe.name, o.count),
            _ => recipe.name.clone(),
        };
        let makeable = can_make(order, state, actor, &recipe, category);
        if makeable > 0 {
            out.makeable += 1;
        }
        out.lines.push(Line {
            recipe: recipe.form_id,
            name: recipe.name.clone(),
            text,
            makeable,
        });
    }
    // Byte order, as `_mbscmp` compares (`00469880`).
    subs.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
    out.subcategories = subs.into_iter().map(|(_, s)| s).collect();
    out.lines.sort_by(order_lines);
    out
}

/// `00728c10`: a line that can be made comes before one that can't;
/// otherwise by name (`_mbscmp`).
fn order_lines(a: &Line, b: &Line) -> Ordering {
    let (ma, mb) = (a.makeable > 0, b.makeable > 0);
    if ma != mb {
        return if ma {
            Ordering::Less
        } else {
            Ordering::Greater
        };
    }
    a.name.as_bytes().cmp(b.name.as_bytes())
}

/// What making a recipe gave, for the notices.
#[derive(Debug, Clone, PartialEq)]
pub struct Made {
    /// One notice per output: "name added", or "n names added"
    /// (`"%s %s"`, `"%i %s%s %s"` with `sAddItemtoInventory` and `sPlural`),
    /// shown with [`NOTICE_ICON`].
    pub notices: Vec<String>,
}

/// Makes `recipe` `times` times for `actor` (`007284f0`, given the count
/// chosen in the quantity menu):
///
/// - each ingredient: `count × times` taken (the actor's remove-item, its
///   vtable +0x17c); a weapon the actor holds just one of and has drawn is
///   put away first (`0088d7d0`);
/// - "Items Crafted" goes up by `times`, unless the first ingredient is
///   casino chips (form type 0x6C);
/// - each output: `count × times` given. Weapons other than grenades,
///   mines and thrown ones (animation types 10 to 13, `004c0bf0`), and
///   armour and clothing (form types 0x18, 0x1A), come at 80 % condition
///   (`00482090(0.8)`); weapons' condition is kept per holder and weapon
///   here, so it is set only when the actor had none of that weapon.
///
/// Returns nothing when `times` is 0. Whether it can be made is the
/// caller's question ([`can_make`]): the game's menu only offers what can.
pub fn make(
    order: &LoadOrder,
    state: &mut GameState,
    actor: FormId,
    recipe: &Recipe,
    times: u32,
) -> Option<Made> {
    if times == 0 {
        return None;
    }
    state.stock(order, actor);
    for c in &recipe.ingredients {
        let n = (c.count * times) as i32;
        if is_kind(order, c.item, b"WEAP")
            && state.item_count(order, actor, c.item) == 1
            && state.is_equipped(actor, c.item)
        {
            state.unequip(actor, c.item);
        }
        let have = state.items.entry((actor, c.item)).or_insert(0);
        *have = (*have - n).max(0);
    }
    let chips_first = recipe
        .ingredients
        .first()
        .is_some_and(|c| is_kind(order, c.item, b"CHIP"));
    if !chips_first {
        crate::stats::bump(state, ITEMS_CRAFTED, i64::from(times));
    }
    let added = text_setting(order, ADDED);
    let plural = text_setting(order, PLURAL);
    let mut notices = Vec::new();
    for c in &recipe.outputs {
        let n = (c.count * times) as i32;
        let had = state.item_count(order, actor, c.item);
        *state.items.entry((actor, c.item)).or_insert(0) += n;
        if is_kind(order, c.item, b"WEAP") && !is_thrown(order, c.item) && had == 0 {
            state.weapon_health.insert((actor, c.item), 0.8);
        }
        let name = order
            .get(c.item)
            .and_then(|r| r.record().ok())
            .and_then(|r| r.full_name())
            .unwrap_or_default();
        notices.push(if n < 2 {
            format!("{name} {added}")
        } else {
            format!("{n} {name}{plural} {added}")
        });
    }
    Some(Made { notices })
}

fn is_kind(order: &LoadOrder, id: FormId, kind: &[u8; 4]) -> bool {
    order
        .get(id)
        .is_some_and(|r| r.entry.header.kind == FourCC::new(kind))
}

/// Grenades, mines and thrown weapons: animation types 10 to 13
/// (`004c0bf0`: types 3 to 13 but not 3 to 9, the weapon's byte at +0xf4,
/// the first byte of `DNAM`).
fn is_thrown(order: &LoadOrder, id: FormId) -> bool {
    order
        .get(id)
        .and_then(|r| r.record().ok())
        .and_then(|r| r.get(FourCC::new(b"DNAM")).map(|s| s.data.first().copied()))
        .flatten()
        .is_some_and(|t| (10..=13).contains(&t))
}
