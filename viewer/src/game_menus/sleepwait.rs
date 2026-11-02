//! Waiting and sleeping (`ui::menus::sleepwait`): T (the Rest control), a
//! bed used, or a script's `ShowSleepWaitMenu` (`menus::Menu::SleepWait`)
//! opens the game's sleep/wait menu; the hours it accepts pass in the
//! world one a real second (`world::living::sleep`), with the screen faded
//! to black for a sleep (the fader manager's fader 0, drawn under the
//! menus by `game_menus`).

use cellview::Game;
use esm::FormId;
use ui::menus::sleepwait::{Frame, Request, SleepWaitMenu, TimeLine, FILE};
use world::living::sleep::{self, Clock, Fade};
use world::scripting::GameState;

use super::{message, OpenMenu, Screen};
use crate::menus::Menu;

/// Whether this module shows a request.
pub fn takes(menu: &Menu) -> bool {
    matches!(menu, Menu::SleepWait { .. })
}

/// The time line's clock from the state's globals (`007c0000`).
fn time_line(order: &esm::LoadOrder, state: &GameState) -> TimeLine {
    let clock = Clock::now(order, state);
    let (hour, minute, pm) = clock.twelve_hour();
    TimeLine {
        weekday: clock.weekday(),
        date: clock.date(),
        hour,
        minute,
        pm,
    }
}

/// An INI flag (`bSaveOnRest`, `bSaveOnWait`), if the INIs set it.
fn ini_flag(game: &Game, key: &str) -> Option<bool> {
    game.settings
        .get("GamePlay", key)
        .and_then(|v| v.trim().parse::<i32>().ok())
        .map(|v| v != 0)
}

/// Opens the menu (`007bfc30`) in sleep or wait mode. The refusals
/// (`world::living::sleep::may_wait`, `may_sleep_in`) are the caller's.
pub fn open(screen: &mut Screen, game: &Game, state: &GameState, request: Menu) -> Vec<FormId> {
    let Menu::SleepWait { sleep: sleeping } = request else {
        return Vec::new();
    };
    let order = &game.order;
    let mut menu = SleepWaitMenu::new(0);
    let tile = match screen.load(game, FILE, &mut menu) {
        Ok(t) => t,
        Err(e) => {
            println!("The sleep/wait menu can't be shown: {e}");
            return Vec::new();
        }
    };
    menu.menu = tile;
    menu.autosave = sleep::autosaves(
        sleeping,
        ini_flag(game, "bSaveOnRest"),
        ini_flag(game, "bSaveOnWait"),
    );
    let alpha = message::background_alpha(screen);
    let clock = time_line(order, state);
    if !menu.open(&mut screen.ui, sleeping, &clock, alpha) {
        println!(
            "MENUS: Sleep Wait Menu Creation Failed... Are your menu and art resources up to date?"
        );
        screen.ui.detach(tile);
        return Vec::new();
    }
    println!(
        "{} menu: {}",
        if sleeping { "Sleep" } else { "Wait" },
        screen
            .ui
            .string(menu.tiles[3].unwrap_or(tile), ui::names::t::STRING)
            .unwrap_or_default()
    );
    let sounds = menu
        .sounds
        .drain(..)
        .filter_map(|s| order.form_by_editor_id(&s))
        .collect();
    screen.open.push(OpenMenu::SleepWait(Box::new(menu)));
    sounds
}

/// For `--open-menu wait:N` / `sleep:N` (screenshots of the hours
/// passing): the bar is put at N − 1 and Wait pressed through the menu's
/// own click.
pub fn choose_and_wait(screen: &mut Screen, hours: u32) {
    use ui::menu::MenuCode;
    let Screen { ui, open, .. } = screen;
    let Some(OpenMenu::SleepWait(menu)) = open.last_mut() else {
        return;
    };
    if let (Some(bar), Some(current)) = (menu.tiles[1], ui.names.lookup("_current_value")) {
        ui.set_base(bar, current, hours.saturating_sub(1) as f32);
        ui.refresh();
    }
    menu.update(ui);
    menu.click(ui, 4, None, 0.0);
}

/// Every frame (`007c03a0`, `007c0580`): what the menu's clicks asked for
/// is carried out first (the game's click handler does it at once), then
/// the hours' text follows the bar, and while waiting an hour passes in
/// the world each real second; `rest_pressed` / `rest_held` are the Rest
/// control (T). Returns sounds to play.
pub fn frame(
    screen: &mut Screen,
    game: &Game,
    state: &mut GameState,
    dt: f32,
    rest_pressed: bool,
    rest_held: bool,
) -> Vec<FormId> {
    let mut sounds = after(screen, game, state);
    let order = &game.order;
    let box_showing = screen.message().is_some();
    {
        let Screen { ui, open, .. } = screen;
        for m in open.iter_mut() {
            let OpenMenu::SleepWait(menu) = m else {
                continue;
            };
            menu.update(ui);
            let hours_left = state.living.hours_left;
            match menu.frame(ui, dt, rest_pressed, rest_held, hours_left, box_showing) {
                Frame::HourPasses => {
                    sleep::pass_hour(order, state);
                    let clock = time_line(order, state);
                    menu.hour_passed(ui, state.living.hours_left, &clock);
                }
                Frame::Nothing | Frame::Cancelled => {}
            }
        }
    }
    sounds.extend(after(screen, game, state));
    sounds
}

/// What the menu asked for, carried out. Returns sounds to play.
fn after(screen: &mut Screen, game: &Game, state: &mut GameState) -> Vec<FormId> {
    let order = &game.order;
    let mut sounds = Vec::new();
    let Screen { open, fade, .. } = screen;
    for m in open.iter_mut() {
        let OpenMenu::SleepWait(menu) = m else {
            continue;
        };
        for name in menu.sounds.drain(..) {
            if let Some(id) = order.form_by_editor_id(&name) {
                sounds.push(id);
            }
        }
        for request in std::mem::take(&mut menu.requests) {
            match request {
                Request::Begin { hours, sleeping } => {
                    println!(
                        "{} {hours} hour{}.",
                        if sleeping { "Sleeping" } else { "Waiting" },
                        if hours == 1 { "" } else { "s" }
                    );
                    sleep::begin(state, hours, sleeping);
                }
                // The game's save manager is asked for an autosave
                // (`00850a40`): the viewer's own, as a script's `Autosave`.
                Request::Autosave => {
                    use world::more_functions::{SaveKind, Shown};
                    state.events.push(world::scripting::Event::More(Shown::Save(
                        SaveKind::Autosave,
                    )));
                }
                Request::FadeToBlack => *fade = Some(Fade::start(sleep::fade_seconds(order))),
                Request::Cancel => sleep::cancel(state),
                Request::FadeBack => {
                    if let Some(f) = fade.as_mut() {
                        f.release();
                    }
                }
            }
        }
    }
    sounds
}
