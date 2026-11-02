//! Bot verbs for the campaign layer (host side only; a guest ignores them).
//!
//! ```text
//! coop recruit SLOT NPC          the player in SLOT recruits NPC (caps apply)
//! coop dismiss SLOT NPC
//! coop command SLOT NPC OP       OP: wait | follow | trade (owner only)
//! coop down SLOT [QUEST]         the player in SLOT is downed
//! coop revive SLOT
//! coop stash SLOT HOLDER ITEM N  put N of ITEM into HOLDER as that player
//! coop give SLOT ITEM N          N of ITEM into the player's inventory
//! coop level SLOT N              set the player's level
//! coop objvar SLOT OWNER SCRIPT VAR VALUE   a script object variable as that player
//! coop vote QUEST SLOT yes|no    a vote on an open fork
//! coop fork QUEST CHOICE         a script asks to take the fork
//! coop pcond QUEST FUNC [ARG]    party condition probe, result in campaign.probes.QUEST.FUNC
//! coop ffhit SLOT QUEST IDX AMT  SLOT hits the IDX-th protected actor of QUEST's row
//! ```

use coop_policy::CompanionOp;
use esm::FormId;
use world::coop::apply::Ctx;
use world::coop::campaign::{consent, downed, owners, party};
use world::coop::facts::puppet_of;
use world::coop::Authority;
use world::dialogue::PLAYER_REF;
use world::scripting::{GameState, Value};

use crate::data::Data;

/// Verbs this file runs.
pub fn is_campaign_verb(v: &str) -> bool {
    matches!(
        v,
        "recruit" | "dismiss" | "command" | "down" | "revive" | "stash" | "give" | "level" | "objvar" | "vote" | "fork" | "pcond" | "ffhit"
    )
}

/// Runs one campaign verb. Returns false when `verb` is not one of them.
pub fn run(verb: &str, words: &[&str], d: &Data, gs: &mut GameState, cx: Ctx) -> bool {
    if !is_campaign_verb(verb) {
        return false;
    }
    // `coop give HOLDER ITEM N` (a vendor, a container) is the interaction bots' verb
    // (`bot.rs`): only the slot form is this file's
    if verb == "give" && words.get(2).and_then(|w| w.parse::<u8>().ok()).is_none() {
        return false;
    }
    if gs.coop.authority != Authority::Host {
        return true;
    }
    let arg = |i: usize| words.get(i).copied();
    let slot = |i: usize| arg(i).and_then(|w| w.parse::<u8>().ok());
    let form = |i: usize| arg(i).and_then(|w| d.form(w));
    let num = |i: usize| arg(i).and_then(|w| w.parse::<f64>().ok());
    let order = &d.order;
    match verb {
        "recruit" => {
            if let (Some(s), Some(n)) = (slot(2), form(3)) {
                owners::recruit(order, gs, s, n);
            }
        }
        "dismiss" => {
            if let (Some(s), Some(n)) = (slot(2), form(3)) {
                owners::dismiss(gs, s, n);
            }
        }
        "command" => {
            if let (Some(s), Some(n)) = (slot(2), form(3)) {
                let op = match arg(4) {
                    Some("follow") => CompanionOp::Follow,
                    Some("trade") => CompanionOp::Trade,
                    _ => CompanionOp::Wait,
                };
                owners::command_ok(gs, s, n, op);
            }
        }
        "down" => {
            if let Some(s) = slot(2) {
                downed::player_down(gs, s, form(3));
            }
        }
        "revive" => {
            if let Some(s) = slot(2) {
                downed::revive(gs, s);
            }
        }
        "stash" => {
            if let (Some(s), Some(h), Some(i), Some(n)) = (slot(2), form(3), form(4), num(5)) {
                gs.with_player(s, |g| {
                    g.items.insert((h, i), n as i32);
                });
            }
        }
        "give" => {
            if let (Some(s), Some(i), Some(n)) = (slot(2), form(3), num(4)) {
                gs.with_player(s, |g| {
                    // stocked first: an unstocked holder's count is its record's kit
                    g.stock(order, PLAYER_REF);
                    g.items.insert((PLAYER_REF, i), n as i32);
                    g.emit(world::coop::Domain::Items, PLAYER_REF);
                });
            }
        }
        "level" => {
            if let (Some(s), Some(n)) = (slot(2), num(3)) {
                gs.with_player(s, |g| g.player_level = n as u16);
            }
        }
        "objvar" => {
            if let (Some(s), Some(owner), Some(script), Some(var), Some(v)) = (slot(2), form(3), arg(4), arg(5), num(6)) {
                gs.coop.campaign.objvar_wanted.entry(script.to_ascii_lowercase()).or_default().push(var.to_ascii_lowercase());
                gs.coop.campaign.note_script_owner(owner, script);
                // a reference that has not run the script yet gets its variables
                if !gs.variables.contains_key(&owner) {
                    if let Some(sc) = d.form(script).and_then(|f| cx.scripts.script(order, f)) {
                        gs.variables.insert(owner, script::interp::Locals::new(&sc));
                    }
                }
                gs.with_player(s, |g| {
                    if let Some(l) = g.variables.get_mut(&owner) {
                        l.set(&var.to_ascii_lowercase(), v);
                    }
                });
            }
        }
        "vote" => {
            if let (Some(q), Some(s)) = (form(2), slot(3)) {
                consent::vote(gs, q, s, arg(4) == Some("yes"));
            }
        }
        "fork" => {
            if let (Some(q), Some(c)) = (form(2), num(3)) {
                consent::check(gs, q, c as u32);
            }
        }
        "pcond" => {
            if let (Some(q), Some(f)) = (form(2), arg(3)) {
                let args: Vec<Value> = form(4).map(Value::Form).into_iter().collect();
                if let Some(v) = party::party_value(gs, q, f, &args, coop_policy::Phase::Entry) {
                    let key = format!("{}_{f}", arg(2).unwrap_or("q"));
                    gs.coop.campaign.probes.insert(key, v);
                }
            }
        }
        "ffhit" => {
            if let (Some(s), Some(q), Some(idx), Some(amt)) = (slot(2), form(3), num(4), num(5)) {
                let victim = gs
                    .coop
                    .gate
                    .as_ref()
                    .and_then(|g| g.row(q))
                    .and_then(|r| r.friendly_fire.as_ref())
                    .and_then(|ff| ff.actors.get(idx as usize))
                    .and_then(|a| owners::resolve(order, a));
                if let Some(v) = victim {
                    let by = if s == 0 { PLAYER_REF } else { puppet_of(coop_session::PlayerId(s)) };
                    world::combat::hurt(order, gs, v, amt, by);
                }
            }
        }
        _ => {}
    }
    let _ = FormId(0);
    true
}
