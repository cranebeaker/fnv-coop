//! Containers and bodies (`ui::menus::container`, mode 1): opened when the
//! player uses a container or a dead person (`menus::Menu::Container`);
//! the items moved are moved in the world's state (`GameState::move_item`)
//! with the game's sounds, the theft rules (`world::crime`) and the
//! container's `OnClose` block when it closes. "How many?" is the quantity
//! menu (`ui::menus::quantity`) over it.
//!
//! A companion's things (mode 3, `menus::Menu::Teammate`,
//! `OpenTeammateContainer`): `0075bc80` opens it with `DRSTraderOpen` and
//! the companion saying their `FollowersTrade` line (`0075ec60`); giving
//! them something they've no room for (`0075dc80`,
//! `world::items::has_room`) is refused with "<name>
//! `sTeammateOverencumbered`" and their `FollowersOverburdened` line;
//! `0075b750` closes it with `DRSTraderClose`, the companion then
//! choosing the armour they wear and the weapon they hold (`00606540`,
//! `005f9e00`, `world::companions::sort_out_gear`).

use cellview::Game;
use esm::FormId;
use ui::menus::container::{ContainerMenu, Item, Request, Side, FILE};
use ui::menus::quantity::{self, QuantityMenu};
use world::dialogue::PLAYER_REF;
use world::scripting::{Facts, GameState, Runner};

use super::companion_wheel::WheelVoices;
use super::{OpenMenu, Screen};
use crate::menus::Menu;

/// Carry weight's actor value number.
const CARRY_WEIGHT: u16 = 13;

/// The container menu on screen and the reference it shows.
pub struct ContainerScreen {
    pub menu: ContainerMenu,
    pub reference: FormId,
    /// A companion's bark to say once the menu runs (`0075bc80`'s
    /// `FollowersTrade`).
    pub bark: Option<&'static str>,
}

/// Whether this module shows a request.
pub fn takes(menu: &Menu) -> bool {
    matches!(menu, Menu::Container(..) | Menu::Teammate(..))
}

/// A companion's bark on the menu (`0075ec60`): their line for the topic
/// (picked as the wheel picks it, `world::companions::say_topic`, no
/// scripts run) voiced and on `CM_Subtitle` (`0075eea0`).
#[allow(clippy::too_many_arguments)]
fn bark(
    c: &mut ContainerScreen,
    ui: &mut ui::Ui,
    game: &Game,
    scripts: &world::scripting::ScriptCache,
    state: &mut GameState,
    voices: &mut WheelVoices,
    topic: &str,
    now_ms: f64,
) {
    let order = &game.order;
    let who = c.reference;
    let Some(info) = world::companions::say_topic(order, scripts, state, who, topic, false) else {
        println!("Container menu: {who} has no line for {topic}.");
        return;
    };
    if let Some((text, until)) = super::companion_wheel::line(order, voices, who, &info, now_ms) {
        c.menu.say(ui, &text, until);
    }
}

/// A reference's name (its base's `FULL`).
fn name_of(order: &esm::LoadOrder, reference: FormId) -> String {
    base(order, reference)
        .and_then(|b| order.get(b))
        .and_then(|r| r.record().ok())
        .and_then(|r| r.full_name())
        .unwrap_or_default()
}

/// A holder's things as the menu reads them (`world::items`).
fn items(order: &esm::LoadOrder, state: &GameState, holder: FormId) -> Vec<Item> {
    world::items::inventory_lines(order, state, holder)
        .into_iter()
        .map(|l| Item {
            form: l.item.0,
            name: l.name,
            count: l.count,
            form_type: l.form_type,
            equipped: l.equipped,
            quest_item: l.quest_item,
            playable: l.playable,
            regenerating_ammo: l.regenerating_ammo,
            // Weapon mods aren't kept in the world's state.
            modded: false,
            weightless: l.weightless,
            is_caps: l.item.0 == 0xF,
            icon: l.icon,
        })
        .collect()
}

/// The weight line's numbers: what the player carries and can carry.
fn weights(order: &esm::LoadOrder, state: &GameState) -> (f32, f32) {
    let carried = state.inventory_weight(order, PLAYER_REF);
    let most = Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(PLAYER_REF, CARRY_WEIGHT)
    .unwrap_or(0.0) as f32;
    (carried, most)
}

/// The base form a reference stands for.
fn base(order: &esm::LoadOrder, reference: FormId) -> Option<FormId> {
    world::scripting::base_of(order, reference)
}

/// The sounds a container's model makes opening or closing: the `Sound:`
/// text keys of its `Open` / `Close` sequence (`004eef00`, as a door's).
/// Most containers (lockers, ammunition boxes, cabinets, footlockers…) have
/// no `SNAM` / `QNAM` of their own (`0075baf0` reads only those): their
/// sounds are in their model's animation, played when the animation runs.
/// (The lid or door itself isn't animated yet; the sound is.)
fn model_sounds(game: &Game, reference: FormId, opening: bool) -> Vec<FormId> {
    let order = &game.order;
    let Some(rr) = base(order, reference).and_then(|b| order.get(b)) else {
        return Vec::new();
    };
    if rr.entry.header.kind.as_bytes() != b"CONT" {
        return Vec::new();
    }
    let Some(model) = rr
        .record()
        .ok()
        .and_then(|r| r.get(esm::sig::MODL).map(|s| s.zstring()))
    else {
        return Vec::new();
    };
    let Some(bytes) = game.assets.read(&assets::mesh_path(&model)).ok().flatten() else {
        return Vec::new();
    };
    let wanted = if opening { "open" } else { "close" };
    let Some(sequences) = nif::Nif::parse(bytes).ok().and_then(|n| n.sequences().ok()) else {
        return Vec::new();
    };
    sequences
        .iter()
        .filter(|s| s.name.eq_ignore_ascii_case(wanted))
        .flat_map(|s| s.text_keys.iter())
        .filter_map(|(_, text)| crate::doors::text_key_sound(order, text))
        .inspect(|s| {
            println!("Container {reference}: its model's {wanted} sequence plays sound {s}.")
        })
        .collect()
}

/// Opens the container menu for a request.
pub fn open(screen: &mut Screen, game: &Game, state: &mut GameState, request: Menu) -> Vec<FormId> {
    let order = &game.order;
    let (reference, name, mode) = match request {
        Menu::Container(reference, name) => (reference, name, 1),
        Menu::Teammate(who) => (who, name_of(order, who), 3),
        _ => return Vec::new(),
    };
    let mut menu = ContainerMenu::new(0);
    let tile = match screen.load(game, FILE, &mut menu) {
        Ok(t) => t,
        Err(e) => {
            println!("The container menu can't be shown: {e}");
            return Vec::new();
        }
    };
    menu.menu = tile;
    menu.name = name;
    menu.mode = mode;
    // `0075e380`: titles in upper case when the game's language is
    // English (`[General] sLanguage`, default "ENGLISH").
    menu.english = game
        .settings
        .get("General", "sLanguage")
        .is_none_or(|l| l.trim() == "ENGLISH");
    menu.ask_quantity_at = world::scripting::game_setting(order, "iInventoryAskQuantityAt")
        .map_or(ui::menus::container::ASK_QUANTITY_AT, |v| v as i32);
    state.stock(order, reference);
    if mode == 1 {
        state.coop_loot(world::coop::facts::LootOp::Open(reference));
    }
    menu.weights = weights(order, state);
    let player = items(order, state, PLAYER_REF);
    let container = items(order, state, reference);
    if !menu.open(&mut screen.ui, player, container) {
        println!("MENUS: Container Menu Creation Failed.");
        screen.ui.detach(tile);
        return Vec::new();
    }
    println!("Container menu: {}.", menu.name);
    let mut sounds = Vec::new();
    let mut bark = None;
    if mode == 3 {
        sounds.extend(order.form_by_editor_id("DRSTraderOpen"));
        bark = Some("FollowersTrade");
    } else if let Some(s) =
        base(order, reference).and_then(|b| world::sound::container_sound(order, b, true))
    {
        sounds.push(s);
    }
    if mode != 3 {
        sounds.extend(model_sounds(game, reference, true));
    }
    screen
        .open
        .push(OpenMenu::Container(Box::new(ContainerScreen {
            menu,
            reference,
            bark,
        })));
    sounds
}

/// What the menu asked for, carried out: items moved (with the item's
/// sound and the theft rules), "how many?" asked, the menu closed (its
/// sound and the container's `OnClose`). Returns sounds to play.
pub fn after(
    screen: &mut Screen,
    game: &Game,
    scripts: &world::scripting::ScriptCache,
    state: &mut GameState,
    voices: &mut WheelVoices,
    mut library: Option<&mut crate::anim_library::AnimLibrary>,
    now_ms: f64,
) -> Vec<FormId> {
    let order = &game.order;
    let mut sounds = Vec::new();
    // "How many?" answered: the container below takes the number.
    let mut answer = None;
    let mut below = false;
    for m in screen.open.iter_mut() {
        match m {
            OpenMenu::Container(_) => below = true,
            OpenMenu::Barter(_) | OpenMenu::Recipe(_) => below = false,
            OpenMenu::Quantity(q) => {
                if below {
                    if let Some(n) = q.answer.take() {
                        answer = Some(n);
                    }
                }
                for name in q.sounds.drain(..) {
                    if let Some(id) = order.form_by_editor_id(&name) {
                        sounds.push(id);
                    }
                }
            }
            _ => {}
        }
    }
    let mut ask: Option<i32> = None;
    for m in screen.open.iter_mut() {
        let OpenMenu::Container(c) = m else {
            continue;
        };
        if let Some(n) = answer {
            c.menu.quantity_chosen(&mut screen.ui, n);
        }
        for name in c.menu.sounds.drain(..) {
            if let Some(id) = order.form_by_editor_id(&name) {
                sounds.push(id);
            }
        }
        if let Some(topic) = c.bark.take() {
            bark(
                c,
                &mut screen.ui,
                game,
                scripts,
                state,
                voices,
                topic,
                now_ms,
            );
        }
        let reference = c.reference;
        let mode = c.menu.mode;
        let requests: Vec<Request> = std::mem::take(&mut c.menu.requests);
        for request in requests {
            match request {
                Request::Move { from, form, count } => {
                    let item = FormId(form);
                    // A companion with no room for it (`0075dc80`).
                    if mode == 3
                        && from == Side::Player
                        && !world::items::has_room(order, state, reference, item, count)
                    {
                        let text = format!(
                            "{} {}",
                            c.menu.name,
                            world::scripting::game_setting_text(order, "sTeammateOverencumbered")
                                .unwrap_or_else(|| "can't carry any more.".into())
                        );
                        println!("{text}");
                        // Type 2: the sad Vault Boy (`0075dc80`).
                        state.events.push(world::scripting::Event::Message {
                            title: None,
                            text,
                            buttons: Vec::new(),
                            icon: Some(world::message_icon::SAD.to_string()),
                        });
                        bark(
                            c,
                            &mut screen.ui,
                            game,
                            scripts,
                            state,
                            voices,
                            "FollowersOverburdened",
                            now_ms,
                        );
                        continue;
                    }
                    let (giver, taker) = match from {
                        Side::Player => (PLAYER_REF, reference),
                        Side::Container => (reference, PLAYER_REF),
                    };
                    let equipped = state.is_equipped(giver, item);
                    // A guest asks the host for the items; what the host
                    // confirms arrives through the live layer.
                    if mode == 1 && state.is_guest() {
                        let (container, n) = (reference, count.max(0) as u32);
                        state.coop_loot(match from {
                            Side::Container => {
                                world::coop::facts::LootOp::Take { container, item, n }
                            }
                            Side::Player => world::coop::facts::LootOp::Put { container, item, n },
                        });
                        continue;
                    }
                    let moved = state.move_item(order, giver, taker, item, count);
                    if moved == 0 {
                        continue;
                    }
                    if equipped
                        && !state
                            .inventory(order, giver)
                            .iter()
                            .any(|(i, _)| *i == item)
                    {
                        state.unequip_item(order, giver, item);
                    }
                    // Taking from someone else's container is stealing
                    // (`world::crime`).
                    if from == Side::Container {
                        let owner = world::crime::owner_of(order, state, reference)
                            .filter(|&o| !world::crime::may_take(order, state, Some(o)));
                        if let Some(o) = owner {
                            world::crime::steal(order, state, reference, o);
                        }
                    }
                    if !c.menu.closed {
                        // `0075dc80`: the item's pick-up sound.
                        if let Some(s) = world::sound::item_sound(order, item, true) {
                            sounds.push(s);
                        }
                        c.menu.weights = weights(order, state);
                        let player = items(order, state, PLAYER_REF);
                        let container = items(order, state, reference);
                        c.menu.refresh_item(&mut screen.ui, form, player, container);
                    }
                }
                Request::AskQuantity { most, .. } => ask = Some(most),
                Request::Close if mode == 3 => {
                    // `0075b750`: the companion sorts out what they wear
                    // and hold (`world::companions::sort_out_gear`: a
                    // person their armour and, unless their package keeps
                    // weapons away, the weapon `GetBestWeapon` rates best
                    // — with the player's animations, as the game's
                    // rating reads them; a creature its weapon).
                    let s = |n: &str, d: f32| world::scripting::game_setting(order, n).unwrap_or(d);
                    let mut lib = library.as_deref_mut().and_then(|l| l.player_library(game));
                    let anims = lib
                        .as_mut()
                        .map(|l| l as &mut dyn world::animation::pick::Library);
                    let before = world::dps::equipped_weapon(order, state, reference);
                    for item in world::companions::sort_out_gear(order, state, reference, anims, &s)
                    {
                        println!("{reference} puts on {item}.");
                    }
                    let after = world::dps::equipped_weapon(order, state, reference);
                    println!(
                        "{reference} holds {} (before {}).",
                        after.map_or("nothing".into(), |w| w.to_string()),
                        before.map_or("nothing".into(), |w| w.to_string()),
                    );
                    sounds.extend(order.form_by_editor_id("DRSTraderClose"));
                }
                Request::Close => {
                    if let Some(s) = base(order, reference)
                        .and_then(|b| world::sound::container_sound(order, b, false))
                    {
                        sounds.push(s);
                    }
                    sounds.extend(model_sounds(game, reference, false));
                    state.coop_loot(world::coop::facts::LootOp::Close(reference));
                    Runner::new(order, scripts, state).run_event(reference, "onclose", PLAYER_REF);
                }
            }
        }
    }
    if let Some(most) = ask {
        open_quantity(screen, game, most);
    }
    sounds
}

/// "How many?" over the container (`007aba00`: up to `most`, starting at
/// all of it; the pop-up background since a menu is open).
pub fn open_quantity(screen: &mut Screen, game: &Game, most: i32) {
    let mut q = QuantityMenu::new(0);
    match screen.load(game, quantity::FILE, &mut q) {
        Ok(tile) => {
            q.menu = tile;
            let popup = game
                .settings
                .float("Interface", "fPopUpBackgroundOpacity")
                .unwrap_or(ui::game::POPUP_BACKGROUND_OPACITY);
            q.open(&mut screen.ui, most, i32::MAX, Some(popup));
            screen.open.push(OpenMenu::Quantity(q));
        }
        Err(e) => println!("The quantity menu can't be shown: {e}"),
    }
}

/// Every frame: the quantity menu's ticks, the subtitle's time
/// (`0075eac0`).
pub fn update(screen: &mut Screen, now_ms: f64) {
    for m in screen.open.iter_mut() {
        match m {
            OpenMenu::Quantity(q) => q.update(&mut screen.ui),
            OpenMenu::Container(c) => c.menu.update(&mut screen.ui, now_ms),
            _ => {}
        }
    }
}
