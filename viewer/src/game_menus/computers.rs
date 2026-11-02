//! A terminal's own screen (`ui::menus::computers`) for a terminal the
//! player gets into (`world::terminal`): its screens from the records,
//! the items the player picks run as the game runs them (`00757f70`):
//! the item's result script on the placed terminal (`00501830`), its note
//! given when flag 0x01 says so and the player hasn't it ("Note Added:
//! name", `sComputersAddedNote`, `00966a70`), the note shown, a sub-menu
//! gone into, flag 0x02 filling the list again with its conditions asked
//! anew. A terminal that isn't unlocked (hacked, or opened with its
//! password note) logs on first, with the hacking game's word length in
//! asterisks.

use esm::FormId;
use ui::menus::computers::{
    sound, ComputersMenu, NoteView, Rates, Request, Screen, Setup, Used, FILE,
};
use world::scripting::{GameState, Runner};
use world::terminal::{self, note_kind, Terminal};

use super::hacking::HackingSounds;
use super::{OpenMenu, Screen as MenuScreen};
use crate::menus::Menu;

/// The menu, and the placed terminal it's for.
pub struct ComputersScreen {
    pub menu: ComputersMenu,
    pub reference: FormId,
    /// Waiting on the terminal tutorial (`+0xc0`).
    pub tutorial_wait: bool,
}

pub fn takes(m: &Menu) -> bool {
    matches!(m, Menu::Terminal(..))
}

/// One screen of a terminal record, as the world lists it now.
fn screen_of(
    order: &esm::LoadOrder,
    state: &GameState,
    terminal: FormId,
    reference: FormId,
) -> Screen {
    let t = Terminal::load(order, terminal);
    Screen {
        terminal: terminal.0,
        welcome: t.as_ref().map(|t| t.header.clone()).unwrap_or_default(),
        server: t.as_ref().map_or(0, |t| t.server),
        items: terminal::shown_items(order, state, terminal, reference)
            .into_iter()
            .map(|(i, item)| (i, item.text))
            .collect(),
    }
}

/// Opens the menu (`00757b70`).
pub fn open(screen: &mut MenuScreen, game: &cellview::Game, state: &mut GameState, request: Menu) {
    let Menu::Terminal(base, reference) = request else {
        return;
    };
    let order = &game.order;
    let Some(t) = Terminal::load(order, base) else {
        return;
    };
    let logon = (!terminal::unlocked(state, &t, reference)).then(|| {
        world::hacking::word_length(base, terminal::difficulty(order, state, &t, reference))
    });
    let mut rates = Rates::default();
    for (i, name) in Rates::NAMES.iter().enumerate() {
        if let Some(v) = world::scripting::game_setting(order, name).map(|v| v.max(0.0) as u32) {
            match i {
                0 => rates.menus = v,
                1 => rates.notes = v,
                2 => rates.input = v,
                _ => rates.result_timeout = v,
            }
        }
    }
    let mut menu = ComputersMenu::new(0);
    match screen.load(game, FILE, &mut menu) {
        Ok(tile) => menu.menu = tile,
        Err(e) => {
            println!("The terminal's menu can't be opened: {e}");
            return;
        }
    }
    let setup = Setup {
        screen: screen_of(order, state, base, reference),
        logon,
        rates,
    };
    if !menu.open(&mut screen.ui, setup) {
        println!("MENUS: Computers Menu Creation Failed.");
        screen.ui.detach(menu.menu);
        return;
    }
    // The terminal tutorial (`00757b70`; vanilla's isn't "Auto Display",
    // so it never comes up).
    let tutorial_wait = world::tutorial::ask(
        order,
        &mut state.tutorials,
        world::tutorial::id::TERMINAL,
        world::tutorial::menu::COMPUTERS,
        world::tutorial::menu::DELAY,
    );
    screen
        .open
        .push(OpenMenu::Computers(Box::new(ComputersScreen {
            menu,
            reference,
            tutorial_wait,
        })));
}

/// Every frame (`00758470`).
pub fn update(screen: &mut MenuScreen, state: &GameState, now_ms: f64) {
    for m in &mut screen.open {
        if let OpenMenu::Computers(c) = m {
            // Waiting until its tutorial has been shown.
            if c.tutorial_wait && !state.tutorials.is_shown(world::tutorial::id::TERMINAL) {
                continue;
            }
            c.menu.update(&mut screen.ui, now_ms);
        }
    }
}

/// What the menus asked for; the notices to show.
pub fn after(
    screen: &mut MenuScreen,
    order: &esm::LoadOrder,
    scripts: &world::scripting::ScriptCache,
    state: &mut GameState,
    sounds: &mut HackingSounds,
    now_ms: f64,
) -> Vec<String> {
    let mut notices = Vec::new();
    let MenuScreen {
        ui,
        open,
        terminal_left: left,
        ..
    } = screen;
    for m in open.iter_mut() {
        let OpenMenu::Computers(c) = m else {
            continue;
        };
        let reference = c.reference;
        for request in std::mem::take(&mut c.menu.requests) {
            match request {
                Request::Sound(name, delay) => {
                    if let Some(f) = order.form_by_editor_id(name) {
                        sounds.play_at(f, now_ms + delay);
                    }
                }
                Request::Keep(name) => {
                    if let Some(f) = order.form_by_editor_id(name) {
                        sounds.keep(f);
                    }
                }
                Request::Hum => {
                    if let Some(f) = order.form_by_editor_id(sound::FAN) {
                        sounds.hum(f);
                    }
                }
                Request::PlayForm(f) => sounds.play_at(FormId(f), now_ms),
                Request::Use { terminal, item } => {
                    let used = use_item(
                        order,
                        scripts,
                        state,
                        FormId(terminal),
                        reference,
                        item,
                        &mut notices,
                    );
                    c.menu.used(ui, used);
                }
                Request::Fill { terminal } => {
                    let s = screen_of(order, state, FormId(terminal), reference);
                    c.menu.fill(ui, s);
                }
                Request::Retype { terminal } => {
                    let passing: Vec<usize> =
                        terminal::shown_items(order, state, FormId(terminal), reference)
                            .into_iter()
                            .map(|(i, _)| i)
                            .collect();
                    c.menu.retype(ui, &passing);
                }
                Request::Close => sounds.stop_all(),
                // Leaving (`00757ea0`): the power down (`007ffe40`), the
                // rendered terminal fading out (`007ffaf0`).
                Request::Leave => {
                    if let Some(f) = order.form_by_editor_id(sound::POWER_DOWN) {
                        sounds.play_at(f, now_ms);
                    }
                    *left = true;
                }
            }
        }
    }
    notices
}

/// Picking an item (`00757f70`): its script, its note, its sub-menu.
fn use_item(
    order: &esm::LoadOrder,
    scripts: &world::scripting::ScriptCache,
    state: &mut GameState,
    terminal: FormId,
    reference: FormId,
    index: usize,
    notices: &mut Vec<String>,
) -> Used {
    let Some(item) = Terminal::load(order, terminal).and_then(|t| t.items.get(index).cloned())
    else {
        return Used::default();
    };
    let before = state.events.len();
    if let Some(script) = &item.script {
        // The terminal menu is the open one while its item's script runs
        // (`ForceTerminalBack` asks, `005dc4e0`).
        let was = state.more.menu_open.replace(terminal::TERMINAL_MENU);
        Runner::new(order, scripts, state).run_source(script, Some(reference), Some(reference));
        state.more.menu_open = was;
    }
    // `ForceTerminalBack` from the item's own script acts on this menu.
    let mut back = false;
    let mut i = before.min(state.events.len());
    while i < state.events.len() {
        if matches!(state.events[i], world::scripting::Event::TerminalBack) {
            state.events.remove(i);
            back = true;
        } else {
            i += 1;
        }
    }
    let mut used = Used {
        result: item.result.clone(),
        back,
        ..Used::default()
    };
    if let Some(sub) = item.submenu {
        used.submenu = Some(screen_of(order, state, sub, reference));
        return used;
    }
    let redraw = item.flags & terminal::FORCE_REDRAW != 0;
    if let Some(id) = item.note {
        if item.flags & terminal::ADD_NOTE != 0 && !state.notes.contains(&id) {
            state.notes.insert(id);
            let name = terminal::note(order, id)
                .map(|n| n.title)
                .unwrap_or_default();
            let text = world::scripting::game_setting_text(order, "sComputersAddedNote")
                .unwrap_or_else(|| "Note Added: %s".into())
                .replacen("%s", &name, 1);
            notices.push(text);
        }
        used.note = terminal::note(order, id).map(|n| match n.kind {
            note_kind::TEXT => NoteView::Text(n.text.unwrap_or_default()),
            note_kind::IMAGE => NoteView::Image(n.image.unwrap_or_default()),
            note_kind::SOUND => n.sound.map_or(NoteView::Voice, |s| NoteView::Sound(s.0)),
            _ => NoteView::Voice,
        });
    }
    if redraw {
        used.redraw = Some(screen_of(order, state, terminal, reference));
    }
    used
}
