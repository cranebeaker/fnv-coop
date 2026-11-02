//! The state dump the co-op test harness (`tools/coop-run`) reads.
//!
//! One JSON object (schema 1): `quests.<id>.stage/running/completed` (shared
//! quests only; the ones the policy marks per player are under
//! `player_quests.<host|guest|guestN>.<id>`),
//! `globals`, `items.<host|guest|holder>.<item>`, `positions`, `disabled`
//! and `hash.shared` (the shared-state hash, equal on both peers when they
//! agree). Keys are editor IDs where a record has one, else 8 digit hex
//! form ids.
//!
//! `positions` holds both peers' avatars as this peer sees them (its own from
//! the `GameState`, the others from the live layer's [`LiveView`]; the first
//! guest is `guest`, later ones `guest2`...). With per-player shadows on,
//! `items.host` / `items.guest` are each player's own inventory (a guest's
//! comes from the host's authoritative copy of its shadow); without, both
//! show the one shared player. `dead` lists dead actors, `live` the live
//! layer's counters.

use std::collections::BTreeMap;

use esm::{FormId, LoadOrder};
use world::dialogue::PLAYER_REF;
use world::scripting::GameState;

use crate::json::esc;
use crate::live::LiveView;
use crate::Role;

fn name(order: &LoadOrder, f: FormId) -> String {
    order
        .get(f)
        .and_then(|r| r.editor_id().ok().flatten())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| format!("{:08X}", f.0))
}

fn num(v: f32) -> String {
    if v.is_finite() {
        format!("{v}")
    } else {
        "0".into()
    }
}

/// Builds the dump text. `t` is seconds since process start, `seq` counts
/// dumps, `root` is [`crate::Session::state_root`] of `gs`.
pub fn state_json(gs: &GameState, order: &LoadOrder, role: Role, t: f64, seq: u64, root: u64) -> String {
    state_json_live(gs, order, role, t, seq, root, None)
}

/// Counters the live layer's viewer glue keeps, for the dump's `live` object.
#[derive(Clone, Debug, Default)]
pub struct LiveCounters {
    /// Dialogue lines played so far.
    pub lines_played: u64,
    /// Chat lines received so far.
    pub chat_in: u64,
    /// Fast travel approvals applied.
    pub travels: u64,
    /// Hit requests sent to the host.
    pub hits_sent: u64,
}

/// [`state_json`] with the live layer's view of the other players.
pub fn state_json_live(
    gs: &GameState,
    order: &LoadOrder,
    role: Role,
    t: f64,
    seq: u64,
    root: u64,
    view: Option<&LiveView>,
) -> String {
    state_json_full(gs, order, role, t, seq, root, view, &LiveCounters::default())
}

/// [`state_json_live`] with the glue's counters.
#[allow(clippy::too_many_arguments)]
pub fn state_json_full(
    gs: &GameState,
    order: &LoadOrder,
    role: Role,
    t: f64,
    seq: u64,
    root: u64,
    view: Option<&LiveView>,
    counters: &LiveCounters,
) -> String {
    let who = if role == Role::Host { "host" } else { "guest" };
    let mut quests: BTreeMap<String, (u16, bool, bool)> = BTreeMap::new();
    let ids = gs.stages.keys().chain(gs.running.iter()).chain(gs.completed.iter());
    let own = |f: &FormId| gs.coop.bank.plan.as_ref().is_some_and(|p| p.quests.contains(f));
    for f in ids.filter(|f| !own(f)) {
        quests.insert(
            name(order, *f),
            (gs.stages.get(f).copied().unwrap_or(0), gs.running.contains(f), gs.completed.contains(f)),
        );
    }
    let globals: BTreeMap<String, f32> = gs.globals.iter().map(|(f, v)| (name(order, *f), *v)).collect();
    let mut holders: BTreeMap<String, BTreeMap<String, i32>> = BTreeMap::new();
    for ((holder, item), n) in &gs.items {
        if *n == 0 {
            continue;
        }
        let key = if *holder == PLAYER_REF { "player".to_string() } else { name(order, *holder) };
        holders.entry(key).or_default().insert(name(order, *item), *n);
    }
    let player = holders.remove("player").unwrap_or_default();
    let disabled: BTreeMap<String, bool> =
        gs.disabled.iter().filter(|(_, d)| **d).map(|(f, d)| (name(order, *f), *d)).collect();

    let obj = |m: &BTreeMap<String, String>| -> String {
        let parts: Vec<String> = m.iter().map(|(k, v)| format!("\"{}\":{}", esc(k), v)).collect();
        format!("{{{}}}", parts.join(","))
    };
    let q: BTreeMap<String, String> = quests
        .into_iter()
        .map(|(k, (s, r, c))| (k, format!("{{\"stage\":{s},\"running\":{r},\"completed\":{c}}}")))
        .collect();
    let g: BTreeMap<String, String> = globals.into_iter().map(|(k, v)| (k, num(v))).collect();
    let items_of = |m: &BTreeMap<String, i32>| -> String {
        obj(&m.iter().map(|(k, v)| (k.clone(), v.to_string())).collect())
    };
    let mut items: BTreeMap<String, String> = BTreeMap::new();
    let slot_items = |slot: u8| -> Option<String> {
        let sh = gs.shadow_of(slot)?;
        let m: BTreeMap<String, i32> = sh.items.iter().filter(|(_, n)| **n != 0).map(|(i, n)| (name(order, *i), *n)).collect();
        Some(items_of(&m))
    };
    if gs.coop.bank.enabled() {
        items.insert("host".into(), slot_items(0).unwrap_or_else(|| items_of(&BTreeMap::new())));
        items.insert("guest".into(), slot_items(1).unwrap_or_else(|| items_of(&BTreeMap::new())));
        for slot in 2..=8u8 {
            if let Some(text) = slot_items(slot) {
                items.insert(format!("guest{slot}"), text);
            }
        }
    } else {
        items.insert("host".into(), items_of(&player));
        items.insert("guest".into(), items_of(&player));
    }
    for (h, m) in &holders {
        items.insert(h.clone(), items_of(m));
    }
    let mut pos: BTreeMap<String, String> = BTreeMap::new();
    if let Some(p) = gs.player_position {
        pos.insert(who.into(), format!("[{},{},{},{}]", num(p[0]), num(p[1]), num(p[2]), num(gs.player_heading)));
    }
    if let Some(v) = view {
        for a in &v.avatars {
            let key = match a.player.0 {
                0 => "host".to_string(),
                1 => "guest".to_string(),
                n => format!("guest{n}"),
            };
            if key != who {
                pos.insert(key, format!("[{},{},{},{}]", num(a.pos[0]), num(a.pos[1]), num(a.pos[2]), num(a.heading)));
            }
        }
    }
    let dead: BTreeMap<String, String> = gs.dead.iter().map(|d| (name(order, *d), "true".to_string())).collect();
    let live = format!(
        "{{\"avatars\":{},\"lines_played\":{},\"chat_in\":{},\"travels\":{},\"hits_sent\":{}}}",
        view.map_or(0, |v| v.avatars.len()),
        counters.lines_played,
        counters.chat_in,
        counters.travels,
        counters.hits_sent
    );
    // Per-player quests, one object per slot (same keys as `items`).
    let mut pq: BTreeMap<String, String> = BTreeMap::new();
    if gs.coop.bank.enabled() {
        let mut slots: Vec<u8> = gs.coop.bank.parked.keys().copied().collect();
        slots.push(gs.coop.bank.active);
        // 255 is the unassigned placeholder, never a player
        slots.retain(|s| *s != 255);
        for slot in slots {
            let Some(sh) = gs.shadow_of(slot) else { continue };
            let mut m: BTreeMap<String, String> = BTreeMap::new();
            for (f, bits) in &sh.quest_flags {
                let st = sh.quest_stage.get(f).copied().unwrap_or(0);
                m.insert(
                    name(order, *f),
                    format!("{{\"stage\":{st},\"running\":{},\"completed\":{}}}", bits & 1 != 0, bits & 2 != 0),
                );
            }
            for (f, st) in &sh.quest_stage {
                m.entry(name(order, *f))
                    .or_insert_with(|| format!("{{\"stage\":{st},\"running\":false,\"completed\":false}}"));
            }
            let key = match slot {
                0 => "host".to_string(),
                1 => "guest".to_string(),
                n => format!("guest{n}"),
            };
            pq.insert(key, obj(&m));
        }
    }
    let d: BTreeMap<String, String> = disabled.into_iter().map(|(k, v)| (k, v.to_string())).collect();
    format!(
        "{{\"schema\":1,\"role\":\"{who}\",\"t\":{t:.3},\"seq\":{seq},\"quests\":{},\"player_quests\":{},\"globals\":{},\"items\":{},\"positions\":{},\"disabled\":{},\"dead\":{},\"live\":{live},\"hash\":{{\"shared\":\"{root:016x}\"}}}}\n",
        obj(&q),
        obj(&pq),
        obj(&g),
        obj(&items),
        obj(&pos),
        obj(&d),
        obj(&dead)
    )
}
