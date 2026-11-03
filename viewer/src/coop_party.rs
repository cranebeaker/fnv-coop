//! Party saves in the viewer: thin glue between the save keys, the start flags and
//! `coop_session`'s party layer (`crates/coop-session/src/party.rs`). The format and the
//! protocol are in `world::coop::party_save`, `world::coop::party_char` and that module; see
//! docs/COOP_SAVES.md.
//!
//! * `--coop-load DIR` (host): [`open_start`] starts from the party folder's world, [`setup`]
//!   hands its guest characters to the session.
//! * `--coop-player-key` / `--coop-character` (guest): [`setup`] gives the session the guest's
//!   persistent id and its character file.
//! * F5 / the menu's Save and F9 / Load (`scripts::save_and_load`) only push into
//!   [`PartyBridge`]; [`pump`] hands them to the session after `coop::send`.

use std::path::{Path, PathBuf};

use bevy::prelude::*;
use coop_session::{PartyEvent, Role, Session};
use world::coop::party_char::read_character;
use world::coop::party_save::{load_party, party_dir_for};

use crate::args::CoopArgs;
use crate::coop::CoopLink;
use crate::dialogue::DialogueState;
use crate::fos_start::FosStart;
use crate::frame_order::ViewerSet;
use crate::hud::{HudMessage, HudMessages};
use crate::scripts::Notices;

/// What the save keys ask of the co-op session. Only present in a co-op session.
#[derive(Resource, Default)]
pub struct PartyBridge {
    /// This process is a guest: F5 saves only its character, F9 loads only its character.
    pub guest: bool,
    /// World saves the host just wrote (the path given to the writer).
    pub host_saved: Vec<PathBuf>,
    /// World saves the host just loaded.
    pub host_loaded: Vec<PathBuf>,
    /// A guest pressed save.
    pub guest_save: bool,
    /// A guest pressed load.
    pub guest_load: bool,
}

/// `--coop-load`: the world and the place of a party save, as `fos_start::open` gives them for
/// an original save.
pub fn open_start(game: &cellview::Game, dir: &Path) -> Result<FosStart, String> {
    let loaded = load_party(dir).map_err(|e| e.to_string())?;
    for p in &loaded.problems {
        println!("Party save {}: {p:?}", dir.display());
    }
    let (state, place) = world::save::load(&loaded.save.world_text)?;
    let place = place.ok_or_else(|| "the party save doesn't say where the host was".to_string())?;
    println!(
        "Party save {}: campaign {}, saved by {}, {} guest character(s).",
        dir.display(),
        world::coop::party_save::campaign_hex(loaded.save.campaign),
        loaded.save.host_name,
        loaded.save.guests.len()
    );
    // As F9 opens a saved place (`scripts::load_saved_world`).
    let (scene, outdoors) = match place.world {
        Some(world) => {
            let grid = world::WorldGrid::load(&game.order, world).map_err(|e| e.to_string())?;
            let start = crate::exterior::ExteriorStart {
                grid,
                feet: [place.position[0], place.position[1]],
                height: Some(place.position[2]),
                heading: place.heading,
            };
            (None, Some(start))
        }
        None => {
            let mut scene = game
                .load_cell_now(place.cell, &state.disabled)
                .map_err(|e| e.0)?;
            let [x, y, z] = place.position;
            scene.start = cellview::Start {
                eye: [x, y, z + cellview::EYE_HEIGHT],
                heading: place.heading,
                via: "a party save",
            };
            (Some(scene), None)
        }
    };
    Ok(FosStart {
        state,
        scene,
        outdoors,
    })
}

/// Before the session starts running: turns the party layer on.
pub fn setup(session: &mut Session, role: Role, c: &CoopArgs, name: &str) -> Result<(), String> {
    match role {
        Role::Host => {
            session.party_host_setup(0, name);
            if let Some(dir) = &c.load {
                let loaded = load_party(dir).map_err(|e| e.to_string())?;
                session.party_host_load(&loaded.save);
            }
        }
        Role::Guest => {
            if let Some(key) = c.player_key {
                session.party_guest_setup(key, c.character.clone());
                if let Some(path) = c.character.as_ref().filter(|p| p.exists()) {
                    match read_character(path) {
                        Ok(file) => session.party_guest_offer(file),
                        Err(e) => eprintln!("Character file {}: {e}", path.display()),
                    }
                }
            }
        }
    }
    Ok(())
}

/// Registers [`PartyBridge`] and [`pump`].
pub struct PartyPlugin(pub Role);

impl Plugin for PartyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PartyBridge {
            guest: self.0 == Role::Guest,
            ..PartyBridge::default()
        })
        .add_systems(
            Update,
            pump.in_set(ViewerSet::AfterFrame).after(crate::coop::send),
        );
    }
}

fn pump(
    time: Res<Time>,
    mut link: NonSendMut<CoopLink>,
    state: Res<DialogueState>,
    mut bridge: ResMut<PartyBridge>,
    (mut messages, mut notices): (ResMut<HudMessages>, ResMut<Notices>),
) {
    let now = time.elapsed_secs();
    let mut say = |text: String| {
        println!("{text}");
        if messages.on {
            messages.queue.push(HudMessage::from(text));
        } else {
            notices.push(text, now);
        }
    };
    let s = &mut link.0;
    for path in std::mem::take(&mut bridge.host_saved) {
        let started = std::fs::read_to_string(&path)
            .map_err(|e| e.to_string())
            .and_then(|text| s.party_begin_save(&state.0, party_dir_for(&path), text, 0));
        if let Err(e) = started {
            say(format!("The party save did not start: {e}"));
        }
    }
    for path in std::mem::take(&mut bridge.host_loaded) {
        match load_party(&party_dir_for(&path)) {
            Ok(loaded) => {
                for p in &loaded.problems {
                    say(format!("Party save: {p:?}"));
                }
                s.party_host_load(&loaded.save);
                s.party_host_loaded();
            }
            Err(e) => say(format!(
                "No party data came with this save ({e}); guests keep their characters."
            )),
        }
    }
    if std::mem::take(&mut bridge.guest_save) {
        match s.party_guest_save(&state.0) {
            Ok(p) => say(format!("Your character was saved ({}).", p.display())),
            Err(e) => say(format!("Couldn't save your character: {e}")),
        }
    }
    if std::mem::take(&mut bridge.guest_load) {
        match s.party_guest_load_local() {
            Ok(()) => say("Loading your character from its file.".into()),
            Err(e) => say(format!("Couldn't load your character: {e}")),
        }
    }
    for ev in s.take_party_events() {
        match ev {
            PartyEvent::HostSaved {
                guests,
                from_guests,
                ..
            } => {
                say(format!("Party saved: {guests} guest character(s), {from_guests} saved by the guests themselves."));
            }
            PartyEvent::HostSaveFailed { why, .. } => say(format!("The party save failed: {why}")),
            PartyEvent::GuestSaved { .. } => {
                say("The host saved the game: your character was saved too.".into())
            }
            PartyEvent::GuestSaveRefused { why, .. } => {
                say(format!("Your character could not be saved: {why}"))
            }
            PartyEvent::Campaign { matched, .. } => {
                if matched {
                    say("The host restored your saved character.".into());
                }
            }
            PartyEvent::Stored { ok, .. } => {
                if !ok {
                    say("The host could not store the party save.".into());
                }
            }
            PartyEvent::CharacterRestored => {
                say("The host loaded a save: your saved character is back.".into())
            }
            PartyEvent::OfferAnswered {
                ok,
                conflicts,
                dropped,
                why,
            } => {
                if ok {
                    say(format!(
                        "Your character joined this game ({conflicts} story difference(s) with the host's; the host's story is used, {dropped} story row(s) dropped)."
                    ));
                } else {
                    say(format!("Your character was not accepted: {why}"));
                }
            }
        }
    }
}
