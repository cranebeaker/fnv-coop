//! Menus waiting for the player, queued by scripts and the world. The
//! game's own menus (`game_menus`) take the ones they show (message boxes,
//! containers, trading, levelling up, tag skills, traits, the name, sleeping
//! waiting and the vigor tester); terminals are shown here as a text panel
//! in the middle of the screen, and message boxes
//! too when the game's menu files can't be read (the Pip-Boy and
//! lockpicking run their own: `pipboy`, `lockpick`). While one is
//! open the game waits (no walking, no script time), as it does in the
//! game's menus.
//!
//! Keys: number keys pick a button; arrows move and change values; Enter
//! accepts.

use std::collections::VecDeque;

use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use esm::FormId;
use world::chargen::CharacterMenu;
use world::dialogue::PLAYER_REF;
use world::scripting::{Facts, Runner};

use crate::dialogue::{Conversation, DialogueState};
use crate::scripts::Scripts;
use crate::walk::Player;
use crate::GameFiles;

/// A menu waiting for the player.
pub enum Menu {
    Message {
        title: Option<String>,
        text: String,
        buttons: Vec<(usize, String)>,
    },
    /// A box the game itself puts up (`world::scripting::Event::Popup`:
    /// a reputation's new title), with one OK button whose answer goes to
    /// no script.
    Popup {
        title: Option<String>,
        text: String,
        icon: Option<String>,
        sound: Option<String>,
    },
    Character(CharacterMenu),
    /// A container opened: its reference and name.
    Container(FormId, String),
    /// Trading with a merchant (their reference).
    Barter(FormId),
    /// Crafting (`ShowRecipeMenu`): who crafts and the recipe category.
    Recipe {
        actor: FormId,
        category: FormId,
    },
    /// A merchant's repairs (their reference, `ShowRepairMenu`).
    RepairServices(FormId),
    /// Trading things with a companion (their reference,
    /// `OpenTeammateContainer`).
    Teammate(FormId),
    /// A companion's wheel of orders (their reference: the player using a
    /// teammate).
    CompanionWheel(FormId),
    /// A game of Caravan (`ShowCaravanMenu`): the opponent, their deck, the
    /// AI's difficulty, the share of their funds they bet.
    Caravan {
        npc: FormId,
        deck: FormId,
        difficulty: i32,
        share: f32,
    },
    /// A casino game (`ShowSlotMachineMenuParams`, `ShowBlackJackMenuParams`,
    /// `ShowRouletteMenuParams`), its `Create` checks passed
    /// (`game_menus::casino`): the casino, the bets' limits.
    Casino {
        game: world::casino::Game,
        casino: FormId,
        min_bet: i32,
        max_bet: i32,
    },
    /// A computer terminal used (`world::terminal`): its record and the
    /// placed terminal.
    Terminal(FormId, FormId),
    /// The hacking menu for a terminal (`game_menus::hacking`): its record
    /// and the placed terminal.
    Hacking(FormId, FormId),
    /// The game's sleep/wait menu (`world::living::sleep`): T, a bed, or
    /// a script's `ShowSleepWaitMenu`; in sleep or wait mode.
    SleepWait {
        sleep: bool,
    },
    /// A level gained (`world::experience::level_up`): its skill points to
    /// share out, then a perk on perk levels.
    LevelUp(world::experience::LevelUp),
    /// A script's `ShowTutorialMenu`: the tutorial menu with this message
    /// (`game_menus::tutorial::show_form`).
    Tutorial(FormId),
}

/// A menu on screen, with what's been chosen so far.
enum Open {
    Message {
        title: Option<String>,
        text: String,
        buttons: Vec<(usize, String)>,
    },
    /// A terminal: the screens gone into (its own first, then sub-menus),
    /// the item chosen, a note being read, and what the last item printed;
    /// `locked` when the player can't get in.
    Terminal {
        reference: FormId,
        stack: Vec<FormId>,
        row: usize,
        reading: Option<String>,
        printed: String,
        locked: Option<Locked>,
    },
}

/// Carries out the `ForceTerminalBack`s scripts asked for (`005dc4e0`):
/// each pops the terminal's screen stack and shows the screen before
/// (`00758a80`); with none left the terminal closes (`00757ea0`): `true`.
fn terminal_backs(
    state: &mut world::scripting::GameState,
    stack: &mut Vec<FormId>,
    row: &mut usize,
    printed: &mut String,
) -> bool {
    let back = world::scripting::Event::TerminalBack;
    let count = state.events.iter().filter(|e| **e == back).count();
    state.events.retain(|e| *e != back);
    for _ in 0..count {
        stack.pop();
        *row = 0;
        printed.clear();
        if stack.is_empty() {
            return true;
        }
    }
    false
}

/// Why a terminal stays shut.
#[derive(Clone, Copy)]
enum Locked {
    /// Locked out (the hacking menu's "TERMINAL LOCKED").
    Out,
}

/// A terminal screen's items the player can pick (their conditions pass),
/// as (item, its number in the record).
fn terminal_items(
    order: &esm::LoadOrder,
    state: &world::scripting::GameState,
    terminal: FormId,
    reference: FormId,
) -> Vec<world::terminal::TerminalItem> {
    let Some(t) = world::terminal::Terminal::load(order, terminal) else {
        return Vec::new();
    };
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    t.items
        .into_iter()
        .filter(|i| facts.conditions_pass(&i.conditions, reference, PLAYER_REF))
        .collect()
}

/// Menus scripts opened, one at a time.
#[derive(Resource, Default)]
pub struct Menus {
    queue: VecDeque<Menu>,
    open: Option<Open>,
    /// The Pip-Boy is up (`pipboy`): a menu as far as the rest of the
    /// game is concerned (no HUD, no walking, no fighting).
    pub pipboy: bool,
    /// The lockpicking menu is open (`lockpick`, which runs it).
    pub lockpicking: bool,
    /// One of the game's own menus is on screen (`game_menus`).
    pub game_open: bool,
    /// The game's own menus can be shown (their files and fonts read).
    pub game_ready: bool,
}

impl Menus {
    pub fn push(&mut self, menu: Menu) {
        self.queue.push_back(menu);
    }

    pub fn is_open(&self) -> bool {
        self.open.is_some()
            || !self.queue.is_empty()
            || self.pipboy
            || self.lockpicking
            || self.game_open
    }

    /// Whether nothing but the game's own menus (`game_open`) is up: no
    /// viewer menu open or waiting, no Pip-Boy, no lockpicking.
    pub fn only_game_menus(&self) -> bool {
        self.open.is_none() && self.queue.is_empty() && !self.pipboy && !self.lockpicking
    }

    /// Whether one of these menus (not the Pip-Boy) is up or waiting.
    pub fn others_open(&self) -> bool {
        self.open.is_some() || !self.queue.is_empty() || self.lockpicking || self.game_open
    }

    /// The next menu waiting, taken off the queue when `wanted` says the
    /// game's own menus show it (`game_menus`), and nothing here is open.
    pub fn take_next(&mut self, wanted: impl Fn(&Menu) -> bool) -> Option<Menu> {
        if self.open.is_some() || !self.queue.front().is_some_and(wanted) {
            return None;
        }
        self.queue.pop_front()
    }
}

/// The menu panel.
#[derive(Component)]
pub struct MenuText;

pub fn setup_menu_text(mut commands: Commands) {
    commands.spawn((
        Text::new(String::new()),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.85, 0.55)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(20.0),
            left: Val::Percent(30.0),
            width: Val::Percent(40.0),
            padding: UiRect::all(Val::Px(16.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        Visibility::Hidden,
        MenuText,
    ));
}

/// Opens the next menu: the character menus start from the player as they
/// are now.
fn open(
    order: &esm::LoadOrder,
    state: &mut world::scripting::GameState,
    menu: Menu,
) -> Option<Open> {
    Some(match menu {
        Menu::Message {
            title,
            text,
            buttons,
        } => Open::Message {
            title,
            text,
            buttons,
        },
        // Without the game's message menu: shown like a script's box.
        Menu::Popup { title, text, .. } => Open::Message {
            title,
            text,
            buttons: vec![(0, "OK".to_string())],
        },
        // Only the game's own menus show these (`game_menus`).
        Menu::Container(..)
        | Menu::Barter(..)
        | Menu::Recipe { .. }
        | Menu::RepairServices(..)
        | Menu::Teammate(..)
        | Menu::CompanionWheel(..)
        | Menu::Caravan { .. }
        | Menu::Casino { .. }
        | Menu::LevelUp(..)
        | Menu::Character(CharacterMenu::Traits { .. })
        | Menu::Character(CharacterMenu::TagSkills { .. })
        | Menu::Character(CharacterMenu::Name)
        | Menu::Character(CharacterMenu::Special { .. })
        | Menu::SleepWait { .. }
        | Menu::Tutorial(..) => {
            println!("That menu can't be opened: the game's menus aren't available.");
            return None;
        }
        // A terminal the player gets into (`game_menus::hacking::use_terminal`
        // decided).
        Menu::Terminal(terminal, reference) => Open::Terminal {
            reference,
            stack: vec![terminal],
            row: 0,
            reading: None,
            printed: String::new(),
            locked: None,
        },
        // The hacking menu without the game's menu files: locked out shows
        // as the menu would; otherwise the terminal counts as hacked.
        Menu::Hacking(terminal, reference) => {
            use world::terminal::Access;
            let t = world::terminal::Terminal::load(order, terminal);
            let access = t
                .as_ref()
                .map(|t| world::terminal::access(order, state, t, reference));
            if let (Some(Access::Hack), Some(t)) = (access, &t) {
                world::terminal::hacked(order, state, t, reference);
                println!("Hacked the terminal (the hacking menu's files aren't available).");
            }
            Open::Terminal {
                reference,
                stack: vec![terminal],
                row: 0,
                reading: None,
                printed: String::new(),
                locked: (access == Some(Access::LockedOut)).then_some(Locked::Out),
            }
        }
    })
}

/// What the panel shows.
fn describe(order: &esm::LoadOrder, state: &world::scripting::GameState, open: &Open) -> String {
    let marker = |on: bool| if on { "> " } else { "  " };
    match open {
        Open::Message {
            title,
            text,
            buttons,
        } => {
            let mut s = String::new();
            if let Some(t) = title {
                s.push_str(&format!("{t}\n\n"));
            }
            if !text.is_empty() {
                s.push_str(&format!("{text}\n\n"));
            }
            for (i, (_, label)) in buttons.iter().enumerate() {
                s.push_str(&format!("{}) {label}\n", i + 1));
            }
            s
        }
        Open::Terminal {
            reference,
            stack,
            row,
            reading,
            printed,
            locked,
        } => {
            let Some(&screen) = stack.last() else {
                return String::new();
            };
            let t = world::terminal::Terminal::load(order, screen);
            let mut s = String::new();
            if let Some(t) = &t {
                if !t.name.is_empty() {
                    s.push_str(&format!("{}\n", t.name));
                }
                if !t.header.is_empty() {
                    s.push_str(&format!("{}\n", t.header));
                }
                s.push('\n');
            }
            if let Some(why) = locked {
                let text = match why {
                    // `sHackingLockout3` and `4`, as the hacking menu shows.
                    Locked::Out => "TERMINAL LOCKED\nPLEASE CONTACT AN ADMINISTRATOR".to_string(),
                };
                s.push_str(&format!("{text}\n\n(Enter or Esc leaves.)"));
                return s;
            }
            if let Some(text) = reading {
                s.push_str(&format!("{text}\n\n(Enter or Esc: back.)"));
                return s;
            }
            for (i, item) in terminal_items(order, state, screen, *reference)
                .iter()
                .enumerate()
            {
                s.push_str(&format!("{}[{}]\n", marker(i == *row), item.text));
            }
            if !printed.is_empty() {
                s.push_str(&format!("\n{printed}\n"));
            }
            s.push_str("\n(Up/Down choose, Enter picks, Esc goes back or leaves.)");
            s
        }
    }
}

/// Runs the menu on screen: takes the keyboard, and once answered, tells
/// the game what was chosen.
#[allow(clippy::too_many_arguments)]
pub fn run_menus(
    game: Res<GameFiles>,
    scripts: Res<Scripts>,
    mut menus: ResMut<Menus>,
    mut state: ResMut<DialogueState>,
    mut player: ResMut<Player>,
    conversation: Res<Conversation>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut typed: EventReader<KeyboardInput>,
    mut panel: Query<(&mut Text, &mut Visibility), With<MenuText>>,
) {
    // The lockpicking menu has the keyboard while it's open (`lockpick`).
    if menus.lockpicking {
        return;
    }
    let order = &game.0.order;
    let state = &mut state.0;
    // The game's own menus have the screen and the keys.
    if menus.game_open {
        typed.clear();
        return;
    }
    // A level-up waiting comes once the player is out of combat and no
    // other menu or conversation is on (`world::experience`).
    if menus.open.is_none()
        && menus.queue.is_empty()
        && !menus.pipboy
        && conversation.0.is_none()
        && world::experience::level_up_ready(state)
    {
        let up = world::experience::level_up(order, state);
        println!(
            "Level {}: {} skill points{}.",
            up.level,
            up.skill_points,
            if up.perk { " and a perk" } else { "" }
        );
        menus.push(Menu::LevelUp(up));
    }
    if menus.open.is_none() {
        // The game's own menus take theirs (`game_menus`) next frame.
        if menus.game_ready && menus.queue.front().is_some_and(crate::game_menus::takes) {
            typed.clear();
            return;
        }
        let Some(next) = menus.queue.pop_front() else {
            typed.clear();
            return;
        };
        menus.open = open(order, state, next);
        if menus.open.is_none() {
            return;
        }
        player.ready = false;
    }
    let Some(open) = menus.open.as_mut() else {
        return;
    };
    let pressed = |k: KeyCode| keys.just_pressed(k);
    let (up, down) = (pressed(KeyCode::ArrowUp), pressed(KeyCode::ArrowDown));
    let enter = pressed(KeyCode::Enter) || pressed(KeyCode::NumpadEnter);
    let digit = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
    ]
    .iter()
    .position(|&k| pressed(k));
    let step = |row: &mut usize, len: usize| {
        if len == 0 {
            return;
        }
        if up {
            *row = (*row + len - 1) % len;
        }
        if down {
            *row = (*row + 1) % len;
        }
    };
    let mut done = false;
    match open {
        Open::Message { buttons, .. } => {
            if let Some(i) = digit.filter(|&i| i < buttons.len()) {
                state.button = Some(buttons[i].0 as i32);
                println!("Message box: picked \"{}\".", buttons[i].1);
                done = true;
            }
            if buttons.is_empty() && (enter || pressed(KeyCode::Escape)) {
                done = true;
            }
        }
        Open::Terminal {
            reference,
            stack,
            row,
            reading,
            printed,
            locked,
        } => {
            let back = pressed(KeyCode::Escape) || pressed(KeyCode::Tab);
            // The terminal menu is open while it's shown (its scripts'
            // `ForceTerminalBack` asks).
            state.more.menu_open = Some(world::terminal::TERMINAL_MENU);
            // `ForceTerminalBack`s since last frame: back a screen each.
            if terminal_backs(state, stack, row, printed) {
                done = true;
            } else if locked.is_some() {
                done = enter || back;
            } else if reading.is_some() {
                if enter || back {
                    *reading = None;
                }
            } else if let Some(&screen) = stack.last() {
                let list = terminal_items(order, state, screen, *reference);
                step(row, list.len());
                *row = (*row).min(list.len().saturating_sub(1));
                if enter {
                    if let Some(item) = list.get(*row) {
                        *printed = item.result.clone().unwrap_or_default();
                        if let Some(script) = &item.script {
                            let r = *reference;
                            Runner::new(order, &scripts.0, state).run_source(
                                script,
                                Some(r),
                                Some(r),
                            );
                        }
                        // The item's script went back (`ForceTerminalBack`)
                        // before its sub-menu, if any, opens (the order
                        // isn't traced).
                        if terminal_backs(state, stack, row, printed) {
                            done = true;
                        }
                        if let Some(note) = item.note {
                            // Flag 0x01: it goes into the Pip-Boy too.
                            if item.flags & 0x01 != 0 {
                                state.notes.insert(note);
                            }
                            *reading = world::terminal::note_text(order, note)
                                .map(|(title, text)| format!("{title}\n\n{text}"));
                        }
                        if let Some(sub) = item.submenu {
                            stack.push(sub);
                            *row = 0;
                        }
                    }
                } else if back {
                    // Out of a sub-menu, else away from the terminal.
                    if stack.len() > 1 {
                        stack.pop();
                        *row = 0;
                        printed.clear();
                    } else {
                        done = true;
                    }
                }
            }
        }
    }
    let shown = if done {
        menus.open = None;
        if state.more.menu_open == Some(world::terminal::TERMINAL_MENU) {
            state.more.menu_open = None;
        }
        if menus.queue.is_empty() && conversation.0.is_none() {
            player.ready = true;
        }
        String::new()
    } else {
        describe(order, state, menus.open.as_ref().unwrap())
    };
    for (mut text, mut visibility) in &mut panel {
        if text.0 != shown {
            text.0 = shown.clone();
        }
        *visibility = if shown.is_empty() {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
    // The menu has the keyboard.
    typed.clear();
    keys.reset_all();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn force_terminal_back_pops_screens_then_closes() {
        let mut state = world::scripting::GameState::default();
        let back = world::scripting::Event::TerminalBack;
        let mut stack = vec![FormId(1), FormId(2), FormId(3)];
        let (mut row, mut printed) = (4, "Done.".to_string());
        // Nothing asked: nothing changes.
        assert!(!terminal_backs(
            &mut state,
            &mut stack,
            &mut row,
            &mut printed
        ));
        assert_eq!(stack.len(), 3);
        // One back: the screen before, from its top, nothing printed.
        state.events.push(back.clone());
        assert!(!terminal_backs(
            &mut state,
            &mut stack,
            &mut row,
            &mut printed
        ));
        assert_eq!(
            (stack.clone(), row, printed.as_str()),
            (vec![FormId(1), FormId(2)], 0, "")
        );
        assert!(state.events.is_empty());
        // Two more: past the first screen, the terminal closes.
        state.events.extend([back.clone(), back]);
        assert!(terminal_backs(
            &mut state,
            &mut stack,
            &mut row,
            &mut printed
        ));
        assert!(stack.is_empty());
    }
}
