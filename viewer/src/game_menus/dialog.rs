//! The dialogue menu (`ui::menus::dialog`): the conversation (`dialogue`)
//! shown in the game's menu: the speaker's name, each response's text, then
//! the topics to choose from; the player's clicks and keys go back to the
//! conversation as "move on" or "this topic".

use cellview::Game;
use ui::menus::dialog::{Answer, DialogMenu, Topic, FILE};

use super::{OpenMenu, Screen};

/// The menu, if a conversation has it open.
pub fn menu(screen: &mut Screen) -> Option<&mut DialogMenu> {
    screen.open.iter_mut().find_map(|m| match m {
        OpenMenu::Dialog(d) => Some(d),
        _ => None,
    })
}

/// Opens the menu for a speaker (`00761a20`): the name, subtitles as
/// `[GamePlay] bDialogueSubtitles` says (on by default). The first line is
/// shown at once (the camera's zoom isn't drawn, so nothing waits for it).
pub fn open(screen: &mut Screen, game: &Game, name: &str) -> bool {
    if menu(screen).is_some() {
        return true;
    }
    let subtitles = game
        .settings
        .get("GamePlay", "bDialogueSubtitles")
        .is_none_or(|v| v.trim() != "0");
    let mut m = DialogMenu::new(0);
    match screen.load(game, FILE, &mut m) {
        Ok(tile) => {
            m.menu = tile;
            m.open(&mut screen.ui, name, subtitles, false);
            println!("Dialogue menu: talking to {name}.");
            screen.open.push(OpenMenu::Dialog(m));
            true
        }
        Err(e) => {
            println!("The dialogue menu can't be shown: {e}");
            false
        }
    }
}

/// A response's text.
pub fn show_line(screen: &mut Screen, text: &str) {
    let Screen { ui, open, .. } = screen;
    for m in open.iter_mut() {
        if let OpenMenu::Dialog(d) = m {
            d.show_line(ui, text);
        }
    }
}

/// The topics to choose from.
pub fn show_topics(screen: &mut Screen, topics: &[world::dialogue::Choice], now: f64) {
    let list: Vec<Topic> = topics
        .iter()
        .map(|c| Topic {
            text: c.label.clone(),
            dim: c.dim,
            failed: c.failed,
        })
        .collect();
    let Screen {
        ui,
        open,
        interface,
        ..
    } = screen;
    for m in open.iter_mut() {
        if let OpenMenu::Dialog(d) = m {
            d.show_topics(ui, &list, now);
            // The keyboard's choice starts afresh with each list.
            interface.focus = None;
            interface.over = None;
        }
    }
}

/// The conversation is over: the menu zooms out and closes.
pub fn end(screen: &mut Screen) {
    if let Some(d) = menu(screen) {
        d.end();
    }
}

/// Remove the dialogue menu after loading another saved world. This avoids
/// leaving its buffered answer or keyboard focus attached to the new state.
pub fn discard(screen: &mut Screen) -> bool {
    let Screen {
        ui,
        interface,
        open,
        ..
    } = screen;
    let before = open.len();
    let mut tiles = Vec::new();
    open.retain(|m| {
        if let OpenMenu::Dialog(dialog) = m {
            tiles.push(dialog.menu);
            false
        } else {
            true
        }
    });
    for tile in tiles {
        ui.detach(tile);
    }
    let discarded = open.len() != before;
    if discarded {
        interface.focus = None;
        interface.over = None;
    }
    discarded
}

/// A service menu (barter, recipes) has just been put over the menus
/// (`00763ff0`, from its `Create`): the conversation's menu, if one is
/// open and not ending, fades out and waits under it.
pub fn service_opened(screen: &mut Screen, dt: f32) {
    let Screen {
        ui, open, fades, ..
    } = screen;
    for m in open.iter_mut() {
        if let OpenMenu::Dialog(d) = m {
            if d.service_opened(ui, fades, dt) {
                println!("Dialogue menu: hidden under a service menu.");
            }
        }
    }
}

/// The service menu closed (`007640a0`): the conversation's menu fades
/// back in.
pub fn service_closed(screen: &mut Screen, dt: f32) {
    let Screen {
        ui, open, fades, ..
    } = screen;
    for m in open.iter_mut() {
        if let OpenMenu::Dialog(d) = m {
            if d.in_service {
                d.service_closed(ui, fades, dt);
                println!("Dialogue menu: back from the service menu.");
            }
        }
    }
}

/// Whether the conversation's menu is hidden under a service menu (nothing
/// can be chosen in it meanwhile).
pub fn hidden(screen: &mut Screen) -> bool {
    menu(screen).is_some_and(|d| d.hidden())
}

/// Whether a service menu cut the line being said short (once).
pub fn take_cut_line(screen: &mut Screen) -> bool {
    menu(screen).is_some_and(|d| std::mem::take(&mut d.cut_line))
}

/// What the player did since last asked.
pub fn take_answer(screen: &mut Screen) -> Option<Answer> {
    menu(screen).and_then(|d| d.answer.take())
}

/// Each frame: the zoom timing (`fDialogZoomInSeconds`,
/// `fDialogZoomOutSeconds` from the game's settings).
pub fn update(screen: &mut Screen, order: &esm::LoadOrder, dt: f32) {
    let setting =
        |name: &str, default: f32| world::scripting::game_setting(order, name).unwrap_or(default);
    let zoom_in = setting("fDialogZoomInSeconds", ui::menus::dialog::ZOOM_IN_SECONDS);
    let zoom_out = setting("fDialogZoomOutSeconds", ui::menus::dialog::ZOOM_OUT_SECONDS);
    let Screen { ui, open, .. } = screen;
    for m in open.iter_mut() {
        if let OpenMenu::Dialog(d) = m {
            if std::env::var_os("NVRS_MENU_DEBUG").is_some() {
                println!(
                    "dialog update dt {dt:.3} state {:?} zoom {:.3}",
                    d.state, d.zoom
                );
            }
            // (Its fade under a service menu and back is the interface's,
            // as every menu's: `Screen::fades`, `ui::fade`.)
            d.update(ui, dt, zoom_in, zoom_out);
        }
    }
}
