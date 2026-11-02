//! The hacking menu (`ui::menus::hacking`) for a terminal the player uses
//! (`world::terminal::access` says hack, or locked out): the session from
//! `world::hacking`, with the dictionary from the game's files; what the
//! menu asks back (sounds, the terminal hacked or locking the player out)
//! carried out; on the password, the terminal's own screen opens after.
//!
//! Using a terminal (`00501310`): too little Science gives the game's
//! message ("A Science skill of N is required to hack this terminal.",
//! `UIPopUpMessageGeneral`) and nothing opens.
//!
//! The retry time (`dwRetryTime`): `iHackingRetryMilliseconds` (10 s) from
//! a hacking menu closing, kept here between menus. The screen's address
//! column starts at the low half of a stack address in the game; here a
//! random multiple of 4 (the buffer's alignment), new each time.

use std::collections::HashMap;

use bevy::prelude::*;
use esm::FormId;
use ui::menus::hacking::{sound, HackingMenu, Rates, Request, Setup, FILE};
use world::dialogue::PLAYER_REF;
use world::grass::Twister;
use world::scripting::GameState;
use world::terminal::{self, Access, Terminal};

use super::{OpenMenu, Screen};
use crate::menus::Menu;
use crate::sounds::PcmSound;
use crate::GameFiles;

/// The dictionary (`00768070`: `Data\Menus\FalloutDict.txt`).
const DICTIONARY: &str = "menus\\falloutdict.txt";

/// `iHackingRetryMilliseconds` (exe default; the data sets none).
const RETRY: (&str, i32) = ("iHackingRetryMilliseconds", 10_000);

/// The menu, and the terminal it's for.
pub struct HackingScreen {
    pub menu: HackingMenu,
    pub terminal: FormId,
    pub reference: FormId,
    pub granted: bool,
    /// Waiting on the hacking tutorial (`+0x1da`).
    pub tutorial_wait: bool,
}

/// The retry time, and the menu's sounds: delayed ones, the hum, and the
/// ones kept going while lines type.
#[derive(Resource, Default)]
pub struct HackingSounds {
    pub retry_until_ms: f64,
    orders: Vec<Order>,
    pending: Vec<(f64, FormId)>,
    hum: Option<Entity>,
    kept: HashMap<FormId, Entity>,
}

impl HackingSounds {
    /// A sound record after a delay (ms from `now`).
    pub fn play_at(&mut self, form: FormId, at_ms: f64) {
        self.orders.push(Order::At(at_ms, form));
    }
    /// A sound kept going while something types.
    pub fn keep(&mut self, form: FormId) {
        self.orders.push(Order::Keep(form));
    }
    /// The menus' looping hum.
    pub fn hum(&mut self, form: FormId) {
        self.orders.push(Order::Hum(form));
    }
    /// Every sound of the menus stopped.
    pub fn stop_all(&mut self) {
        self.orders.push(Order::StopAll);
    }
}

enum Order {
    At(f64, FormId),
    Keep(FormId),
    Hum(FormId),
    StopPending(FormId),
    StopAll,
}

/// What using a terminal does.
pub enum Using {
    Open(Menu),
    Refused(String),
}

/// The player uses a terminal (`00501310` / `005015f0`).
pub fn use_terminal(
    order: &esm::LoadOrder,
    state: &GameState,
    base: FormId,
    reference: FormId,
) -> Using {
    let Some(t) = Terminal::load(order, base) else {
        return Using::Open(Menu::Terminal(base, reference));
    };
    match terminal::access(order, state, &t, reference) {
        Access::Open => Using::Open(Menu::Terminal(base, reference)),
        Access::Hack | Access::LockedOut => Using::Open(Menu::Hacking(base, reference)),
        Access::NeedsScience(n) => Using::Refused(
            world::scripting::game_setting_text(order, "sHackIneligible")
                .unwrap_or_else(|| "A %s skill of %d is required to hack this terminal.".into())
                .replacen(
                    "%s",
                    &world::chargen::actor_value_name(order, terminal::SCIENCE),
                    1,
                )
                .replacen("%d", &n.to_string(), 1),
        ),
    }
}

pub fn takes(m: &Menu) -> bool {
    matches!(m, Menu::Hacking(..))
}

/// The game's generator for one menu: seeded from the clock and run on, as
/// the game's has been since it started (`NV_HACKING_SEED` fixes the seed,
/// for screenshots).
fn generator() -> Twister {
    let seed = std::env::var("NV_HACKING_SEED")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(1, |d| d.subsec_nanos() ^ d.as_secs() as u32)
        });
    let mut t = Twister::seeded(seed | 1);
    for _ in 0..624 * 3 {
        t.next_u32();
    }
    t
}

/// Opens the menu (`00765b80`).
pub fn open(
    screen: &mut Screen,
    game: &cellview::Game,
    state: &mut GameState,
    request: Menu,
    sounds: &HackingSounds,
) {
    let Menu::Hacking(base, reference) = request else {
        return;
    };
    let order = &game.order;
    let Some(t) = Terminal::load(order, base) else {
        return;
    };
    let mut rng = generator();
    let session = if terminal::access(order, state, &t, reference) == Access::LockedOut {
        None
    } else {
        let Some(text) = game.assets.read(DICTIONARY).ok().flatten() else {
            println!("Hacking Menu: Couldn't locate dictionary file Data\\Menus\\FalloutDict.txt");
            return;
        };
        let d = terminal::difficulty(order, state, &t, reference);
        let length = world::hacking::word_length(base, d);
        let science = world::perks::apply_for(
            order,
            state,
            PLAYER_REF,
            world::hacking::SCIENCE_BONUS_ENTRY,
            terminal::science(order, state),
            &[],
        );
        let count = world::hacking::word_count(
            science,
            world::hacking::min_skill(d),
            world::hacking::setting_int(order, world::hacking::MAX_WORDS),
            world::hacking::setting_int(order, world::hacking::MIN_WORDS),
        );
        let dictionary = world::hacking::dictionary_words(&text, length);
        // The game's clock (`GetTickCount`); with `NV_HACKING_SEED` one
        // that counts its readings, so a screen comes out the same.
        let start = std::time::Instant::now();
        let fixed = std::env::var("NV_HACKING_SEED").is_ok();
        let mut readings = 0u32;
        let mut clock = || {
            readings += 1;
            if fixed {
                readings
            } else {
                start.elapsed().as_millis() as u32
            }
        };
        let session = world::hacking::Game::new(&dictionary, length, count, &mut rng, &mut clock);
        if let Some(g) = &session {
            println!(
                "Hacking: {} words of {} letters, {} attempts.",
                g.words.len(),
                length,
                g.max_attempts
            );
            // The game's own debug line (`00768070`), for testing.
            if fixed {
                println!("The password is '{}'", String::from_utf8_lossy(&g.password));
            }
        }
        session
    };
    let mut rates = Rates::default();
    for (i, name) in Rates::NAMES.iter().enumerate() {
        let v = world::scripting::game_setting(order, name).map(|v| v as i32);
        if let Some(v) = v {
            match i {
                0 => rates.output = v,
                1 => rates.input = v,
                2 => rates.dump = v,
                3 => rates.flash_on = v,
                _ => rates.flash_off = v,
            }
        }
    }
    let address = (rng.below(0x4000) * 4) as u16;
    let mut menu = HackingMenu::new(0);
    match screen.load(game, FILE, &mut menu) {
        Ok(tile) => menu.menu = tile,
        Err(e) => {
            println!("The hacking menu can't be opened: {e}");
            return;
        }
    }
    let setup = Setup {
        game: session,
        rng,
        address,
        rates,
        retry_until: sounds.retry_until_ms,
    };
    if !menu.open(&mut screen.ui, setup) {
        println!("MENUS: Hacking Menu Creation Failed.");
        screen.ui.detach(menu.menu);
        return;
    }
    // The hacking tutorial (`00765b80`): the menu waits until it's shown.
    let tutorial_wait = world::tutorial::ask(
        order,
        &mut state.tutorials,
        world::tutorial::id::HACKING,
        world::tutorial::menu::HACKING,
        world::tutorial::menu::DELAY,
    );
    screen.open.push(OpenMenu::Hacking(Box::new(HackingScreen {
        menu,
        terminal: base,
        reference,
        granted: false,
        tutorial_wait,
    })));
}

/// Every frame (`00767c90`): only while it's the menu on top, and not
/// while it waits on its tutorial.
pub fn update(screen: &mut Screen, state: &GameState, now_ms: f64) {
    let over = screen.interface.over;
    let x = screen.pointer.map(|(x, _)| x);
    let top = screen.open.len().saturating_sub(1);
    for (i, m) in screen.open.iter_mut().enumerate() {
        if let OpenMenu::Hacking(h) = m {
            let waiting =
                h.tutorial_wait && !state.tutorials.is_shown(world::tutorial::id::HACKING);
            if i != top || waiting {
                continue;
            }
            h.menu.update(&mut screen.ui, now_ms, over, x);
        }
    }
}

/// What the menus asked for; the terminal's screen to open after a hack.
pub fn after(
    screen: &mut Screen,
    order: &esm::LoadOrder,
    state: &mut GameState,
    sounds: &mut HackingSounds,
    now_ms: f64,
) -> Vec<Menu> {
    let mut then = Vec::new();
    let mut left = false;
    for m in &mut screen.open {
        let OpenMenu::Hacking(h) = m else {
            continue;
        };
        for request in std::mem::take(&mut h.menu.requests) {
            let form = |name: &str| order.form_by_editor_id(name);
            match request {
                Request::Sound(name, delay) => {
                    if let Some(f) = form(name) {
                        sounds.orders.push(Order::At(now_ms + delay, f));
                    }
                }
                Request::Keep(name) => {
                    if let Some(f) = form(name) {
                        sounds.orders.push(Order::Keep(f));
                    }
                }
                Request::Hum => {
                    if let Some(f) = form(sound::FAN) {
                        sounds.orders.push(Order::Hum(f));
                    }
                }
                Request::StopTyping => {
                    if let Some(f) = form(sound::SINGLE) {
                        sounds.orders.push(Order::StopPending(f));
                    }
                }
                Request::Granted => {
                    if let Some(t) = Terminal::load(order, h.terminal) {
                        terminal::hacked(order, state, &t, h.reference);
                    }
                    h.granted = true;
                    println!("Hacked the terminal.");
                }
                Request::LockedOut => {
                    terminal::lock_out(state, h.reference);
                    println!("The terminal locked the player out.");
                }
                Request::Close => {
                    sounds.orders.push(Order::StopAll);
                    let retry = world::hacking::setting_int(order, RETRY);
                    sounds.retry_until_ms = now_ms + f64::from(retry);
                    if h.granted {
                        then.push(Menu::Terminal(h.terminal, h.reference));
                    }
                }
                // Leaving (`00766aa0`): the power down (`007ffe40`), the
                // rendered terminal fading out (`007ffaf0`).
                Request::Leave => {
                    if let Some(f) = form(ui::menus::computers::sound::POWER_DOWN) {
                        sounds.orders.push(Order::At(now_ms, f));
                    }
                    left = true;
                }
            }
        }
    }
    screen.terminal_left |= left;
    then
}

/// Plays the menu's sounds.
#[allow(clippy::too_many_arguments)]
pub fn play_sounds(
    mut commands: Commands,
    game: Res<GameFiles>,
    state: Res<crate::dialogue::DialogueState>,
    mut sounds: ResMut<HackingSounds>,
    mut wavs: ResMut<Assets<PcmSound>>,
    time: Res<Time<bevy::time::Real>>,
) {
    let now = time.elapsed_secs_f64() * 1000.0;
    let order = &game.0.order;
    let pick = state.0.dice;
    let mut start = |commands: &mut Commands, id: FormId, looping: bool| {
        world::sound::Sound::load(order, id)
            .and_then(|s| crate::sounds::play(commands, &game.0, &mut wavs, &s, pick, looping))
    };
    let s = &mut *sounds;
    for order in std::mem::take(&mut s.orders) {
        match order {
            Order::At(at, f) => s.pending.push((at, f)),
            Order::StopPending(f) => s.pending.retain(|(_, g)| *g != f),
            Order::Keep(f) => {
                let alive = s
                    .kept
                    .get(&f)
                    .is_some_and(|&e| commands.get_entity(e).is_ok());
                if !alive {
                    if let Some(e) = start(&mut commands, f, false) {
                        s.kept.insert(f, e);
                    }
                }
            }
            Order::Hum(f) => {
                if s.hum.is_none() {
                    s.hum = start(&mut commands, f, true);
                }
            }
            Order::StopAll => {
                s.pending.clear();
                for e in s
                    .hum
                    .take()
                    .into_iter()
                    .chain(s.kept.drain().map(|(_, e)| e))
                {
                    if let Ok(mut entity) = commands.get_entity(e) {
                        entity.despawn();
                    }
                }
            }
        }
    }
    let due: Vec<FormId> = s
        .pending
        .iter()
        .filter(|(at, _)| *at <= now)
        .map(|(_, f)| *f)
        .collect();
    s.pending.retain(|(at, _)| *at > now);
    for f in due {
        start(&mut commands, f, false);
    }
}
