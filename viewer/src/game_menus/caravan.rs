//! Caravan (`ShowCaravanMenu`, `menus::Menu::Caravan`): the game runs in
//! `world::caravan::menu::Menu`, its tiles in `ui::menus::caravan`, its
//! table in `cellview::caravan` (drawn by `crate::caravan_table`). Here the
//! menu's effects are carried out: sounds, the table's models posed and
//! textured, the "How many?" box for a raise, the quit box, the player's
//! card lists written back, the stake paid, the tutorial messages
//! (`HelpCaravanBetting`…) shown once each (`tutorial::show_once`; the menu
//! waits, `+0xe78` / `+0xe7c`, until it's back on top).

use std::collections::HashMap;
use std::sync::Arc;

use cellview::caravan::{CaravanModels, Part, Pose};
use cellview::Game;
use esm::{FormId, FourCC, LoadOrder};
use nif::math::Transform;
use ui::menus::caravan::{CaravanMenu, Facts, FILE};
use ui::menus::message::MessageBox;
use world::caravan::menu::{Effect, Menu, Model};
use world::caravan::{bet, Card};
use world::dialogue::PLAYER_REF;
use world::scripting::GameState;

use super::{OpenMenu, Screen};

/// The quit box's owner (the menu's class number).
pub const FORFEIT_OWNER: u32 = ui::menus::caravan::CLASS as u32;

/// A bill or coin on the table: its kind's model, its spot, its pose.
pub struct Money {
    pub part: Part,
    pub pose: Pose,
}

/// The table as the menu has it: each model's pose, the money, which
/// models are in, the card textures put on shapes.
#[derive(Default)]
pub struct Table {
    pub poses: HashMap<Part, Pose>,
    pub money: Vec<Money>,
    /// The deck screen's models in, the game's in.
    pub deck_models: bool,
    pub game_models: bool,
    /// Textures put on shapes: (model, shape lower case) → the path under
    /// `Textures\`.
    pub textures: HashMap<(Part, String), String>,
    /// Bumped whenever a texture changes.
    pub texture_changes: u64,
    /// The hand card raised (`pSelectedObject`).
    lifted: Option<usize>,
}

/// The Caravan menu on screen.
pub struct CaravanScreen {
    pub menu: CaravanMenu,
    pub game: Menu,
    pub models: Arc<CaravanModels>,
    pub table: Table,
    pub facts: Facts,
    /// The "How many?" box is ours.
    asked: bool,
    /// The tutorial the menu waits on (`+0xe78`, its id `+0xe7c`).
    tutorial: Option<u8>,
    pub closed: bool,
}

/// Whether this module shows a request.
pub fn takes(m: &crate::menus::Menu) -> bool {
    matches!(m, crate::menus::Menu::Caravan { .. })
}

/// A card's face (`TX00`) and back (`TX01`) textures.
pub fn card_textures(order: &LoadOrder, form: FormId) -> (String, String) {
    let record = order.get(form).and_then(|r| r.record().ok());
    let tex = |sig: &[u8; 4]| {
        record
            .as_ref()
            .and_then(|r| r.get(FourCC::new(sig)).map(|s| s.zstring()))
            .unwrap_or_default()
    };
    (tex(b"TX00"), tex(b"TX01"))
}

/// The sequences' lengths, from the models.
struct Clips<'a>(&'a CaravanModels);

impl world::caravan::menu::Clips for Clips<'_> {
    fn end(&self, model: Model, sequence: &str) -> f32 {
        let part = match model {
            Model::Table => Part::Table,
            Model::Deck => Part::Deck,
            Model::Available => Part::Available,
            Model::Hand { npc } => Part::Hand { npc },
            Model::Row { track, row } => Part::Row { track, row },
            // The money's own clocks aren't asked about (state 4 counts a
            // second).
            Model::Bill(_) | Model::Coin(_) => return 0.0,
        };
        self.0
            .model(part)
            .and_then(|m| m.sequence(sequence))
            .map_or(0.0, |s| s.stop - s.start)
    }
}

fn record_facts(state: &GameState, npc_name: String) -> Facts {
    let c = &state.caravan;
    Facts {
        npc_name,
        cap_winnings: c.cap_winnings,
        cap_losses: c.cap_losses,
        winnings: c.winnings,
        losses: c.losses,
        largest_winning: c.largest_winning,
    }
}

/// Opens the menu for a request (`CaravanMenu::Create`).
pub fn open(screen: &mut Screen, game: &Game, state: &mut GameState, request: crate::menus::Menu) {
    let crate::menus::Menu::Caravan {
        npc,
        deck,
        difficulty,
        share,
    } = request
    else {
        return;
    };
    let order = &game.order;
    let Some(models) = cellview::caravan::load(&game.assets) else {
        println!("The Caravan table can't be read.");
        return;
    };
    let cards = |list: &[FormId]| -> Vec<Card> {
        list.iter()
            .filter_map(|&f| world::caravan::card(order, f))
            .collect()
    };
    let out = cards(&state.caravan.inactive);
    let in_deck = cards(&state.caravan.active);
    let npc_deck = world::caravan::deck(order, deck);
    let player_funds = bet::funds(order, state, PLAYER_REF);
    let npc_funds = bet::funds(order, state, npc);
    let barter = bet::player_barter(order, state);
    let mut dice = |n: usize| (state.roll() % n.max(1) as u64) as usize;
    let (menu, fx) = Menu::open(
        npc,
        npc_deck,
        difficulty,
        &out,
        &in_deck,
        bet::Bet::new(player_funds, npc_funds, share),
        barter,
        &mut dice,
    );
    let mut code = CaravanMenu::new(0);
    let tile = match screen.load(game, FILE, &mut code) {
        Ok(t) => t,
        Err(e) => {
            println!("MENUS: Caravan Menu Creation Failed... ({e})");
            return;
        }
    };
    code.menu = tile;
    let name = world::items::item_info(order, world::scripting::base_of(order, npc).unwrap_or(npc))
        .map(|i| i.name)
        .unwrap_or_default();
    println!(
        "Caravan against {npc} ({name}): deck {deck}, difficulty {difficulty}, betting {share}; funds {player_funds} and {npc_funds}; {} models.",
        models.parts.len()
    );
    let mut s = CaravanScreen {
        menu: code,
        game: menu,
        models: Arc::new(models),
        table: Table::default(),
        facts: record_facts(state, name),
        asked: false,
        tutorial: None,
        closed: false,
    };
    s.menu.open(&mut screen.ui);
    let mut sounds = Vec::new();
    let mut prompts = Vec::new();
    s.apply(fx, order, state, &mut sounds, &mut prompts);
    s.menu.fill(&mut screen.ui, &s.game, &s.facts, None);
    screen.open.push(OpenMenu::Caravan(Box::new(s)));
    // The betting tutorial (`Create`, after the menu is shown).
    for p in prompts {
        if let Prompt::Tutorial(name) = p {
            tutorial(screen, game, state, name);
        }
    }
}

/// A box the menu asks for.
pub enum Prompt {
    HowMany(i32),
    Forfeit,
    /// A tutorial message by editor ID.
    Tutorial(&'static str),
}

/// A tutorial the menu asks for (`Create` `00741060`, `DoIdle`
/// `00741500`): shown if it hasn't been, the menu waiting until it's back
/// on top; else the menu carries on.
fn tutorial(screen: &mut Screen, game: &Game, state: &mut GameState, name: &'static str) {
    let id = world::caravan::menu::tutorial_id(name);
    let wait = id.is_some_and(|id| super::tutorial::show_once(screen, game, state, id, name));
    for m in screen.open.iter_mut() {
        if let OpenMenu::Caravan(c) = m {
            if wait {
                c.tutorial = id;
            } else {
                c.game.tutorial_done();
            }
        }
    }
}

impl CaravanScreen {
    fn part(&self, model: Model) -> Option<Part> {
        Some(match model {
            Model::Table => Part::Table,
            Model::Deck => Part::Deck,
            Model::Available => Part::Available,
            Model::Hand { npc } => Part::Hand { npc },
            Model::Row { track, row } => Part::Row { track, row },
            Model::Bill(_) | Model::Coin(_) => return None,
        })
    }

    /// A model's pose (with its model data).
    fn pose(&mut self, part: Part) -> Option<(&cellview::caravan::ModelData, &mut Pose)> {
        let data = self.models.model(part)?;
        Some((data, self.table.poses.entry(part).or_default()))
    }

    fn texture(&mut self, part: Part, shape: &str, path: String) {
        self.table
            .textures
            .insert((part, shape.to_ascii_lowercase()), path);
        self.table.texture_changes += 1;
    }

    fn cull(&mut self, part: Part, node: &str) {
        self.table
            .poses
            .entry(part)
            .or_default()
            .culled
            .insert(node.to_ascii_lowercase());
    }

    /// Carries out the menu's effects.
    fn apply(
        &mut self,
        fx: Vec<Effect>,
        order: &LoadOrder,
        state: &mut GameState,
        sounds: &mut Vec<FormId>,
        prompts: &mut Vec<Prompt>,
    ) {
        let models = self.models.clone();
        for e in fx {
            match e {
                Effect::Sound(name) => sounds.extend(order.form_by_editor_id(name)),
                Effect::Tutorial(name) => prompts.push(Prompt::Tutorial(name)),
                Effect::Activate { model, sequence } => match model {
                    Model::Bill(i) | Model::Coin(i) => {
                        if let Some(m) = self.money_mut(model, i) {
                            m.pose.activate(&sequence);
                        }
                    }
                    _ => {
                        if let Some((_, pose)) = self.part(model).and_then(|p| self.pose(p)) {
                            pose.activate(&sequence);
                        }
                    }
                },
                Effect::Deactivate { model, sequence } => {
                    if let Some((_, pose)) = self.part(model).and_then(|p| self.pose(p)) {
                        pose.deactivate(&sequence);
                    }
                }
                Effect::DeactivateAll(model) => {
                    if let Some((_, pose)) = self.part(model).and_then(|p| self.pose(p)) {
                        pose.deactivate_all();
                    }
                }
                Effect::Update { model, time } => match model {
                    Model::Bill(i) | Model::Coin(i) => {
                        let Some(m) = self.money_mut(model, i) else {
                            continue;
                        };
                        let part = m.part;
                        if let Some(data) = models.model(part) {
                            if let Some(m) = self.money_mut(model, i) {
                                m.pose.update(data, time);
                            }
                        }
                    }
                    _ => {
                        if let Some((data, pose)) = self.part(model).and_then(|p| self.pose(p)) {
                            pose.update(data, time);
                        }
                    }
                },
                Effect::ResetPose { model, sequence } => {
                    if let Some((data, pose)) = self.part(model).and_then(|p| self.pose(p)) {
                        pose.reset(data, &sequence);
                    }
                }
                Effect::LoadDeckModels => self.table.deck_models = true,
                Effect::UnloadDeckModels => self.table.deck_models = false,
                Effect::LoadGameModels => {
                    self.table.game_models = true;
                    // The markers start under the table (`0073f4db`: z −5).
                    for valid in [true, false] {
                        self.set_root(Part::Cursor { valid }, None, [0.0, 0.0, -5.0]);
                    }
                }
                Effect::DeckTextures { around } => self.deck_textures(order, around, false),
                Effect::FlipSelected => {
                    let around = self.game.chosen as i32;
                    self.deck_textures(order, around, true);
                }
                Effect::HandTextures { npc, setup } => self.hand_textures(order, npc, setup),
                Effect::DealCard { npc } => self.deal_card(order, npc),
                Effect::TrackCard { track, row, column } => {
                    let card = self
                        .game
                        .game
                        .as_ref()
                        .and_then(|g| g.tracks[track][row].get(column).copied());
                    if let Some(card) = card {
                        self.row_card(order, track, row, column, card);
                    }
                }
                Effect::PlacedCard {
                    track,
                    row,
                    column,
                    card,
                } => self.row_card(order, track, row, column, card),
                Effect::LiftHandCard(card) => self.lift_hand_card(card),
                Effect::Cursor {
                    track,
                    row,
                    column,
                    valid,
                    card,
                } => {
                    let spot = models
                        .grids
                        .get(track)
                        .and_then(|g| g.get(&(row, column)))
                        .copied();
                    if let Some(spot) = spot {
                        let turn = cellview::caravan::quarter_turn();
                        self.set_root(
                            Part::Cursor { valid },
                            Some(nif::math::mat_mul(&spot.rotation, &turn)),
                            spot.center,
                        );
                    }
                    self.set_root(Part::Cursor { valid: !valid }, None, [0.0, 0.0, -50.0]);
                    let shape = if valid { "Add:3" } else { "Remove:3" };
                    let (face, _) = card_textures(order, card.form);
                    self.texture(Part::Cursor { valid }, shape, face);
                }
                Effect::HideCursors => {
                    for valid in [true, false] {
                        self.set_root(Part::Cursor { valid }, None, [0.0, 0.0, -100.0]);
                    }
                }
                Effect::LiftTrack(t) | Effect::LowerTrack(t) => {
                    let z = if matches!(e, Effect::LiftTrack(_)) {
                        3.0
                    } else {
                        0.0
                    };
                    for row in 0..world::caravan::ROWS {
                        self.set_root(Part::Row { track: t, row }, None, [0.0, 0.0, z]);
                    }
                }
                Effect::CullDrawPile { npc } => {
                    let side = if npc { "Opponent" } else { "Player" };
                    let part = Part::Hand { npc };
                    let names: [String; 4] = if npc {
                        [
                            format!("{side}-Deck_06:5"),
                            format!("{side}-Deck_Back:0"),
                            format!("{side}-Deck_06_Shadow:0"),
                            format!("{side}-Deck_Back_Shadow:0"),
                        ]
                    } else {
                        // `00747d30`: `Player-Deck_06:5` twice, its shadow
                        // left.
                        [
                            format!("{side}-Deck_06_Back:0"),
                            format!("{side}-Deck_06:5"),
                            format!("{side}-Deck_Back:0"),
                            format!("{side}-Deck_Back_Shadow:0"),
                        ]
                    };
                    for n in names {
                        self.cull(part, &n);
                    }
                }
                Effect::Money => self.new_money(),
                Effect::HowMany { max } => prompts.push(Prompt::HowMany(max)),
                Effect::ConfirmForfeit => prompts.push(Prompt::Forfeit),
                Effect::SaveDeck { out, deck } => {
                    state.caravan.inactive = out;
                    state.caravan.active = deck;
                }
                Effect::Settle { won, stake } => {
                    bet::settle(order, state, self.game.npc, stake, won);
                    self.facts = record_facts(state, self.facts.npc_name.clone());
                    println!("Caravan: {} for {stake}.", if won { "won" } else { "lost" });
                }
                Effect::Close => self.closed = true,
            }
        }
    }

    fn money_mut(&mut self, model: Model, i: usize) -> Option<&mut Money> {
        let bill = matches!(model, Model::Bill(_));
        self.table
            .money
            .iter_mut()
            .filter(|m| matches!(m.part, Part::Bill(_)) == bill)
            .nth(i)
    }

    /// New pieces from the menu's money (`LoadNewBill`, `LoadNewCoin`):
    /// each starts its landing.
    fn new_money(&mut self) {
        let bills = self
            .table
            .money
            .iter()
            .filter(|m| matches!(m.part, Part::Bill(_)))
            .count();
        let coins = self.table.money.len() - bills;
        let new_bills: Vec<_> = self.game.money.bills[bills..].to_vec();
        let new_coins: Vec<_> = self.game.money.coins[coins..].to_vec();
        for (bill, pieces) in [(true, new_bills), (false, new_coins)] {
            for p in pieces {
                let part = if bill {
                    Part::Bill(p.kind)
                } else {
                    Part::Coin(p.kind)
                };
                let mut pose = Pose::default();
                pose.activate(p.sequence);
                // On its spot (`0074a2c0`): the spot shape's own turn, its
                // bound's centre.
                let spots = if bill {
                    &self.models.bill_spots
                } else {
                    &self.models.coin_spots
                };
                if let (Some(spot), Some(data)) = (
                    p.spot.and_then(|s| spots.get(s)).copied(),
                    self.models.model(part),
                ) {
                    pose.nodes.insert(
                        data.top.clone(),
                        Transform {
                            rotation: spot.rotation,
                            translation: spot.center,
                            scale: 1.0,
                        },
                    );
                }
                self.table.money.push(Money { part, pose });
            }
        }
    }

    /// A model's top node turned and moved (the markers, a track lifted).
    fn set_root(&mut self, part: Part, rotation: Option<[[f32; 3]; 3]>, t: [f32; 3]) {
        let Some((data, pose)) = self.pose(part) else {
            return;
        };
        let top = data.top.clone();
        let mut own = pose
            .nodes
            .get(&top)
            .or_else(|| data.nodes.get(&top))
            .copied()
            .unwrap_or(Transform::IDENTITY);
        own.translation = t;
        if let Some(r) = rotation {
            own.rotation = r;
        }
        pose.nodes.insert(top, own);
    }

    /// `SwapDeckBuildingTextures` (`0074b700`): the 24 slots show the cards
    /// from `around` − 11, the middle one (`_12`) the chosen; out of range a
    /// slot's shapes go to z −50 (the shadows of slots 11 and 12 keep their
    /// x and y), in range back to 0 with the card face up in the row it
    /// belongs to and face down in the other. `only_middle`:
    /// `FlipSelectedCardTextures` (slot 11 alone).
    fn deck_textures(&mut self, order: &LoadOrder, around: i32, only_middle: bool) {
        let cards = self.game.cards.clone();
        let slots: Vec<usize> = if only_middle {
            vec![11]
        } else {
            (0..24).collect()
        };
        for i in slots {
            let index = around - 11 + i as i32;
            let n = i + 1;
            let names = |kind: &str| {
                (
                    format!("{kind}_{n:02}:{i}"),
                    format!("{kind}_{n:02}_Back:0"),
                    format!("{kind}_{n:02}_Shadow:0"),
                )
            };
            let shown = usize::try_from(index)
                .ok()
                .and_then(|k| cards.get(k))
                .copied();
            let z = if shown.is_some() { 0.0 } else { -50.0 };
            for (part, kind) in [(Part::Deck, "Deck"), (Part::Available, "Available")] {
                let (face, back, shadow) = names(kind);
                let Some((data, pose)) = self.pose(part) else {
                    continue;
                };
                for shape in [&face, &back] {
                    pose.set_translation(data, shape, [0.0, 0.0, z]);
                }
                let keep = if i == 11 || i == 12 {
                    let at = pose.translation(data, &shadow);
                    [at[0], at[1], z]
                } else {
                    [0.0, 0.0, z]
                };
                pose.set_translation(data, &shadow, keep);
                if let Some(c) = shown {
                    let (front_tex, back_tex) = card_textures(order, c.card.form);
                    let up = c.in_deck == (part == Part::Deck);
                    let (on_face, on_back) = if up {
                        (front_tex, back_tex)
                    } else {
                        (back_tex, front_tex)
                    };
                    self.texture(part, &face, on_face);
                    self.texture(part, &back, on_back);
                }
            }
        }
    }

    /// `UpdateGameHandTextures` (`0074d730`).
    fn hand_textures(&mut self, order: &LoadOrder, npc: bool, setup: bool) {
        let Some(g) = self.game.game.clone() else {
            return;
        };
        let part = Part::Hand { npc };
        let side = if npc { "Opponent" } else { "Player" };
        let hand = if npc { &g.npc_hand } else { &g.player_hand };
        for i in 0..8 {
            let shape = format!("{side}-Deck_0{}:{i}", i + 1);
            let shadow = format!("{side}-Deck_0{}_Shadow:0", i + 1);
            if let Some(c) = hand.get(i) {
                let (face, back) = card_textures(order, c.form);
                self.texture(part, &shape, if npc { back } else { face });
            } else if (setup && i != 5) || (!setup && hand.len() < 5) {
                self.cull(part, &shape);
                self.cull(part, &shadow);
            }
        }
        if !setup {
            let (deck, next) = if npc {
                (&g.npc_deck, g.next_npc)
            } else {
                (&g.player_deck, g.next_player)
            };
            if let Some(c) = next.and_then(|n| deck.get(n)) {
                let (_, back) = card_textures(order, c.form);
                self.texture(part, &format!("{side}-Deck_06:5"), back);
            }
        }
    }

    /// `UpdateDealCardTexture` (`0074d190`).
    fn deal_card(&mut self, order: &LoadOrder, npc: bool) {
        let Some(g) = self.game.game.clone() else {
            return;
        };
        let part = Part::Hand { npc };
        let side = if npc { "Opponent" } else { "Player" };
        let hand = if npc { &g.npc_hand } else { &g.player_hand };
        if let Some(c) = hand.get(4) {
            let (face, back) = card_textures(order, c.form);
            self.texture(
                part,
                &format!("{side}-Deck_06:5"),
                if npc { back } else { face },
            );
        }
        let (deck, next) = if npc {
            (&g.npc_deck, g.next_npc)
        } else {
            (&g.player_deck, g.next_player)
        };
        match next.and_then(|n| deck.get(n)) {
            Some(c) if !deck.is_empty() => {
                let (_, back) = card_textures(order, c.form);
                self.texture(part, &format!("{side}-Deck_Back:0"), back);
            }
            _ => self.cull(part, &format!("{side}-Deck_Back:0")),
        }
    }

    /// `UpdateTrackCardTexture` (`0074e220`): a row's card at a column
    /// (`Card_0{n}:{column}`: n the column + 1 on the player's tracks, 4 −
    /// the column on the opponent's).
    fn row_card(&mut self, order: &LoadOrder, track: usize, row: usize, column: usize, card: Card) {
        let n = if track < 3 {
            column + 1
        } else {
            4 - column.min(3)
        };
        let (face, _) = card_textures(order, card.form);
        self.texture(
            Part::Row { track, row },
            &format!("Card_0{n}:{column}"),
            face,
        );
    }

    /// `SelectDeckCard` (`0074e480`): the raised card put down; a new one
    /// raised (its shape and shadow 1 up).
    fn lift_hand_card(&mut self, card: Option<usize>) {
        let part = Part::Hand { npc: false };
        if let Some(old) = self.table.lifted.take() {
            let shape = format!("Player-Deck_0{}:{old}", old + 1);
            let shadow = format!("Player-Deck_0{}_Shadow:0", old + 1);
            if let Some((data, pose)) = self.pose(part) {
                let at = pose.translation(data, &shape);
                if at[2] > 0.0 {
                    pose.set_translation(data, &shape, [0.0, at[1], 0.0]);
                    pose.set_translation(data, &shadow, [0.0, at[1], 0.0]);
                }
            }
        }
        let Some(sel) = card else {
            return;
        };
        let shape = format!("Player-Deck_0{}:{sel}", sel + 1);
        let shadow = format!("Player-Deck_0{}_Shadow:0", sel + 1);
        if let Some((data, pose)) = self.pose(part) {
            let at = pose.translation(data, &shape);
            pose.set_translation(data, &shape, [0.0, at[1], 1.0]);
            pose.set_translation(data, &shadow, [0.0, at[1], 1.0]);
        }
        self.table.lifted = Some(sel);
    }
}

/// Every frame: input passed on, the prompts' answers, the update, the
/// effects, the tiles. Returns sounds.
/// What Caravan reads of the mouse and pad each update.
#[derive(Debug, Clone, Copy, Default)]
pub struct PadInput {
    /// A pad in use (`004b71d0`).
    pub pad: bool,
    /// Its left stick (`world::caravan::menu::stick_axis`; y down).
    pub stick: [f32; 2],
    /// A mouse button held (`InterfaceManager::fMouseHeldTime` above 0).
    pub mouse_held: bool,
}

pub fn after(
    screen: &mut Screen,
    game: &Game,
    state: &mut GameState,
    now_ms: f64,
    input: PadInput,
) -> Vec<FormId> {
    let order = &game.order;
    let mut sounds = Vec::new();
    // Answers from the boxes above the menu.
    let mut how_many = None;
    let mut forfeit = None;
    for m in screen.open.iter_mut() {
        match m {
            OpenMenu::Quantity(q) => {
                if let Some(n) = q.answer.take() {
                    how_many = Some(n);
                }
                for name in q.sounds.drain(..) {
                    sounds.extend(order.form_by_editor_id(&name));
                }
            }
            OpenMenu::Message(msg) => {
                if let Some(n) = msg.take_pressed_for(Some(FORFEIT_OWNER)) {
                    forfeit = Some(n == 0);
                }
            }
            _ => {}
        }
    }
    let mut prompts = Vec::new();
    let on_top = matches!(screen.open.last(), Some(OpenMenu::Caravan(_)));
    for m in screen.open.iter_mut() {
        let OpenMenu::Caravan(c) = m else {
            continue;
        };
        // Waiting on a tutorial: done once the menu is back on top and the
        // message shown (`DoIdle`'s start).
        if let Some(id) = c.tutorial {
            if on_top && state.tutorials.is_shown(id) {
                c.tutorial = None;
                c.game.tutorial_done();
            }
        }
        let mut dice = |n: usize| (state.roll() % n.max(1) as u64) as usize;
        let mut fx = Vec::new();
        if let (Some(n), true) = (how_many, c.asked) {
            c.asked = false;
            fx.extend(c.game.how_many(n, &mut dice));
        }
        if let Some(yes) = forfeit {
            c.game.forfeit_answer(yes);
        }
        for ch in std::mem::take(&mut c.menu.typed) {
            fx.extend(c.game.key(ch, &mut dice));
        }
        for a in std::mem::take(&mut c.menu.arrows) {
            c.game.arrow_key(a);
        }
        for tile in std::mem::take(&mut c.menu.clicks) {
            fx.extend(c.game.click(tile, &mut dice));
        }
        // The pad's stick, and the mouse on the deck screen's scrollbar:
        // a button held starts following a drag from the bar's value, and
        // let go the cards move as far as it moved (`DoIdle`,
        // `InterfaceManager::fMouseHeldTime`).
        c.game.pad = input.pad;
        c.game.stick = input.stick;
        if let Some(bar) = c.menu.meter(&mut screen.ui) {
            let before = c.game.chosen;
            c.game.mouse(input.mouse_held, bar);
            if c.game.chosen != before {
                c.menu.meter_taken(c.game.chosen);
            }
        }
        let models = c.models.clone();
        fx.extend(c.game.update(now_ms as u32, &Clips(&models), &mut dice));
        let mut asks = Vec::new();
        c.apply(fx, order, state, &mut sounds, &mut asks);
        for p in &asks {
            if matches!(p, Prompt::HowMany(_)) {
                c.asked = true;
            }
        }
        prompts.extend(asks);
        let track_x = crate::caravan_table::track_x(c);
        c.menu.fill(&mut screen.ui, &c.game, &c.facts, track_x);
    }
    for p in prompts {
        match p {
            Prompt::HowMany(max) => super::container::open_quantity(screen, game, max),
            Prompt::Forfeit => {
                let text = |s: &str| {
                    world::scripting::game_setting_text(order, s)
                        .or_else(|| ui::game::exe_text_setting(s).map(str::to_string))
                        .unwrap_or_default()
                };
                let alpha = super::message::background_alpha(screen);
                let mut b = MessageBox::new(
                    &text("sQuitCaravanText"),
                    None,
                    &[&text("sYes"), &text("sNo")],
                    0,
                    alpha,
                );
                b.owner = Some(FORFEIT_OWNER);
                super::message::show(screen, game, b);
            }
            Prompt::Tutorial(name) => tutorial(screen, game, state, name),
        }
    }
    sounds
}
