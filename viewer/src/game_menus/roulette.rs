//! Roulette (`ShowRouletteMenuParams`, `menus::Menu::Casino` with
//! `Game::Roulette`): the game runs in `world::casino::roulette`, its tiles
//! in `ui::menus::roulette`, its 3D in `cellview::roulette` (drawn by
//! `crate::casino_scene`). Here the game's effects are carried out: sounds,
//! the table's spins, the wheel's ring turned, the chips and the cursor
//! moved, the spot under the cursor and its ring shown, the player's line
//! for the casino, the chips settled as the menu closes. The mouse's
//! movement moves the cursor (`DoIdle` state 4: the game hides its own
//! pointer while the menu is on top).
//!
//! `Create`'s checks are made where the script asks (`scripts`, with
//! `casino::create`).

use std::sync::Arc;

use cellview::caravan::Pose;
use cellview::roulette::{RouletteModels, INVALID_RING, VALID_RING, WHEEL_RING};
use cellview::Game;
use esm::FormId;
use ui::menus::roulette::{RouletteMenu, FILE};
use world::casino::roulette::{self, Effect, Roulette, Status};
use world::casino::Casino;
use world::dialogue::PLAYER_REF;
use world::scripting::GameState;

use super::{OpenMenu, Screen};

/// Roulette on screen.
pub struct RouletteScreen {
    pub menu: RouletteMenu,
    pub game: Roulette,
    pub casino: Casino,
    pub models: Arc<RouletteModels>,
    /// Each model's pose (the table, the spots, the cursor, the chips and
    /// their shadows).
    pub poses: Vec<Pose>,
    pub closed: bool,
}

/// Whether this module shows a request.
pub fn takes(m: &crate::menus::Menu) -> bool {
    matches!(
        m,
        crate::menus::Menu::Casino {
            game: world::casino::Game::Roulette,
            ..
        }
    )
}

/// The table's sequences' lengths.
struct Clips<'a>(&'a RouletteModels);

impl roulette::Clips for Clips<'_> {
    fn end(&self, sequence: &str) -> Option<f32> {
        self.0
            .models
            .first()
            .and_then(|m| m.sequence(sequence))
            .map(|s| s.stop - s.start)
    }
}

/// Every spot's mesh (all hidden but the one under the cursor).
fn spot_faces() -> impl Iterator<Item = String> {
    (0..roulette::SPOTS).map(|i| {
        if i < 144 {
            format!("{}{:02}:0", (b'a' + (i / 24) as u8) as char, i % 24 + 1)
        } else {
            format!("s{:02}:0", i - 143)
        }
    })
}

/// Opens the menu (`RouletteMenu::Create` after its checks); returns the
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
    let models = cellview::roulette::load(
        &game.assets,
        &casino.models[world::casino::model::ROULETTE_TABLE],
        &casino.models[world::casino::model::ROULETTE_CHIP],
    );
    let Some(models) = models else {
        println!("The roulette table can't be read.");
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
    let text = |name: &str, default: &str| world::casino::text(order, name, default);
    let (rl, fx) = Roulette::open(
        casino.max_winnings,
        min_bet,
        max_bet,
        chips,
        luck,
        data,
        roulette::spots(&text),
        models.places.clone(),
    );
    let mut code = RouletteMenu::new(0);
    let tile = match screen.load(game, FILE, &mut code) {
        Ok(t) => t,
        Err(e) => {
            println!("MENUS: Roulette Menu Creation Failed... ({e})");
            return Vec::new();
        }
    };
    code.menu = tile;
    println!(
        "Roulette at {} ({}): bets {min_bet} to {max_bet}, {chips} chips, Luck {luck}.",
        casino.name, casino.form
    );
    let models = Arc::new(models);
    let mut s = RouletteScreen {
        menu: code,
        game: rl,
        casino,
        poses: vec![Pose::default(); models.models.len()],
        models,
        closed: false,
    };
    // Every spot hidden (`PrepareBetTiles`), the invalid ring too.
    for face in spot_faces() {
        s.poses[1].culled.insert(face);
    }
    s.menu.open(&mut screen.ui);
    let mut sounds = Vec::new();
    let mut messages = Vec::new();
    s.apply(fx, order, state, 0.0, &mut sounds, &mut messages, None);
    let models = s.models.clone();
    for (pose, model) in s.poses.iter_mut().zip(&models.models) {
        pose.update(model, 0.0);
    }
    s.fill(&mut screen.ui, order);
    screen.open.push(OpenMenu::Roulette(Box::new(s)));
    sounds
}

impl RouletteScreen {
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
                Effect::Activate(sequence) => self.poses[0].activate(sequence),
                Effect::Deactivate(sequence) => self.poses[0].deactivate(sequence),
                Effect::UpdateTable(time) => self.poses[0].update(&models.models[0], time),
                Effect::WheelTurn(angle) => {
                    let table = &models.models[0];
                    let mut own = self.poses[0]
                        .nodes
                        .get(WHEEL_RING)
                        .or_else(|| table.nodes.get(WHEEL_RING))
                        .copied()
                        .unwrap_or(nif::math::Transform::IDENTITY);
                    // `004a0c90`: rows (c, s, 0), (−s, c, 0), (0, 0, 1).
                    let (s, c) = angle.sin_cos();
                    own.rotation = [[c, s, 0.0], [-s, c, 0.0], [0.0, 0.0, 1.0]];
                    self.poses[0].nodes.insert(WHEEL_RING.to_string(), own);
                }
                Effect::Place { model, at } => {
                    if let (Some(pose), Some(data)) =
                        (self.poses.get_mut(model), models.models.get(model))
                    {
                        pose.set_translation(data, &data.top, at);
                    }
                }
                Effect::Face(face) => {
                    let grid = &mut self.poses[1];
                    for f in spot_faces() {
                        grid.culled.insert(f);
                    }
                    if let Some(f) = face {
                        grid.culled.remove(&f.to_ascii_lowercase());
                    }
                }
                Effect::Marker { valid } => {
                    let cursor = &mut self.poses[2];
                    let (show, hide) = if valid {
                        (VALID_RING, INVALID_RING)
                    } else {
                        (INVALID_RING, VALID_RING)
                    };
                    cursor.culled.remove(show);
                    cursor.culled.insert(hide.to_string());
                }
                Effect::GamePlayed => {
                    println!(
                        "Roulette: bets {:?}, the ball on {}.",
                        self.game
                            .bets
                            .iter()
                            .map(|b| (b.index, b.value))
                            .collect::<Vec<_>>(),
                        if self.game.result == roulette::DOUBLE_ZERO {
                            "00".to_string()
                        } else {
                            self.game.result.to_string()
                        }
                    );
                    *state
                        .misc_stats
                        .entry(roulette::GAMES_PLAYED_STAT)
                        .or_insert(0) += 1;
                }
                Effect::Data(d) => *world::casino::data_mut(state, self.casino.form) = d,
                Effect::Broke => {
                    messages.push(super::casino::broke_message(
                        order,
                        world::casino::Game::Roulette,
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
                        "Roulette closed: {} chips, winnings {} (level {}).",
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
            Status::Spot(i) => self
                .game
                .spots
                .get(i)
                .map(|s| s.text.clone())
                .unwrap_or_default(),
            Status::NoSpot => format!(
                "{}: \n{}: ",
                text("sBetText", "Bet"),
                text("sPayoutText", "Payout")
            ),
            Status::Won { amount, lucky } => world::casino::result_line(order, true, amount, lucky),
            Status::Lost { amount } => world::casino::result_line(order, false, amount, false),
            Status::Even => text("sYouBreakEvenText", "You break even"),
        });
        self.menu.fill(ui, &self.game, &earnings, status.as_deref());
    }
}

/// Every frame: the clicks, the cursor's movement, the update, the effects,
/// the tiles. `moved` is the mouse's movement this frame in pixels and the
/// window's height. Returns sounds and corner messages.
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
    let (dx, dy) = std::mem::take(&mut screen.mouse_move);
    // `00715d40() / 00706e50()`: 960 over the screen's height (wider than
    // high).
    let s = 960.0 / screen.ui.screen_size.height_px.max(1) as f32;
    let moved = ((s * -dx) / 10.0, (s * dy) / 10.0);
    let mut open = std::mem::take(&mut screen.open);
    let top = open.len().saturating_sub(1);
    for (i, m) in open.iter_mut().enumerate() {
        let OpenMenu::Roulette(r) = m else {
            continue;
        };
        if r.closed {
            continue;
        }
        r.game.on_top = i == top;
        let mut dice = |n: usize| (state.roll() % n.max(1) as u64) as usize;
        let mut fx = Vec::new();
        for id in std::mem::take(&mut r.menu.clicks) {
            fx.extend(r.game.click(id, &mut dice));
        }
        let models = r.models.clone();
        let moved = if i == top { moved } else { (0.0, 0.0) };
        fx.extend(r.game.update(now_ms as u32, moved, &Clips(&models)));
        r.apply(
            fx,
            order,
            state,
            now_ms / 1000.0,
            &mut sounds,
            &mut messages,
            Some(lock),
        );
        r.fill(&mut screen.ui, order);
    }
    screen.open = open;
    (sounds, messages)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The meshes hidden at the start are the spots' own faces.
    #[test]
    fn spot_faces_match_the_spots() {
        let text = |_: &str, d: &str| d.to_string();
        let spots = roulette::spots(&text);
        let faces: Vec<String> = spot_faces().collect();
        assert_eq!(faces.len(), spots.len());
        for (f, s) in faces.iter().zip(&spots) {
            assert_eq!(*f, s.face.to_ascii_lowercase());
        }
    }
}
