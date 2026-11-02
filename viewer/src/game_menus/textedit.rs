//! The player's name (`ui::menus::textedit`): a script's `ShowNameMenu`
//! or `GetPlayerName` (`menus::Menu::Character(CharacterMenu::Name)`, Doc
//! Mitchell's intro stage 15). OK sets the player's name (`007ab9a0`).

use cellview::Game;
use ui::menus::textedit::{TextEditMenu, FILE};
use world::chargen::{self, CharacterMenu};
use world::scripting::GameState;

use super::{OpenMenu, Screen};
use crate::menus::Menu;

/// Whether this module shows a request.
pub fn takes(menu: &Menu) -> bool {
    matches!(menu, Menu::Character(CharacterMenu::Name))
}

/// Opens the name menu (`007ab690`): `sEnterName`, the name so far.
pub fn open(screen: &mut Screen, game: &Game, state: &GameState, now_ms: f64) {
    let mut menu = TextEditMenu::new(0);
    let tile = match screen.load(game, FILE, &mut menu) {
        Ok(t) => t,
        Err(e) => {
            println!("The name menu can't be shown: {e}");
            return;
        }
    };
    menu.menu = tile;
    let prompt = screen.ui.setting_text("sEnterName").unwrap_or_default();
    let name = chargen::player_name(&game.order, state);
    if !menu.open(&mut screen.ui, &prompt, &name, now_ms) {
        screen.ui.detach(tile);
        return;
    }
    screen.open.push(OpenMenu::TextEdit(Box::new(menu)));
}

/// Every frame: the cursor's blinking (`007e65f0`).
pub fn update(screen: &mut Screen, now_ms: f64) {
    let Screen { ui, open, .. } = screen;
    for m in open.iter_mut() {
        if let OpenMenu::TextEdit(menu) = m {
            menu.update(ui, now_ms);
        }
    }
}

/// The name chosen, given to the player.
pub fn after(screen: &mut Screen, state: &mut GameState) {
    for m in screen.open.iter_mut() {
        let OpenMenu::TextEdit(menu) = m else {
            continue;
        };
        if let Some(name) = menu.answer.take() {
            println!("Name: {name}");
            state.player_name = Some(name);
        }
    }
}
