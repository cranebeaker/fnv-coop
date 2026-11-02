//! The once-only tutorial messages (`world::tutorial` the manager,
//! `ui::menus::tutorial` the menu): every frame the manager's update runs
//! over the menus on screen (`007182e0`, from the interface manager's
//! update `0070c4a0`) and a message it picks opens in the tutorial menu
//! and is marked shown; Caravan and the crafting menu open theirs directly
//! ([`show`]), as does a script's `ShowTutorialMenu` ([`show_form`]); the
//! start menu's Help opens the help manual ([`open_manual`]).
//!
//! The manager is held while the start menu is up as the pause menu
//! ([`held`]); the name entry and "outside the game" (the main menu) don't
//! arise here. A menu's fade-in (`Menu` +0x24, `007024e0`) isn't kept:
//! menus here are shown at once, so the top menu counts as shown.

use cellview::Game;
use esm::FormId;
use ui::menus::tutorial::{Message, TutorialMenu, FILE};
use world::scripting::GameState;

use super::{OpenMenu, Screen};

/// The top menu, for the manager (`00702450`, `007024e0`).
struct Top(Option<i32>);

impl world::tutorial::Screen for Top {
    fn is_menu_open(&self, class: i32) -> bool {
        self.0 == Some(class)
    }
    fn top_menu_shown(&self) -> bool {
        self.0.is_some()
    }
}

/// A message record's title and text (`FULL`, `DESC`).
fn message(game: &Game, form: FormId) -> Option<Message> {
    let record = game.order.get(form)?.record().ok()?;
    let text = record
        .get(esm::FourCC::new(b"DESC"))
        .map(|s| s.zstring())
        .unwrap_or_default();
    Some(Message {
        form: form.0,
        title: record.full_name().unwrap_or_default(),
        text,
    })
}

/// Loads the tutorial menu's file (a tutorial menu already open closed
/// first, `007e8890`).
fn load(screen: &mut Screen, game: &Game, id: u8) -> Option<TutorialMenu> {
    for open in screen.open.iter_mut() {
        if let OpenMenu::Tutorial(t) = open {
            t.close(&mut screen.ui);
        }
    }
    let mut menu = TutorialMenu::new(0, id);
    match screen.load(game, FILE, &mut menu) {
        Ok(tile) => menu.menu = tile,
        Err(e) => {
            println!("The tutorial menu can't be opened: {e}");
            return None;
        }
    }
    Some(menu)
}

/// A script's `ShowTutorialMenu` (`005da630`): the menu with the message
/// it names (`TutorialMenu::Create(message, 0)`, not through the
/// manager); with none it doesn't stay open.
pub fn show_form(screen: &mut Screen, game: &Game, form: FormId) {
    let id = form
        .0
        .checked_sub(world::tutorial::message_form(0).0)
        .filter(|&i| i < world::tutorial::COUNT as u32)
        .map_or(world::tutorial::COUNT as u8, |i| i as u8);
    let Some(mut menu) = load(screen, game, id) else {
        return;
    };
    match message(game, form) {
        Some(m) => {
            menu.open(&mut screen.ui, &m);
            println!("Tutorial menu: {} ({form}).", m.title);
            screen.open.push(OpenMenu::Tutorial(Box::new(menu)));
        }
        None => {
            println!("Warning:  Unable to find valid starting message for Tutorial Menu.");
            screen.ui.detach(menu.menu);
        }
    }
}

/// The message the start menu's Help opens the manual on (`007d0770`):
/// going up the menus under the start menu, the first whose class a
/// tutorial word waits for (`007d09c0`) with that message in the manual.
/// (The game also looks at the Pip-Boy's page, class 1: the viewer's
/// start menu doesn't open over the Pip-Boy.)
pub fn manual_start(screen: &mut Screen, state: &GameState, manual: &[FormId]) -> Option<FormId> {
    for m in screen.open.iter_mut() {
        if let OpenMenu::Start(_) = m {
            break;
        }
        let id = state.tutorials.menu_message(m.code().class());
        if usize::from(id) >= world::tutorial::COUNT {
            continue;
        }
        let form = world::tutorial::message_form(id);
        if manual.contains(&form) {
            return Some(form);
        }
    }
    None
}

/// The start menu's Help (`007d0770`): the help manual (`HelpManual`)
/// in the tutorial menu, on the page [`manual_start`] picks or its
/// first.
pub fn open_manual(screen: &mut Screen, game: &Game, state: &GameState) {
    let order = &game.order;
    let forms = world::script_functions::form_list(order, state, world::tutorial::HELP_MANUAL);
    let start = manual_start(screen, state, &forms).and_then(|f| message(game, f));
    let pages: Vec<Message> = forms.iter().filter_map(|&f| message(game, f)).collect();
    let Some(mut menu) = load(screen, game, world::tutorial::COUNT as u8) else {
        return;
    };
    if menu.open_manual(&mut screen.ui, start, pages) {
        println!("Help manual: page {} of {}.", menu.page + 1, menu.pages);
        screen.open.push(OpenMenu::Tutorial(Box::new(menu)));
    } else {
        screen.ui.detach(menu.menu);
    }
}

/// Opens the tutorial menu with a message (`TutorialMenu::Create`,
/// `007e8890`): one already open is closed first. False when it can't be
/// (no record, no menu file), as the game's `Create` answers.
pub fn show(screen: &mut Screen, game: &Game, id: u8, form: FormId) -> bool {
    let Some(m) = message(game, form) else {
        return false;
    };
    let Some(mut menu) = load(screen, game, id) else {
        return false;
    };
    menu.open(&mut screen.ui, &m);
    println!("Tutorial: {} ({form}).", m.title);
    screen.open.push(OpenMenu::Tutorial(Box::new(menu)));
    true
}

/// Shows a message a menu opens itself if it hasn't been shown (Caravan's
/// `00741060` / `00741500`, the crafting menu's `00726ff0`): the record by
/// editor ID, marked shown once the menu has opened. True when the caller
/// is to wait for it (the menu's own flag, Caravan's `+0xe78`).
pub fn show_once(
    screen: &mut Screen,
    game: &Game,
    state: &mut GameState,
    id: u8,
    name: &str,
) -> bool {
    if state.tutorials.is_shown(id) {
        return false;
    }
    let Some(form) = game.order.form_by_editor_id(name) else {
        return false;
    };
    let shown = show(screen, game, id, form);
    if shown {
        state.tutorials.mark_shown(id);
    }
    shown
}

/// Whether the manager waits (`007182e0`'s first test, `004a4040`): the
/// start menu is up with its pause flag (`+0x1a8` bit 1, the pause menu
/// over the game).
pub fn held(open: &[OpenMenu]) -> bool {
    open.iter().any(|m| match m {
        OpenMenu::Start(s) => s.menu.flags & ui::menus::start::flag::PAUSE != 0,
        _ => false,
    })
}

/// The class of the menu on top: the game's menus here, else `other`,
/// the viewer's own menu that's up (the lockpicking menu, V.A.T.S.'s, the
/// Pip-Boy's: not among them in the viewer).
pub fn top_class(screen: &mut Screen, other: Option<i32>) -> Option<i32> {
    match screen.open.last_mut() {
        Some(m) => Some(m.code().class()),
        None => other,
    }
}

/// The manager's update (`007182e0`): a message it picks is shown and
/// marked (or the game's error line printed).
pub fn update(
    screen: &mut Screen,
    game: &Game,
    state: &mut GameState,
    now_ms: f64,
    other: Option<i32>,
) {
    let held = held(&screen.open);
    let top = Top(top_class(screen, other));
    let Some(id) = state.tutorials.update(now_ms as u32, true, held, &top) else {
        return;
    };
    let form = world::tutorial::message_form(id);
    if show(screen, game, id, form) && state.tutorials.mark_shown(id) {
        return;
    }
    println!("Error occurred while trying to display tutorial message");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui::menus::start::{flag, StartMenu};

    /// The start menu holds the manager only as the pause menu (its flag
    /// 1), as `004a4040` asks.
    #[test]
    fn the_pause_menu_holds_the_tutorials() {
        let ui = ui::Ui::new(
            ui::Screen {
                width_px: 1920,
                height_px: 1080,
                safe_x: 15.0,
                safe_y: 15.0,
            },
            ui::SystemColors::new(None, None),
            Box::new(|_| None),
        );
        let start = |flags: u32| {
            let mut menu = StartMenu::new(0, &ui);
            menu.flags = flags;
            OpenMenu::Start(Box::new(super::super::start::StartScreen::new(menu, false)))
        };
        assert!(!held(&[]));
        assert!(!held(&[start(0)]));
        assert!(held(&[start(flag::PAUSE)]));
        assert!(held(&[start(flag::PAUSE | flag::SAVE_MODE)]));
    }
}
