//! Blackjack (`ShowBlackJackMenuParams`, `menus::Menu::Casino` with
//! `Game::Blackjack`): the game runs in `world::casino::blackjack`, its
//! tiles in `ui::menus::blackjack`, its 3D in `cellview::blackjack` (drawn
//! by `crate::casino_scene`). Here the game's effects are carried out:
//! sounds, the models posed, the cards' faces and backs, the hidden cards,
//! the split hand and the arrows, the chip stacks, the player's line for
//! the casino, the chips settled as the menu closes.
//!
//! `Create`'s checks are made where the script asks (`scripts`, with
//! `casino::create`).

use std::collections::HashMap;
use std::sync::Arc;

use cellview::blackjack::{index, BlackjackModels};
use cellview::caravan::Pose;
use cellview::Game;
use esm::FormId;
use ui::menus::blackjack::{BlackjackMenu, FILE, MAX_HAND_TEXT};
use world::casino::blackjack::{self, Blackjack, Effect, Hand, Model, Status, MAX_CARDS};
use world::casino::Casino;
use world::dialogue::PLAYER_REF;
use world::scripting::GameState;

use super::{OpenMenu, Screen};

/// The table as the menu has it: each model's pose, the textures put on
/// shapes.
#[derive(Default)]
pub struct Table {
    pub poses: Vec<Pose>,
    /// Textures put on shapes: (model index, shape lower case) → the path
    /// under `Data\`.
    pub textures: HashMap<(usize, String), String>,
    /// Bumped whenever a texture changes.
    pub texture_changes: u64,
}

/// Blackjack on screen.
pub struct BlackjackScreen {
    pub menu: BlackjackMenu,
    pub game: Blackjack,
    pub casino: Casino,
    pub models: Arc<BlackjackModels>,
    pub table: Table,
    /// Each deck's back and faces (`deck_textures`).
    decks: Vec<Option<(String, Vec<String>)>>,
    pub closed: bool,
}

/// Whether this module shows a request.
pub fn takes(m: &crate::menus::Menu) -> bool {
    matches!(
        m,
        crate::menus::Menu::Casino {
            game: world::casino::Game::Blackjack,
            ..
        }
    )
}

/// The sequences' lengths, from the models.
struct Clips<'a>(&'a BlackjackModels);

impl blackjack::Clips for Clips<'_> {
    fn end(&self, model: Model, sequence: &str) -> Option<f32> {
        self.0
            .models
            .get(index(model))
            .and_then(|m| m.sequence(sequence))
            .map(|s| s.stop - s.start)
    }
}

/// The arrows' shapes on the table (`HighlightSelectedDeck` `007399f0`).
const ARROWS: [&str; 2] = ["hand1_arrow:0", "hand1_arrow:0@#2"];

/// Opens the menu (`BlackJackMenu::Create` after its checks); returns the
/// sounds it starts with.
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
    let models = cellview::blackjack::load(
        &game.assets,
        &casino.models[world::casino::model::BLACKJACK_TABLE],
        &casino.models[world::casino::model::CHIPS..world::casino::model::CHIPS + 6],
    );
    let Some(models) = models else {
        println!("The blackjack table can't be read.");
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
    let (bj, fx) = Blackjack::open(
        casino.blackjack_payout,
        casino.shuffle_percent,
        casino.decks,
        casino.dealer_hole_card,
        casino.max_winnings,
        min_bet,
        max_bet,
        chips,
        luck,
        data,
    );
    let mut code = BlackjackMenu::new(0);
    let tile = match screen.load(game, FILE, &mut code) {
        Ok(t) => t,
        Err(e) => {
            println!("MENUS: BlackJack Menu Creation Failed... ({e})");
            return Vec::new();
        }
    };
    code.menu = tile;
    println!(
        "Blackjack at {} ({}): bets {min_bet} to {max_bet}, {chips} chips, Luck {luck}, {} deck(s), payout {}.",
        casino.name, casino.form, bj.decks, bj.payout
    );
    let decks = (0..bj.decks)
        .map(|d| blackjack::deck_textures(&casino.textures[world::casino::texture::DECKS + d]))
        .collect();
    let models = Arc::new(models);
    let mut s = BlackjackScreen {
        menu: code,
        game: bj,
        casino,
        table: Table {
            poses: vec![Pose::default(); models.models.len()],
            ..Table::default()
        },
        models,
        decks,
        closed: false,
    };
    // `Deck:0` takes the first deck's back (`00732540`).
    if let Some(Some((back, _))) = s.decks.first() {
        s.table.textures.insert((0, "deck:0".into()), back.clone());
        s.table.texture_changes += 1;
    }
    s.menu.open(&mut screen.ui);
    let mut sounds = Vec::new();
    let mut messages = Vec::new();
    s.apply(fx, order, state, 0.0, &mut sounds, &mut messages, None);
    s.fill(&mut screen.ui, order);
    screen.open.push(OpenMenu::Blackjack(Box::new(s)));
    sounds
}

impl BlackjackScreen {
    /// A card's textures: its deck's face and back.
    fn card_textures(&self, slot: usize) -> Option<(String, String)> {
        let card = self.game.shoe.get(slot)?;
        if card.is_empty() {
            return None;
        }
        let (back, faces) = self.decks.get(usize::from(card.deck))?.as_ref()?;
        Some((faces.get(usize::from(card.face))?.clone(), back.clone()))
    }

    fn cull(&mut self, model: Model, shape: &str, hidden: bool) {
        let pose = &mut self.table.poses[index(model)];
        let shape = shape.to_ascii_lowercase();
        if hidden {
            pose.culled.insert(shape);
        } else {
            pose.culled.remove(&shape);
        }
    }

    /// Carries out the game's effects; `lock` stamps the anti-cheat lock
    /// as it closes (`now` seconds).
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
        let models = self.models.clone();
        for e in fx {
            match e {
                Effect::Sound(name) => sounds.extend(order.form_by_editor_id(name)),
                Effect::Activate { model, sequence } => {
                    self.table.poses[index(model)].activate(&sequence)
                }
                Effect::Deactivate { model, sequence } => {
                    self.table.poses[index(model)].deactivate(&sequence)
                }
                Effect::DeactivateAll(model) => self.table.poses[index(model)].deactivate_all(),
                Effect::Update { model, time } => {
                    let i = index(model);
                    self.table.poses[i].update(&models.models[i], time);
                }
                Effect::UpdateAll(time) => {
                    for (pose, model) in self.table.poses.iter_mut().zip(&models.models) {
                        pose.update(model, time);
                    }
                }
                Effect::SwapCard {
                    hand,
                    index: i,
                    card,
                } => {
                    if let Some((face, back)) = self.card_textures(card) {
                        let (face_shape, back_shape) = hand.card_shapes(i);
                        let m = index(hand.model());
                        let t = &mut self.table.textures;
                        t.insert((m, face_shape.to_ascii_lowercase()), face);
                        t.insert((m, back_shape.to_ascii_lowercase()), back);
                        self.table.texture_changes += 1;
                    }
                }
                Effect::HideUndealt(next) => {
                    for (h, hand) in [Hand::Player, Hand::Split, Hand::Dealer].iter().enumerate() {
                        for i in 1..=MAX_CARDS {
                            let hidden = i >= next[h];
                            let (face, back) = hand.card_shapes(i);
                            for shape in [face, back, hand.shadow_shape(i)] {
                                self.cull(hand.model(), &shape, hidden);
                            }
                        }
                    }
                }
                Effect::ShowAllCards => {
                    for hand in [Hand::Player, Hand::Split, Hand::Dealer] {
                        for i in 1..=MAX_CARDS {
                            let (face, back) = hand.card_shapes(i);
                            for shape in [face, back, hand.shadow_shape(i)] {
                                self.cull(hand.model(), &shape, false);
                            }
                        }
                    }
                }
                Effect::SideHand(on) => {
                    let i = index(Model::Split);
                    let model = &models.models[i];
                    let z = if on { 0.0 } else { -5.0 };
                    self.table.poses[i].set_translation(model, &model.top, [0.0, 0.0, z]);
                }
                Effect::Highlight(hand) => {
                    let (main, split) = match hand {
                        Some(Hand::Split) => (false, true),
                        Some(_) => (true, false),
                        None => (false, false),
                    };
                    self.cull(Model::Table, ARROWS[0], !main);
                    self.cull(Model::Table, ARROWS[1], !split);
                }
                Effect::BetChips(bet) => {
                    let (z, hide) = blackjack::bet_chips(bet);
                    for i in 0..6 {
                        for (model, h) in [(Model::Chip(i), z[i]), (Model::Shadow(i), 0.0)] {
                            let h = if matches!(model, Model::Shadow(_)) && hide[i] {
                                -5.0
                            } else {
                                h
                            };
                            let k = index(model);
                            let m = &models.models[k];
                            self.table.poses[k].set_translation(m, &m.top, [0.0, 0.0, h]);
                        }
                    }
                }
                Effect::GamePlayed => {
                    println!("Blackjack: results {:?}.", self.game.results);
                    *state
                        .misc_stats
                        .entry(blackjack::GAMES_PLAYED_STAT)
                        .or_insert(0) += 1;
                }
                Effect::Data(d) => *world::casino::data_mut(state, self.casino.form) = d,
                Effect::Broke => {
                    messages.push(super::casino::broke_message(
                        order,
                        world::casino::Game::Blackjack,
                    ));
                    sounds.extend(order.form_by_editor_id("UIPopUpMessageGeneral"));
                }
                Effect::Close => {
                    let settled = world::casino::settle(
                        order,
                        state,
                        &self.casino,
                        self.game.chips,
                        self.game.new_level,
                    );
                    messages.extend(super::casino::settled_message(settled));
                    if let Some(lock) = lock.as_deref_mut() {
                        lock.closed(now as u64);
                    }
                    println!(
                        "Blackjack closed: {} chips, winnings {} (level {}).",
                        self.game.chips, self.game.data.winnings, self.game.data.level
                    );
                    self.closed = true;
                }
            }
        }
    }

    fn fill(&mut self, ui: &mut ui::Ui, order: &esm::LoadOrder) {
        let earnings = world::casino::earnings_line(order, &self.casino, self.game.data.winnings);
        let text = |name: &str, default: &str| world::casino::text(order, name, default);
        let status = self.game.status.as_ref().map(|s| match *s {
            Status::Won { amount, lucky } => world::casino::result_line(order, true, amount, lucky),
            Status::Lost { amount, unlucky } => {
                world::casino::result_line(order, false, amount, unlucky)
            }
            Status::Even => text("sYouBreakEvenText", "You break even"),
            Status::MaxHand => MAX_HAND_TEXT.to_string(),
            Status::Empty => String::new(),
        });
        self.menu.fill(ui, &self.game, &earnings, status.as_deref());
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
        let OpenMenu::Blackjack(s) = m else {
            continue;
        };
        if s.closed {
            continue;
        }
        // Its clock stops while something is over it.
        s.game.on_top = i == top;
        let mut dice = |n: usize| (state.roll() % n.max(1) as u64) as usize;
        let mut fx = Vec::new();
        for id in std::mem::take(&mut s.menu.clicks) {
            fx.extend(s.game.click(id, &mut dice));
        }
        let models = s.models.clone();
        fx.extend(s.game.update(now_ms as u32, &Clips(&models), &mut dice));
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
