//! Crafting (`ui::menus::recipe`): a script's `ShowRecipeMenu` on the player
//! (`menus::Menu::Recipe`). What is listed, how many can be made and the
//! making are `world::crafting`'s; "How many?" is the quantity menu over it,
//! and making closes the menu (`007284f0` ends with `00727430`), its notices
//! going to the HUD.

use std::sync::atomic::{AtomicI32, Ordering};

use cellview::Game;
use esm::{FormId, FourCC, LoadOrder};
use ui::menus::recipe::{Details, Line, RecipeMenu, Request, FILE};
use world::crafting::{self, Recipe};
use world::scripting::{Facts, GameState};

use super::{OpenMenu, Screen};
use crate::menus::Menu;

/// The filter's index (`0119fbf4`, a global that starts at -1 and keeps
/// its value from one opening to the next).
static FILTER: AtomicI32 = AtomicI32::new(-1);

/// The crafting menu on screen, who crafts and the category, and the
/// recipe of each line.
pub struct RecipeScreen {
    pub menu: RecipeMenu,
    pub actor: FormId,
    pub category: FormId,
    pub recipes: Vec<Recipe>,
    /// The line Accept was used on, waiting for "How many?".
    making: Option<usize>,
}

/// Whether this module shows a request.
pub fn takes(menu: &Menu) -> bool {
    matches!(menu, Menu::Recipe { .. })
}

fn name_of(order: &LoadOrder, id: FormId) -> String {
    order
        .get(id)
        .and_then(|r| r.record().ok())
        .and_then(|r| r.full_name())
        .unwrap_or_default()
}

fn icon_of(order: &LoadOrder, id: FormId) -> Option<String> {
    order
        .get(id)
        .and_then(|r| r.record().ok())
        .and_then(|r| r.get(FourCC::new(b"ICON")).map(|s| s.zstring()))
        .filter(|s| !s.is_empty())
}

/// Opens the crafting menu for a request.
pub fn open(screen: &mut Screen, game: &Game, state: &mut GameState, request: Menu) {
    let Menu::Recipe { actor, category } = request else {
        return;
    };
    let order = &game.order;
    state.stock(order, actor);
    let listing = crafting::listing(order, state, actor, category, None);
    let all = crafting::recipes(order);
    let recipes: Vec<Recipe> = listing
        .lines
        .iter()
        .filter_map(|l| all.iter().find(|r| r.form_id == l.recipe).cloned())
        .collect();
    let lines: Vec<Line> = listing
        .lines
        .iter()
        .zip(&recipes)
        .map(|(l, r)| Line {
            text: l.text.clone(),
            makeable: l.makeable > 0,
            subcategory: r.subcategory.map_or(0, |s| s.0),
        })
        .collect();
    let filters: Vec<(u32, String)> = listing
        .subcategories
        .iter()
        .map(|s| (s.0, crafting::category_name(order, *s)))
        .collect();
    let mut menu = RecipeMenu::new(0, FILTER.load(Ordering::Relaxed));
    let tile = match screen.load(game, FILE, &mut menu) {
        Ok(t) => t,
        Err(e) => {
            println!("The crafting menu can't be shown: {e}");
            return;
        }
    };
    menu.menu = tile;
    if !menu.open(&mut screen.ui, lines, filters) {
        println!(
            "MENUS: Recipe Menu Creation Failed... Are your menu and art resources up to date?"
        );
        screen.ui.detach(tile);
        return;
    }
    println!(
        "Crafting menu: {} ({} recipes, {} can be made).",
        crafting::category_name(order, category),
        recipes.len(),
        listing.makeable
    );
    screen.open.push(OpenMenu::Recipe(Box::new(RecipeScreen {
        menu,
        actor,
        category,
        recipes,
        making: None,
    })));
    // The crafting tutorial, once (`00726ff0`: the message by editor ID,
    // marked as id 0x26; the menu doesn't wait on it).
    super::tutorial::show_once(
        screen,
        game,
        state,
        world::tutorial::id::RECIPE,
        "HelpCrafting",
    );
}

/// The right side for a recipe (`00727b10` with a recipe line).
fn details(
    order: &LoadOrder,
    state: &GameState,
    actor: FormId,
    category: FormId,
    r: &Recipe,
) -> Details {
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    let made_at = match r.category {
        Some(c) => crafting::category_name(order, c),
        None => "*** MISSING CATEGORY ***".to_string(),
    };
    // One ingredient and several products (a breakdown): the products are
    // listed, under the subcategory's name.
    let breakdown = r.outputs.len() >= 2 && r.ingredients.len() == 1;
    let mut parts: Vec<(String, String, bool)> = if breakdown {
        r.outputs
            .iter()
            .map(|c| {
                let name = name_of(order, c.item);
                (name.clone(), format!("{name} ({})", c.count), false)
            })
            .collect()
    } else {
        r.ingredients
            .iter()
            .map(|c| {
                let name = name_of(order, c.item);
                let have = state.item_count(order, actor, c.item).max(0) as u32;
                (
                    name.clone(),
                    format!("{name} ({have}/{})", c.count),
                    have < c.count,
                )
            })
            .collect()
    };
    // `00728cd0`: by the items' names (`_mbscmp`).
    parts.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
    let heading = breakdown.then(|| {
        r.subcategory
            .map(|s| crafting::category_name(order, s))
            .unwrap_or_default()
    });
    let (skill, skill_dim) = match r.skill {
        Some(av) => {
            let v = facts.current_actor_value(actor, av).unwrap_or(0.0);
            let v = if (32..=45).contains(&av) {
                v.clamp(0.0, 100.0)
            } else {
                v
            }
            .floor() as i32;
            (
                format!(
                    "{} ({v}/{})",
                    world::chargen::actor_value_name(order, av),
                    r.skill_level
                ),
                v < r.skill_level,
            )
        }
        None => (String::new(), false),
    };
    Details {
        made_at,
        made_at_dim: r.category != Some(category),
        heading,
        parts: parts.into_iter().map(|(_, t, d)| (t, d)).collect(),
        skill,
        skill_dim,
        can_make: crafting::can_make(order, state, actor, r, category) > 0,
        icon: r.outputs.first().and_then(|o| icon_of(order, o.item)),
        // The first product's item card (`00727b10` → `00728da0`).
        card: r
            .outputs
            .first()
            .map(|o| ui::pipboy::gather::recipe_card(order, state, o.item)),
    }
}

/// What the menu asked for, carried out. Returns the HUD notices of what
/// was made.
pub fn after(screen: &mut Screen, game: &Game, state: &mut GameState) -> Vec<String> {
    let order = &game.order;
    let mut notices = Vec::new();
    // The quantity menu's answer goes to the crafting menu below it.
    let mut answer = None;
    let mut below = false;
    for m in screen.open.iter_mut() {
        match m {
            OpenMenu::Recipe(_) => below = true,
            OpenMenu::Quantity(q) if below => {
                if let Some(n) = q.answer.take() {
                    answer = Some(n);
                }
            }
            _ => {}
        }
    }
    let mut ask = None;
    for m in screen.open.iter_mut() {
        let OpenMenu::Recipe(r) = m else {
            continue;
        };
        if let (Some(n), Some(line)) = (answer, r.making.take()) {
            if let Some(recipe) = r.recipes.get(line) {
                if let Some(made) = crafting::make(order, state, r.actor, recipe, n.max(0) as u32) {
                    println!("Crafted {} x{n}.", recipe.name);
                    notices.extend(made.notices);
                }
            }
            // `007284f0` does nothing for 0: the menu stays.
            if n > 0 {
                FILTER.store(r.menu.filter, Ordering::Relaxed);
                r.menu.close(&mut screen.ui);
            }
        }
        for request in std::mem::take(&mut r.menu.requests) {
            match request {
                Request::Show(line) => {
                    if let Some(recipe) = r.recipes.get(line) {
                        let d = details(order, state, r.actor, r.category, recipe);
                        r.menu.show_details(&mut screen.ui, line, &d);
                    }
                }
                Request::Make(line) => {
                    if let Some(recipe) = r.recipes.get(line) {
                        let most = crafting::can_make(order, state, r.actor, recipe, r.category);
                        if most > 0 {
                            r.making = Some(line);
                            ask = Some(most as i32);
                        }
                    }
                }
                Request::Close => {
                    FILTER.store(r.menu.filter, Ordering::Relaxed);
                }
            }
        }
    }
    if let Some(most) = ask {
        super::container::open_quantity(screen, game, most);
    }
    notices
}
