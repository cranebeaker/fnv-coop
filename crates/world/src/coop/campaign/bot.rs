//! Bot verbs for the campaign layer, shared by the headless simulator's bot
//! (`coop-headless`) and the viewer's bot (`viewer/src/coop_live/bot.rs`).
//! They run on the host only; a guest ignores them.
//!
//! ```text
//! coop recruit SLOT NPC          the player in SLOT recruits NPC (caps apply)
//! coop dismiss SLOT NPC
//! coop command SLOT NPC OP       OP: wait | follow | trade (owner only)
//! coop down SLOT [QUEST]         a script run as the player in SLOT takes its health
//!                                (under QUEST's row, for example a Dead Money collar)
//! coop downrevive SLOT           the downed player in SLOT stands up (campaign layer only;
//!                                `coop revive SLOT` is the live revive of the bots)
//! coop stash SLOT HOLDER ITEM N  put N of ITEM into HOLDER as that player
//! coop give SLOT ITEM N          N of ITEM into the player's inventory
//! coop level SLOT N              set the player's level
//! coop objvar SLOT OWNER SCRIPT VAR VALUE   a script object variable as that player
//! coop vote QUEST SLOT yes|no    a vote on an open fork (`coop vote [yes|no]` answers the
//!                                party prompt, the bots' own verb)
//! coop fork QUEST CHOICE         a script asks to take the fork
//! coop pcond QUEST FUNC [ARG]    party condition probe, result in campaign.probes.QUEST_FUNC
//! coop ffhit SLOT QUEST IDX AMT  SLOT hits the IDX-th protected actor of QUEST's row
//! coop hardcore 0|1              the host turns hardcore mode off or on (`SetHardcore`)
//! ```

use coop_core::ids::PlayerId;
use coop_policy::CompanionOp;
use esm::FormId;

use super::{consent, downed, owners, party};
use crate::coop::apply::Ctx;
use crate::coop::facts::puppet_of;
use crate::coop::Authority;
use crate::dialogue::PLAYER_REF;
use crate::scripting::{GameState, Value};

/// Verbs this file runs.
pub fn is_campaign_verb(v: &str) -> bool {
    matches!(
        v,
        "recruit"
            | "dismiss"
            | "command"
            | "down"
            | "downrevive"
            | "stash"
            | "give"
            | "level"
            | "objvar"
            | "vote"
            | "fork"
            | "pcond"
            | "ffhit"
            | "hardcore"
    )
}

/// Whether this file runs the line (`words[0]` is `coop`, `verb` is
/// `words[1]` in lower case). The verbs shared with the bots' own lists are
/// told apart by their arguments.
pub fn claims(verb: &str, words: &[&str]) -> bool {
    if !is_campaign_verb(verb) {
        return false;
    }
    // `coop give HOLDER ITEM N` (a vendor, a container) is the interaction bots' verb:
    // only the slot form is this file's
    if verb == "give" && words.get(2).and_then(|w| w.parse::<u8>().ok()).is_none() {
        return false;
    }
    // `coop vote [yes|no]` answers the party prompt (the bots' verb): only the fork
    // form `coop vote QUEST SLOT yes|no` is this file's
    if verb == "vote" && (words.len() < 5 || matches!(words.get(2).copied(), Some("yes" | "no"))) {
        return false;
    }
    true
}

/// Runs one campaign line (`words` split on white space, `words[0]` is
/// `coop`). `form` reads an editor id or a hex form id. Returns false when
/// the line is not one of this file's.
pub fn bot_line(
    cx: Ctx,
    gs: &mut GameState,
    words: &[&str],
    form: &dyn Fn(&str) -> Option<FormId>,
) -> bool {
    let Some(verb) = words.get(1).map(|v| v.to_ascii_lowercase()) else {
        return false;
    };
    if !claims(&verb, words) {
        return false;
    }
    if gs.coop.authority != Authority::Host {
        return true;
    }
    let arg = |i: usize| words.get(i).copied();
    let slot = |i: usize| arg(i).and_then(|w| w.parse::<u8>().ok());
    let form = |i: usize| arg(i).and_then(form);
    let num = |i: usize| arg(i).and_then(|w| w.parse::<f64>().ok());
    let order = cx.order;
    match verb.as_str() {
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
                downed::script_down(gs, s, form(3));
            }
        }
        "downrevive" => {
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
                    g.emit(crate::coop::Domain::Items, PLAYER_REF);
                });
            }
        }
        "level" => {
            if let (Some(s), Some(n)) = (slot(2), num(3)) {
                gs.with_player(s, |g| g.player_level = n as u16);
            }
        }
        "objvar" => {
            if let (Some(s), Some(owner), Some(script), Some(var), Some(v)) =
                (slot(2), form(3), arg(4), arg(5), num(6))
            {
                gs.coop
                    .campaign
                    .objvar_wanted
                    .entry(script.to_ascii_lowercase())
                    .or_default()
                    .push(var.to_ascii_lowercase());
                gs.coop.campaign.note_script_owner(owner, script);
                // a reference that has not run the script yet gets its variables
                if let std::collections::hash_map::Entry::Vacant(e) = gs.variables.entry(owner) {
                    if let Some(sc) = form(4).and_then(|f| cx.scripts.script(order, f)) {
                        e.insert(script::interp::Locals::new(&sc));
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
                    let by = if s == 0 {
                        PLAYER_REF
                    } else {
                        puppet_of(PlayerId(s))
                    };
                    crate::combat::hurt(order, gs, v, amt, by);
                }
            }
        }
        "hardcore" => {
            // the switch `SetHardcore` and the settings use
            if let Some(n) = num(2) {
                crate::living::needs::set_hardcore(order, gs, n != 0.0);
            }
        }
        _ => {}
    }
    true
}
