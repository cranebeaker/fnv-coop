//! Levelling up (`ui::menus::levelup`): a level gained
//! (`world::experience::level_up`, `menus::Menu::LevelUp`). The skill
//! points go into the player's skills as they're clicked, as the game
//! does; the perk picked is given when the menu closes.

use cellview::Game;
use esm::{FormId, LoadOrder};
use ui::menus::levelup::{LevelUpMenu, Perk, Request, Skill, FILE};
use world::chargen;
use world::dialogue::PLAYER_REF;
use world::scripting::{Facts, GameState};

use super::{OpenMenu, Screen};
use crate::menus::Menu;

/// Whether this module shows a request.
pub fn takes(menu: &Menu) -> bool {
    matches!(menu, Menu::LevelUp(..))
}

/// The skills the menu lists (`00785540`): the actor values 32 to 45 but
/// those the exe's actor value table flags 0x2000 (only Big Guns, 33:
/// `0066f260`), each with the player's permanent value (whole).
fn skills(order: &LoadOrder, state: &GameState) -> Vec<Skill> {
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    chargen::SHOWN_SKILLS
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
        .collect()
}

/// The perks it offers (`world::perks::level_up_choices`), with their
/// pictures and requirement texts; only those that can be picked now when
/// the INI's `[Interface] bHideUnavailablePerks` is set (`00784c80`).
fn perks(screen: &Screen, game: &Game, state: &GameState) -> Vec<Perk> {
    let choices = world::perks::level_up_choices(&game.order, state);
    menu_perks(screen, game, state, choices)
}

/// Perks as the level-up and trait menus show them: with their pictures
/// and requirement texts (`005ebac0`); only those that can be picked now
/// when the INI's `[Interface] bHideUnavailablePerks` is set (`00784c80`,
/// `007e6990`).
pub(super) fn menu_perks(
    screen: &Screen,
    game: &Game,
    state: &GameState,
    choices: Vec<world::perks::Choice>,
) -> Vec<Perk> {
    let order = &game.order;
    let hide = game
        .settings
        .get("Interface", "bHideUnavailablePerks")
        .and_then(|v| v.trim().parse::<i32>().ok())
        .is_some_and(|v| v != 0);
    let setting = |n: &str| screen.ui.setting_text(n).unwrap_or_default();
    choices
        .into_iter()
        .filter(|c| !hide || c.available)
        .map(|c| Perk {
            form: c.perk.0,
            text: world::perks::requirements_text(order, state, c.perk, &setting),
            icon: world::perks::icon(order, c.perk),
            name: c.name,
            rank: c.rank,
            ranks: c.ranks,
            min_level: c.min_level,
            available: c.available,
        })
        .collect()
}

/// Opens the level-up menu for a level gained.
pub fn open(screen: &mut Screen, game: &Game, state: &mut GameState, request: Menu) -> Vec<FormId> {
    let Menu::LevelUp(up) = request else {
        return Vec::new();
    };
    let order = &game.order;
    let mut menu = LevelUpMenu::new(0);
    let tile = match screen.load(game, FILE, &mut menu) {
        Ok(t) => t,
        Err(e) => {
            println!("The level-up menu can't be shown: {e}");
            return Vec::new();
        }
    };
    menu.menu = tile;
    menu.tag_mult =
        world::scripting::game_setting(order, "iSkillPointsTagSkillMult").map_or(1, |v| v as i32);
    let skills = skills(order, state);
    let perks = if up.perk {
        perks(screen, game, state)
    } else {
        Vec::new()
    };
    if !menu.open(
        &mut screen.ui,
        up.level,
        up.skill_points as i32,
        skills,
        perks,
    ) {
        println!(
            "MENUS: Level Up Menu Creation Failed... Are your menu and art resources up to date?"
        );
        screen.ui.detach(tile);
        return Vec::new();
    }
    println!(
        "Level up menu: level {}, {} skill points{}.",
        up.level,
        menu.most[0],
        if menu.perks.is_empty() {
            ""
        } else {
            ", a perk"
        }
    );
    let sounds = menu
        .sounds
        .drain(..)
        .filter_map(|s| order.form_by_editor_id(&s))
        .collect();
    screen.open.push(OpenMenu::LevelUp(Box::new(menu)));
    sounds
}

/// For `--open-menu levelup:perks` (screenshots of the perk page): the
/// level's points are given through the menu's own clicks on the skills'
/// "more" arrows, top to bottom, then Continue is clicked.
pub fn give_points_and_continue(screen: &mut Screen, game: &Game, state: &mut GameState) {
    use ui::menu::MenuCode;
    use ui::menus::levelup::MORE_ID;
    use ui::names::t;
    loop {
        let Some(OpenMenu::LevelUp(menu)) = screen.open.last_mut() else {
            return;
        };
        let ui = &mut screen.ui;
        let arrow = menu.skill_list.items.iter().find_map(|line| {
            let children = ui.tiles[line.tile].children.clone();
            children.into_iter().find(|&c| {
                ui.has(c, t::ID)
                    && ui.number(c, t::ID) as i32 == MORE_ID
                    && ui.number(c, t::VISIBLE) != 0.0
            })
        });
        match arrow {
            Some(a) if menu.points[0] < menu.most[0] => menu.click(ui, MORE_ID, Some(a), 0.0),
            _ => break,
        }
    }
    let _ = after(screen, game, state);
    if let Some(OpenMenu::LevelUp(menu)) = screen.open.last_mut() {
        menu.click(&mut screen.ui, 7, None, 0.0);
    }
}

/// What the menu asked for, carried out. Returns sounds to play.
pub fn after(screen: &mut Screen, game: &Game, state: &mut GameState) -> Vec<FormId> {
    let order = &game.order;
    let mut sounds = Vec::new();
    for m in screen.open.iter_mut() {
        let OpenMenu::LevelUp(menu) = m else {
            continue;
        };
        for name in menu.sounds.drain(..) {
            if let Some(id) = order.form_by_editor_id(&name) {
                sounds.push(id);
            }
        }
        let requests: Vec<Request> = std::mem::take(&mut menu.requests);
        let mut changed = false;
        for request in requests {
            match request {
                // The skill's base value moves at once (`00785aa0`).
                Request::Skill { av, by } => {
                    let points = state.skill_points.entry(av).or_insert(0);
                    *points = points.saturating_add_signed(by);
                    changed = true;
                }
                Request::Close { perk } => {
                    if let Some(p) = perk.map(FormId) {
                        world::perks::add(order, state, p);
                        let name = order
                            .get(p)
                            .and_then(|r| r.record().ok())
                            .and_then(|r| r.full_name())
                            .unwrap_or_else(|| p.to_string());
                        println!("Perk: {name}");
                    }
                }
            }
        }
        // The perks' conditions are asked again when their page shows: the
        // points just given count.
        if changed && !menu.perks.is_empty() {
            let now = world::perks::level_up_choices(order, state);
            for p in menu.perks.iter_mut() {
                if let Some(c) = now.iter().find(|c| c.perk.0 == p.form) {
                    p.available = c.available;
                }
            }
        }
    }
    sounds
}
