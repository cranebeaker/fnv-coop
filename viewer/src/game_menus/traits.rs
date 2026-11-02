//! Traits (`ui::menus::traits`): a script's `ShowTraitMenu`
//! (`menus::Menu::Character(CharacterMenu::Traits)`, Doc Mitchell's intro).
//! The traits picked become the player's perks when the menu closes.

use cellview::Game;
use esm::FormId;
use ui::menus::traits::{Request, TraitMenu, FILE};
use world::chargen::CharacterMenu;
use world::scripting::GameState;

use super::{OpenMenu, Screen};
use crate::menus::Menu;

/// Whether this module shows a request.
pub fn takes(menu: &Menu) -> bool {
    matches!(menu, Menu::Character(CharacterMenu::Traits { .. }))
}

/// Opens the trait menu.
pub fn open(screen: &mut Screen, game: &Game, state: &mut GameState, request: Menu) {
    let Menu::Character(CharacterMenu::Traits { max }) = request else {
        return;
    };
    let mut menu = TraitMenu::new(0);
    let tile = match screen.load(game, FILE, &mut menu) {
        Ok(t) => t,
        Err(e) => {
            println!("The trait menu can't be shown: {e}");
            return;
        }
    };
    menu.menu = tile;
    let choices = world::perks::trait_choices(&game.order, state);
    let traits = super::levelup::menu_perks(screen, game, state, choices);
    if !menu.open(&mut screen.ui, max as i32, traits) {
        println!(
            "MENUS: Trait Menu Creation Failed... Are your menu and art resources up to date?"
        );
        screen.ui.detach(tile);
        return;
    }
    println!("Trait menu: up to {max}, {} listed.", menu.traits.len());
    screen.open.push(OpenMenu::Traits(Box::new(menu)));
}

/// What the menu asked for, carried out. Returns sounds to play.
pub fn after(screen: &mut Screen, game: &Game, state: &mut GameState) -> Vec<FormId> {
    let order = &game.order;
    let mut sounds = Vec::new();
    for m in screen.open.iter_mut() {
        let OpenMenu::Traits(menu) = m else {
            continue;
        };
        for name in menu.sounds.drain(..) {
            if let Some(id) = order.form_by_editor_id(&name) {
                sounds.push(id);
            }
        }
        for request in std::mem::take(&mut menu.requests) {
            let Request::Close { traits } = request;
            // Traits are perks the player has (`007e76e0`: each one's next
            // rank), abilities and all.
            for &t in &traits {
                world::perks::add(order, state, FormId(t));
            }
            let names: Vec<String> = traits
                .iter()
                .map(|&t| {
                    order
                        .get(FormId(t))
                        .and_then(|r| r.record().ok())
                        .and_then(|r| r.full_name())
                        .unwrap_or_else(|| FormId(t).to_string())
                })
                .collect();
            println!(
                "Traits: {}",
                if names.is_empty() {
                    "none".to_string()
                } else {
                    names.join(", ")
                }
            );
        }
    }
    sounds
}
