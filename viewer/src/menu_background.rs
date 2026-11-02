//! The world behind the menus (`world::menu_background`): each frame the
//! menus up are read into the game's tests, the background is captured
//! once as they open, with its modifier, and let go as they close. The
//! modifier's values go to the final passes with the scripts' (`effects`);
//! the picture is held there (`grade`).

use bevy::prelude::*;
use world::menu_background::{menu, Background, Change, MenuState, STATIC_BACKGROUND_DEFAULT};

use crate::game_menus::{GameMenus, MenuDraw};
use crate::menus::Menus;

#[derive(Resource, Default)]
pub struct MenuBackground {
    pub state: Background,
    /// Counts the captures (each one is drawn once).
    pub generation: u32,
    /// `bStaticMenuBackground:Display`, read once (as the game reads it at
    /// start, `0086d590`).
    setting: Option<bool>,
}

/// The tests' view of the menus this frame.
pub fn menu_state(
    setting: bool,
    menus: &Menus,
    game_menus: &GameMenus,
    classes: &[i32],
    vats: bool,
) -> MenuState {
    let (pause_menu, main_menu, name_entry) = crate::game_menus::background_facts(game_menus);
    let mut shown: Vec<u16> = classes
        .iter()
        .filter_map(|&c| u16::try_from(c).ok())
        .collect();
    // The viewer's own lock and V.A.T.S. menus, on top when up.
    if menus.lockpicking {
        shown.push(menu::LOCKPICK);
    }
    if vats {
        shown.push(menu::VATS);
    }
    MenuState {
        setting,
        // The game's menus here, the Pip-Boy, the lock and V.A.T.S. (the
        // viewer's own older menus, `menus`' queue, aren't the game's).
        menu_mode: !shown.is_empty() || menus.pipboy,
        top: shown.last().copied(),
        shown,
        pause_menu,
        main_menu,
        pipboy: menus.pipboy,
        name_entry,
        held: false,
        fader_1: false,
    }
}

pub fn update(
    game: Res<crate::GameFiles>,
    menus: Res<Menus>,
    game_menus: Res<GameMenus>,
    draw: Res<MenuDraw>,
    vats: Option<Res<crate::vats::Vats>>,
    mut background: ResMut<MenuBackground>,
    mut effects: ResMut<crate::effects::Effects>,
) {
    let setting = *background.setting.get_or_insert_with(|| {
        game.0
            .settings
            .get("Display", "bStaticMenuBackground")
            .and_then(|v| v.trim().parse::<i32>().ok())
            .map_or(STATIC_BACKGROUND_DEFAULT, |v| v != 0)
    });
    let state = menu_state(
        setting,
        &menus,
        &game_menus,
        &draw.1,
        vats.is_some_and(|v| v.is_on()),
    );
    let bg = background.bypass_change_detection();
    match bg.state.update(&state) {
        Change::Captured => {
            bg.generation += 1;
            println!(
                "Menu background captured with {:?} (top menu {:?}).",
                bg.state.modifier, state.top
            );
        }
        Change::Released => println!("Menu background let go."),
        Change::None => {}
    }
    let held = bg
        .state
        .captured
        .then_some((bg.state.modifier, bg.generation));
    if effects.background != held {
        effects.background = held;
    }
}
