//! Party saves in the simulator: the stand-in for the viewer's `coop_party` glue.
//!
//! Script lines (bot lines, so `coop` prefixed, run on the peer that has them):
//!
//! * `coop psave` (host): saves the world text and orders the guests to save their characters;
//!   the party folder lands in the run's party directory.
//! * `coop pload` (guest): the guest's F9: restores its character from its local file.
//!
//! A scenario with a `party` object gives the guest a persistent id and may restart both peers
//! from the party save (see [`crate::scenario::PartyCfg`]). A restart is "drop everything":
//! new sessions, new link, new `GameState`s; the host loads the party folder as the viewer's
//! `--coop-load` does and the guest rejoins with its id.

use std::path::{Path, PathBuf};

use world::coop::party_char::{carry_coop, read_character};
use world::coop::party_save::{load_party, party_dir_for};
use world::scripting::GameState;

use crate::data::Data;
use crate::json::{n, s, J};
use crate::peer::Peer;

/// The file a guest keeps its own character in.
pub const GUEST_FILE: &str = "guest.char";
/// The name of the host's save inside the party directory.
pub const HOST_SAVE: &str = "Save.txt";

/// Where the party folder of the run lives.
pub fn party_folder(dir: &Path) -> PathBuf {
    party_dir_for(&dir.join(HOST_SAVE))
}

/// Turns the party layer on for a peer.
pub fn attach(p: &mut Peer, key: u64, dir: &Path) {
    p.party_dir = Some(dir.to_path_buf());
    match p.role {
        coop_session::Role::Host => p.sess.party_host_setup(0, "Host"),
        coop_session::Role::Guest => p.sess.party_guest_setup(key, Some(dir.join(GUEST_FILE))),
    }
}

/// Handles a `coop psave` / `coop pload` line. True when it was one.
pub fn line(p: &mut Peer, t: f64, l: &str) -> bool {
    let verb = l.split_whitespace().nth(1).map(str::to_ascii_lowercase);
    let Some(dir) = p.party_dir.clone() else {
        return false;
    };
    match verb.as_deref() {
        Some("psave") => {
            let text = world::save::save(&p.gs, None);
            match p
                .sess
                .party_begin_save(&p.gs, party_folder(&dir), text, 1_700_000_000)
            {
                Ok(seq) => log(p, t, "party_save_begun", vec![("seq", n(seq as f64))]),
                Err(e) => log(p, t, "party_save_refused", vec![("why", s(e))]),
            }
            true
        }
        Some("pload") => {
            match read_character(&dir.join(GUEST_FILE)) {
                Ok(f) => {
                    p.sess.party_guest_load(&f);
                    log(
                        p,
                        t,
                        "party_character_load",
                        vec![("level", n(f.shadow().map_or(0, |x| x.level) as f64))],
                    );
                }
                Err(e) => log(
                    p,
                    t,
                    "party_character_missing",
                    vec![("why", s(e.to_string()))],
                ),
            }
            true
        }
        _ => false,
    }
}

fn log(p: &mut Peer, t: f64, ev: &str, f: Vec<(&str, J)>) {
    p.log_pub(t, ev, f);
}

/// Per frame: the party events become log lines the verdict can show.
pub fn pump(p: &mut Peer, t: f64) {
    for e in p.sess.take_party_events() {
        let (name, f) = match &e {
            coop_session::PartyEvent::HostSaved {
                seq,
                guests,
                from_guests,
                ..
            } => (
                "party_saved",
                vec![
                    ("seq", n(*seq as f64)),
                    ("guests", n(*guests as f64)),
                    ("from_guests", n(*from_guests as f64)),
                ],
            ),
            coop_session::PartyEvent::HostSaveFailed { why, .. } => {
                ("party_save_failed", vec![("why", s(why.clone()))])
            }
            coop_session::PartyEvent::GuestSaved { seq, .. } => {
                ("party_char_saved", vec![("seq", n(*seq as f64))])
            }
            coop_session::PartyEvent::GuestSaveRefused { why, .. } => {
                ("party_char_refused", vec![("why", s(why.clone()))])
            }
            coop_session::PartyEvent::Campaign { matched, .. } => {
                ("party_campaign", vec![("matched", J::Bool(*matched))])
            }
            coop_session::PartyEvent::Stored { ok, .. } => {
                ("party_stored", vec![("ok", J::Bool(*ok))])
            }
            coop_session::PartyEvent::CharacterRestored => ("party_restored", vec![]),
            coop_session::PartyEvent::OfferAnswered { ok, conflicts, .. } => (
                "party_offer_answered",
                vec![("ok", J::Bool(*ok)), ("conflicts", n(*conflicts as f64))],
            ),
        };
        p.log_pub(t, name, f);
        p.party_log.push(name.to_string());
    }
}

/// After a restart: the host loads the party folder (the viewer's `--coop-load`).
pub fn restore_host(host: &mut Peer, d: &Data) -> Result<(), String> {
    let dir = host.party_dir.clone().ok_or("no party directory")?;
    if !party_folder(&dir).exists() {
        // nothing was saved (a mutation twin): the host starts a fresh world
        return Ok(());
    }
    let loaded = load_party(&party_folder(&dir)).map_err(|e| e.to_string())?;
    let (mut state, _) = world::save::load(&loaded.save.world_text)?;
    state.player_cell = host.gs.player_cell;
    state.player_world = host.gs.player_world;
    let mut old: GameState = std::mem::take(&mut host.gs);
    carry_coop(&mut old, &mut state, Some(&d.order));
    host.gs = state;
    host.sess.party_host_load(&loaded.save);
    Ok(())
}

/// The guest's character file, if it saved one.
pub fn guest_file(dir: &Path) -> Option<world::coop::party_char::CharacterFile> {
    read_character(&dir.join(GUEST_FILE)).ok()
}
