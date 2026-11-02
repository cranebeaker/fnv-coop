//! Tag skills (`ui::menus::chargen`): a script's `SetTagSkills`
//! (`menus::Menu::Character(CharacterMenu::TagSkills)`, Doc Mitchell's
//! intro after the psych exam). Done puts the picked skills in the player's
//! tag slots (`world::chargen::set_tag_skills`).

use cellview::Game;
use esm::FormId;
use ui::menus::chargen::{CharGenMenu, Request, Skill, FILE};
use world::chargen::{self, CharacterMenu};
use world::dialogue::PLAYER_REF;
use world::scripting::{Facts, GameState};

use super::{OpenMenu, Screen};
use crate::menus::Menu;

/// Whether this module shows a request.
pub fn takes(menu: &Menu) -> bool {
    matches!(menu, Menu::Character(CharacterMenu::TagSkills { .. }))
}

/// Opens the tag skills menu (`SetTagSkills`: not "only add").
pub fn open(screen: &mut Screen, game: &Game, state: &mut GameState, request: Menu) {
    let Menu::Character(CharacterMenu::TagSkills { count, preselect }) = request else {
        return;
    };
    let order = &game.order;
    let mut menu = CharGenMenu::new(0);
    let tile = match screen.load(game, FILE, &mut menu) {
        Ok(t) => t,
        Err(e) => {
            println!("The tag skills menu can't be shown: {e}");
            return;
        }
    };
    menu.menu = tile;
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    // The skills 32 to 45 but Big Guns (the exe's actor value flag 0x2000).
    let skills: Vec<Skill> = chargen::SHOWN_SKILLS
        .iter()
        .map(|&av| {
            let (icon, description) = chargen::actor_value_icon_and_text(order, av);
            Skill {
                av,
                name: chargen::actor_value_name(order, av),
                base: facts.permanent_actor_value(PLAYER_REF, av).unwrap_or(0.0) as i32,
                tagged: state.tag_skills.contains(&av),
                icon,
                description,
            }
        })
        .collect();
    let bonus = world::scripting::game_setting(order, "fAVDTagSkillBonus").unwrap_or(10.0);
    if !menu.open(
        &mut screen.ui,
        count as i32,
        false,
        preselect,
        bonus,
        skills,
    ) {
        println!(
            "MENUS: Char Gen Menu Creation Failed... Are your menu and art resources up to date?"
        );
        screen.ui.detach(tile);
        return;
    }
    println!("Tag skills menu: {count} to tag.");
    screen.open.push(OpenMenu::CharGen(Box::new(menu)));
}

/// What the menu asked for, carried out. Returns sounds to play.
pub fn after(screen: &mut Screen, game: &Game, state: &mut GameState) -> Vec<FormId> {
    let order = &game.order;
    let mut sounds = Vec::new();
    for m in screen.open.iter_mut() {
        let OpenMenu::CharGen(menu) = m else {
            continue;
        };
        for name in menu.sounds.drain(..) {
            if let Some(id) = order.form_by_editor_id(&name) {
                sounds.push(id);
            }
        }
        for request in std::mem::take(&mut menu.requests) {
            let Request::Close { tags } = request;
            chargen::set_tag_skills(state, &tags);
            let names: Vec<String> = tags
                .iter()
                .map(|&av| chargen::actor_value_name(order, av))
                .collect();
            println!("Tag skills: {}", names.join(", "));
        }
    }
    sounds
}
