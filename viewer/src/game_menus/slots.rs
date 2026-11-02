//! The slot machines (`ShowSlotMachineMenuParams`, `menus::Menu::Casino`
//! with `Game::Slots`): the machine runs in `world::casino::slots`, its
//! tiles in `ui::menus::slots`, its 3D in `cellview::slots` (drawn by
//! `crate::casino_scene`). Here the machine's effects are carried out:
//! sounds, the models posed, the reels' faces retextured, the player's
//! line for the casino kept, the chips settled as the menu closes.
//!
//! `Create`'s checks (the anti-cheat lock, the ban, the chips, the least
//! winnings) are made where the script asks (`scripts`, with
//! `casino::create`), as the game makes them before any menu loads.

use std::collections::HashMap;
use std::sync::Arc;

use cellview::caravan::Pose;
use cellview::slots::SlotModels;
use cellview::Game;
use esm::FormId;
use ui::menus::slots::{SlotsMenu, FILE};
use world::casino::slots::{self, Effect, Model, SlotMachine, Status};
use world::casino::Casino;
use world::dialogue::PLAYER_REF;
use world::scripting::GameState;

use super::{OpenMenu, Screen};

/// The slot machine on screen.
pub struct SlotsScreen {
    pub menu: SlotsMenu,
    pub machine: SlotMachine,
    pub casino: Casino,
    pub models: Arc<SlotModels>,
    /// Each model's pose (the machine, the three reels).
    pub poses: [Pose; 4],
    /// Each reel face's texture slot (model index, shape lower case).
    pub faces: HashMap<(usize, String), usize>,
    /// Bumped whenever a face changes.
    pub texture_changes: u64,
    pub closed: bool,
}

/// Whether this module shows a request.
pub fn takes(m: &crate::menus::Menu) -> bool {
    matches!(
        m,
        crate::menus::Menu::Casino {
            game: world::casino::Game::Slots,
            ..
        }
    )
}

/// The sequences' lengths, from the models.
struct Clips<'a>(&'a SlotModels);

impl slots::Clips for Clips<'_> {
    fn end(&self, model: Model, sequence: &str) -> f32 {
        self.0
            .models
            .get(index(model))
            .and_then(|m| m.sequence(sequence))
            .map_or(0.0, |s| s.stop - s.start)
    }
}

/// A model's index (the scene node's child).
fn index(model: Model) -> usize {
    match model {
        Model::Machine => 0,
        Model::Reel(i) => 1 + i.min(2),
    }
}

/// Opens the menu (`SlotMachineMenu::Create` after its checks); returns
/// the sounds it starts with (`GAMESlotsActivate`).
pub fn open(
    screen: &mut Screen,
    game: &Game,
    state: &mut GameState,
    request: crate::menus::Menu,
) -> Vec<FormId> {
    let crate::menus::Menu::Casino {
        casino,
        min_bet,
        max_bet,
        ..
    } = request
    else {
        return Vec::new();
    };
    let order = &game.order;
    let Some(casino) = Casino::load(order, casino) else {
        return Vec::new();
    };
    let models = cellview::slots::load(
        &game.assets,
        &casino.models[world::casino::model::SLOT_MACHINE],
        &casino.textures[..7],
    );
    let Some(models) = models else {
        println!("The slot machine can't be read.");
        return Vec::new();
    };
    let chips = state.item_count(order, PLAYER_REF, casino.chip);
    let luck = world::scripting::Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(PLAYER_REF, 11)
    .unwrap_or(5.0) as f32;
    let data = *world::casino::data_mut(state, casino.form);
    let Some((machine, fx)) = SlotMachine::open(
        &casino.slot_stops,
        casino.max_winnings,
        min_bet,
        max_bet,
        chips,
        luck,
        data,
    ) else {
        let text = "Casino Form is not setup properly. # of stops for the slot reels must sum to 14. Exiting out of Slot machine menu.";
        println!("{text}");
        return Vec::new();
    };
    let mut code = SlotsMenu::new(0);
    let tile = match screen.load(game, FILE, &mut code) {
        Ok(t) => t,
        Err(e) => {
            println!("MENUS: Slot Machine Menu Creation Failed... ({e})");
            return Vec::new();
        }
    };
    code.menu = tile;
    println!(
        "Slots at {} ({}): bets {min_bet} to {max_bet}, {chips} chips, Luck {luck}.",
        casino.name, casino.form
    );
    let mut s = SlotsScreen {
        menu: code,
        machine,
        casino,
        models: Arc::new(models),
        poses: Default::default(),
        faces: HashMap::new(),
        texture_changes: 0,
        closed: false,
    };
    // `Prepare3DElements`: each reel's `Forward` started and every model
    // updated at 0.
    for i in 1..4 {
        s.poses[i].deactivate_all();
        s.poses[i].activate(slots::sequence::FORWARD);
    }
    s.menu.open(&mut screen.ui);
    let mut sounds = Vec::new();
    let mut messages = Vec::new();
    s.apply(fx, order, state, 0.0, &mut sounds, &mut messages, None);
    for i in 0..4 {
        s.poses[i].update(&s.models.models[i], 0.0);
    }
    s.fill(&mut screen.ui, order);
    screen.open.push(OpenMenu::Slots(Box::new(s)));
    sounds
}

impl SlotsScreen {
    /// The reels' faces for the reels whose result changed.
    fn swap(&mut self, back: bool) {
        for reel in 0..3 {
            let (r, start) = (self.machine.results[reel], self.machine.starts[reel]);
            if r == start {
                continue;
            }
            let faces: Vec<(i32, usize)> = if back {
                slots::back_faces(r).to_vec()
            } else {
                slots::front_faces(r).to_vec()
            };
            for (offset, slot) in faces {
                let shape = slots::face_name(reel, offset).to_ascii_lowercase();
                self.faces.insert((1 + reel, shape), slot);
            }
        }
        self.texture_changes += 1;
    }

    /// Carries out the machine's effects; `lock` stamps the anti-cheat
    /// lock as it closes (`now` seconds).
    #[allow(clippy::too_many_arguments)]
    fn apply(
        &mut self,
        fx: Vec<Effect>,
        order: &esm::LoadOrder,
        state: &mut GameState,
        now: f64,
        sounds: &mut Vec<FormId>,
        messages: &mut Vec<crate::hud::HudMessage>,
        mut lock: Option<&mut world::casino::AntiCheat>,
    ) {
        for e in fx {
            match e {
                Effect::Sound(name) => sounds.extend(order.form_by_editor_id(&name)),
                Effect::Activate { model, sequence } => self.poses[index(model)].activate(sequence),
                Effect::Deactivate { model, sequence } => {
                    self.poses[index(model)].deactivate(sequence)
                }
                Effect::DeactivateAll(model) => self.poses[index(model)].deactivate_all(),
                Effect::Update { model, time } => {
                    let i = index(model);
                    self.poses[i].update(&self.models.models[i], time);
                }
                Effect::SwapFront => self.swap(false),
                Effect::SwapBack => self.swap(true),
                Effect::GamePlayed => {
                    println!(
                        "Slots: bet {}, reels {:?}, result {}{}.",
                        self.machine.bet,
                        self.machine.results,
                        self.machine.result,
                        if self.machine.lucky { " (luck)" } else { "" }
                    );
                    *state
                        .misc_stats
                        .entry(slots::GAMES_PLAYED_STAT)
                        .or_insert(0) += 1;
                }
                Effect::Data(d) => *world::casino::data_mut(state, self.casino.form) = d,
                Effect::Broke => {
                    messages.push(super::casino::broke_message(
                        order,
                        world::casino::Game::Slots,
                    ));
                    sounds.extend(order.form_by_editor_id("UIPopUpMessageGeneral"));
                }
                Effect::Close => {
                    let settled = world::casino::settle(
                        order,
                        state,
                        &self.casino,
                        self.machine.chips,
                        self.machine.new_level,
                    );
                    messages.extend(super::casino::settled_message(settled));
                    if let Some(lock) = lock.as_deref_mut() {
                        lock.closed(now as u64);
                    }
                    println!(
                        "Slots closed: {} chips, winnings {} (level {}).",
                        self.machine.chips, self.machine.data.winnings, self.machine.data.level
                    );
                    self.closed = true;
                }
            }
        }
    }

    fn fill(&mut self, ui: &mut ui::Ui, order: &esm::LoadOrder) {
        let earnings =
            world::casino::earnings_line(order, &self.casino, self.machine.data.winnings);
        let status = self.machine.status.as_ref().map(|s| match *s {
            Status::Result { won, amount, lucky } => {
                world::casino::result_line(order, won, amount, lucky)
            }
            Status::PressAnyButton => world::casino::text(
                order,
                "sSlotPressAnyButtonText",
                "Press any valid slot machine button to continue.",
            ),
        });
        self.menu
            .fill(ui, &self.machine, &earnings, status.as_deref());
    }
}

/// Every frame: the clicks, the update, the effects, the tiles. Returns
/// sounds and corner messages.
pub fn after(
    screen: &mut Screen,
    game: &Game,
    state: &mut GameState,
    lock: &mut world::casino::AntiCheat,
    now_ms: f64,
) -> (Vec<FormId>, Vec<crate::hud::HudMessage>) {
    let order = &game.order;
    let mut sounds = Vec::new();
    let mut messages = Vec::new();
    let mut open = std::mem::take(&mut screen.open);
    let top = open.len().saturating_sub(1);
    for (i, m) in open.iter_mut().enumerate() {
        let OpenMenu::Slots(s) = m else {
            continue;
        };
        if s.closed {
            continue;
        }
        // Its clock stops while something is over it.
        s.machine.on_top = i == top;
        let mut dice = |n: usize| (state.roll() % n.max(1) as u64) as usize;
        let mut fx = Vec::new();
        for id in std::mem::take(&mut s.menu.clicks) {
            fx.extend(s.machine.click(id, &mut dice));
        }
        let models = s.models.clone();
        fx.extend(s.machine.update(now_ms as u32, &Clips(&models)));
        s.apply(
            fx,
            order,
            state,
            now_ms / 1000.0,
            &mut sounds,
            &mut messages,
            Some(lock),
        );
        s.fill(&mut screen.ui, order);
    }
    screen.open = open;
    (sounds, messages)
}
