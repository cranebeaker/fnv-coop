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
//! layer's counters, `interact` the player-world interaction counters (acts
//! done by name, `refused`, `shared_in`, `follows_in`, `placed`).
//!
//! Each peer writes its own rows under its own slot name (a later guest's
//! position is `positions.guest2` in its own dump too). The story objects
//! are read from this peer's game state: `topics`, `notes`, `map_markers`
//! (`shown`, `travel`, `found`), `karma`, `reputations`, `factions`,
//! `faction_relations` and `crime` (`story_json`). `status` is the status
//! line and desync dashboard (`status_json`).

use std::collections::BTreeMap;

use esm::{FormId, LoadOrder};
use world::dialogue::PLAYER_REF;
use world::scripting::GameState;

use coop_core::ids::PlayerId;

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

/// The shared world objects the interaction layer carries, by name:
/// `locks.<ref>` (the lock level, -1 unlocked, -2 broken), `open.<door>`,
/// `hacked.<terminal>` (lock-outs, or -1 when hacked) and `mines.<ref>`.
fn objects_json(gs: &GameState, order: &LoadOrder) -> String {
    let obj = |m: &BTreeMap<String, String>| -> String {
        let parts: Vec<String> = m
            .iter()
            .map(|(k, v)| format!("\"{}\":{}", esc(k), v))
            .collect();
        format!("{{{}}}", parts.join(","))
    };
    let mut locks: BTreeMap<String, String> = BTreeMap::new();
    for (f, l) in &gs.locks {
        let broken = gs.broken_locks.get(f).copied().unwrap_or(0) > 0;
        let v = if broken { -2 } else { l.map_or(-1, i32::from) };
        locks.insert(name(order, *f), v.to_string());
    }
    for f in gs.broken_locks.keys().filter(|f| !gs.locks.contains_key(f)) {
        locks.insert(name(order, *f), "-2".into());
    }
    let open: BTreeMap<String, String> = gs
        .set_by_scripts
        .open
        .iter()
        .map(|(f, o)| (name(order, *f), o.to_string()))
        .collect();
    let hacked: BTreeMap<String, String> = gs
        .terminal_states
        .iter()
        .map(|(f, t)| {
            (
                name(order, *f),
                if t.hacked {
                    "-1".to_string()
                } else {
                    t.lockouts.to_string()
                },
            )
        })
        .collect();
    let mut mines: BTreeMap<String, String> = BTreeMap::new();
    for f in &gs.more.mines.disarmed {
        mines.insert(name(order, *f), "\"disarmed\"".into());
    }
    for f in &gs.more.mines.gone {
        mines.insert(name(order, *f), "\"gone\"".into());
    }
    format!(
        "{{\"locks\":{},\"open\":{},\"hacked\":{},\"mines\":{}}}",
        obj(&locks),
        obj(&open),
        obj(&hacked),
        obj(&mines)
    )
}

fn slot_key(slot: u8) -> String {
    match slot {
        0 => "host".to_string(),
        1 => "guest".to_string(),
        n => format!("guest{n}"),
    }
}

/// The `campaign` object: what the campaign layer did, per player slot
/// (`xp`, `followers`, `storage`, `objvars`, `down`), and the counters.
/// Only the host knows every slot; a guest shows what it has.
fn campaign_json(gs: &GameState, order: &LoadOrder) -> String {
    use world::coop::campaign::extra::extra_of;
    let c = &gs.coop.campaign;
    let mut slots: Vec<u8> = gs
        .coop
        .bank
        .parked
        .keys()
        .copied()
        .filter(|s| *s != 255)
        .collect();
    if gs.coop.bank.active != 255 {
        slots.push(gs.coop.bank.active);
    }
    slots.sort_unstable();
    let obj = |m: BTreeMap<String, String>| -> String {
        let parts: Vec<String> = m
            .iter()
            .map(|(k, v)| format!("\"{}\":{}", esc(k), v))
            .collect();
        format!("{{{}}}", parts.join(","))
    };
    let (mut xp, mut followers, mut storage, mut objvars, mut down) = (
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
    );
    // per player progress, needs and ledgers (caravan and casino are the
    // host's own bookkeeping: `LocalExtras` does not replicate)
    let (mut level, mut perks, mut skill_points, mut needs, mut caravan, mut casino) = (
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
    );
    for s in &slots {
        let key = slot_key(*s);
        if let Some(sh) = gs.shadow_of(*s) {
            level.insert(key.clone(), sh.level.to_string());
            perks.insert(key.clone(), sh.perks.len().to_string());
            skill_points.insert(
                key.clone(),
                sh.skill_points.values().sum::<u32>().to_string(),
            );
            let vd = |av: u16| num(sh.value_damage.get(&av).copied().unwrap_or(0.0) as f32);
            needs.insert(
                key.clone(),
                format!(
                    "{{\"thirst\":{},\"hunger\":{},\"sleep\":{}}}",
                    vd(world::living::needs::DEHYDRATION),
                    vd(world::living::needs::HUNGER),
                    vd(world::living::needs::SLEEP_DEPRIVATION)
                ),
            );
            let c = &sh.local.caravan;
            caravan.insert(
                key.clone(),
                format!(
                    "{{\"winnings\":{},\"losses\":{},\"cap_winnings\":{},\"cap_losses\":{}}}",
                    c.winnings, c.losses, c.cap_winnings, c.cap_losses
                ),
            );
            let cs: BTreeMap<String, String> = sh
                .local
                .casinos
                .iter()
                .map(|d| (name(order, d.casino), d.winnings.to_string()))
                .collect();
            casino.insert(key.clone(), obj(cs));
            xp.insert(
                key.clone(),
                num(sh
                    .actor_values
                    .get(&world::experience::XP)
                    .copied()
                    .unwrap_or(0.0) as f32),
            );
            followers.insert(key.clone(), sh.teammates.len().to_string());
        }
        let x = extra_of(gs, *s);
        let mut st: BTreeMap<String, String> = BTreeMap::new();
        for ((h, i), n) in &x.storage {
            st.insert(
                format!("{}_{}", name(order, *h), name(order, *i)),
                n.to_string(),
            );
        }
        storage.insert(key.clone(), obj(st));
        let mut ov: BTreeMap<String, String> = BTreeMap::new();
        for ((o, v), val) in &x.objvars {
            ov.insert(format!("{}_{}", name(order, *o), v), num(*val as f32));
        }
        objvars.insert(key.clone(), obj(ov));
        down.insert(key, c.down.down.contains_key(s).to_string());
    }
    let owners: BTreeMap<String, String> = gs
        .teammates
        .iter()
        .chain(
            gs.coop
                .bank
                .parked
                .values()
                .flat_map(|p| p.teammates.iter()),
        )
        .filter_map(|n| {
            world::coop::campaign::owners::owner_of(gs, *n)
                .map(|o| (name(order, *n), format!("\"{}\"", slot_key(o))))
        })
        .collect();
    let forks: BTreeMap<String, String> = c
        .forks
        .done
        .iter()
        .map(|(q, ch, ok)| (format!("{}_{ch}", name(order, *q)), ok.to_string()))
        .collect();
    let probes: BTreeMap<String, String> = c
        .probes
        .iter()
        .map(|(k, v)| (k.clone(), num(*v as f32)))
        .collect();
    format!(
        "{{\"xp\":{},\"followers\":{},\"owner\":{},\"storage\":{},\"objvars\":{},\"down\":{},\"forks\":{},\"boxes\":{},\"probes\":{},\"paid\":{},\"kills_settled\":{},\"ff_blocked\":{},\"commands_refused\":{},\"respawned\":{},\"warped\":{},\"revived\":{},\"level\":{},\"perks\":{},\"skill_points\":{},\"needs\":{},\"caravan\":{},\"casino\":{},\"hardcore\":{}}}",
        obj(xp),
        obj(followers),
        obj(owners),
        obj(storage),
        obj(objvars),
        obj(down),
        obj(forks),
        world::coop::campaign::consent::boxes_json(gs),
        obj(probes),
        c.paid.len(),
        c.stats.kills_settled,
        c.stats.ff_blocked,
        c.stats.commands_refused,
        c.down.respawned.len(),
        c.down.warped.len(),
        c.down.revived.len(),
        obj(level),
        obj(perks),
        obj(skill_points),
        obj(needs),
        obj(caravan),
        obj(casino),
        gs.living.hardcore
    )
}

/// Builds the dump text. `t` is seconds since process start, `seq` counts
/// dumps, `root` is [`crate::Session::state_root`] of `gs`.
pub fn state_json(
    gs: &GameState,
    order: &LoadOrder,
    role: Role,
    t: f64,
    seq: u64,
    root: u64,
) -> String {
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
    /// Load doors the party's lead made this machine follow through.
    pub doors_followed: u64,
    /// Combat action events received from other players.
    pub puppet_actions: u64,
    /// Actors this machine replicates (host) or reported (guest).
    pub tracked: u64,
    /// Spawned person entities in the scene (the viewer counts them).
    pub npc_entities: u64,
    /// Of those, the ones not hidden.
    pub npc_entities_visible: u64,
    /// Spawned creature entities in the scene.
    pub creature_entities: u64,
    /// Of those, the ones not hidden.
    pub creature_entities_visible: u64,
    /// The start flow, shown as the dump's `start` object when set.
    pub start: Option<crate::start::StartInfo>,
    /// Voice packets sent by the local microphone.
    pub voice_out: u64,
    /// Voice packets accepted from other players.
    pub voice_in: u64,
    /// 20 ms output frames with someone audible.
    pub voiced_frames: u64,
    /// Pings and waypoints the local player placed.
    pub pings: u64,
    /// Frames a name tag showed someone speaking.
    pub speaking_frames: u64,
    /// Handshake refusals reported (host: a guest turned away; guest: refused).
    pub compat_refused: u64,
    /// Handshakes let in with differences (the override).
    pub compat_warned: u64,
    /// The session's status line (`Session::status`), shown as the dump's
    /// `status` object when set.
    pub status: Option<crate::session::Status>,
    /// Joins refused because the party was full (`Session::full_refusals`).
    pub full_refused: u64,
    /// Avatar glitches the remote-avatar detectors saw ([`crate::glitch`]):
    /// position pops, heading snaps, animation flip-flops.
    pub glitch: crate::glitch::GlitchCounts,
    /// The revision of `hash.shared`, and on a host the recent roots
    /// (`Session::hash_points`), shown in the dump's `hash` object when set.
    pub hash_points: Option<crate::session::HashPoints>,
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
    state_json_full(
        gs,
        order,
        role,
        t,
        seq,
        root,
        view,
        &LiveCounters::default(),
    )
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
    // this peer's own slot name for its own rows: a later guest is `guest2`...
    let mine = own_key(role, view);
    let mut quests: BTreeMap<String, (u16, bool, bool)> = BTreeMap::new();
    let ids = gs
        .stages
        .keys()
        .chain(gs.running.iter())
        .chain(gs.completed.iter());
    let own = |f: &FormId| {
        gs.coop
            .bank
            .plan
            .as_ref()
            .is_some_and(|p| p.quests.contains(f))
    };
    for f in ids.filter(|f| !own(f)) {
        quests.insert(
            name(order, *f),
            (
                gs.stages.get(f).copied().unwrap_or(0),
                gs.running.contains(f),
                gs.completed.contains(f),
            ),
        );
    }
    let globals: BTreeMap<String, f32> = gs
        .globals
        .iter()
        .map(|(f, v)| (name(order, *f), *v))
        .collect();
    let mut holders: BTreeMap<String, BTreeMap<String, i32>> = BTreeMap::new();
    for ((holder, item), n) in &gs.items {
        if *n == 0 {
            continue;
        }
        let key = if *holder == PLAYER_REF {
            "player".to_string()
        } else {
            name(order, *holder)
        };
        holders
            .entry(key)
            .or_default()
            .insert(name(order, *item), *n);
    }
    let player = holders.remove("player").unwrap_or_default();
    let disabled: BTreeMap<String, bool> = gs
        .disabled
        .iter()
        .filter(|(_, d)| **d)
        .map(|(f, d)| (name(order, *f), *d))
        .collect();

    let obj = |m: &BTreeMap<String, String>| -> String {
        let parts: Vec<String> = m
            .iter()
            .map(|(k, v)| format!("\"{}\":{}", esc(k), v))
            .collect();
        format!("{{{}}}", parts.join(","))
    };
    let q: BTreeMap<String, String> = quests
        .into_iter()
        .map(|(k, (s, r, c))| {
            (
                k,
                format!("{{\"stage\":{s},\"running\":{r},\"completed\":{c}}}"),
            )
        })
        .collect();
    let g: BTreeMap<String, String> = globals.into_iter().map(|(k, v)| (k, num(v))).collect();
    let items_of = |m: &BTreeMap<String, i32>| -> String {
        obj(&m.iter().map(|(k, v)| (k.clone(), v.to_string())).collect())
    };
    let mut items: BTreeMap<String, String> = BTreeMap::new();
    let slot_items = |slot: u8| -> Option<String> {
        let sh = gs.shadow_of(slot)?;
        let m: BTreeMap<String, i32> = sh
            .items
            .iter()
            .filter(|(_, n)| **n != 0)
            .map(|(i, n)| (name(order, *i), *n))
            .collect();
        Some(items_of(&m))
    };
    if gs.coop.bank.enabled() {
        items.insert(
            "host".into(),
            slot_items(0).unwrap_or_else(|| items_of(&BTreeMap::new())),
        );
        items.insert(
            "guest".into(),
            slot_items(1).unwrap_or_else(|| items_of(&BTreeMap::new())),
        );
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
        pos.insert(
            mine.clone(),
            format!(
                "[{},{},{},{}]",
                num(p[0]),
                num(p[1]),
                num(p[2]),
                num(gs.player_heading)
            ),
        );
    }
    if let Some(v) = view {
        for a in &v.avatars {
            let key = match a.player.0 {
                0 => "host".to_string(),
                1 => "guest".to_string(),
                n => format!("guest{n}"),
            };
            if key != mine {
                pos.insert(
                    key,
                    format!(
                        "[{},{},{},{}]",
                        num(a.pos[0]),
                        num(a.pos[1]),
                        num(a.pos[2]),
                        num(a.heading)
                    ),
                );
            }
        }
    }
    let dead: BTreeMap<String, String> = gs
        .dead
        .iter()
        .map(|d| (name(order, *d), "true".to_string()))
        .collect();
    // the actor table: who each actor is, whether it lives, who owns it
    let me = view.map_or(PlayerId::HOST, |v| v.me);
    let mut actor_rows: BTreeMap<String, String> = BTreeMap::new();
    let (mut owned_local, mut remote, mut dormant, mut creatures, mut creatures_alive) =
        (0usize, 0usize, 0usize, 0usize, 0usize);
    for r in view.map(|v| v.actor_table.as_slice()).unwrap_or(&[]) {
        let who = match r.owner {
            None => {
                dormant += 1;
                "none"
            }
            Some(o) if o == me => {
                owned_local += 1;
                "me"
            }
            Some(_) => {
                remote += 1;
                "other"
            }
        };
        let dead = gs.dead.contains(&r.id) || !r.alive;
        let base = world::more_functions::placed::base_now(order, gs, r.id);
        let creature = base
            .and_then(|b| order.get(b))
            .is_some_and(|x| x.entry.header.kind.as_bytes() == b"CREA");
        if creature {
            creatures += 1;
            if !dead {
                creatures_alive += 1;
            }
        }
        // the cell this machine has the actor in (0: nowhere known), and where it stands
        let place = gs.place(order, r.id);
        let cell = place.map_or(0, |p| p.1 .0);
        let (pos, heading) = place.map_or(([0.0; 3], 0.0), |p| (p.2, p.3));
        actor_rows.insert(
            name(order, r.id),
            format!(
                "{{\"alive\":{},\"owner\":\"{who}\",\"creature\":{creature},\"cell\":{cell},\"pos\":[{:.0},{:.0},{:.0}],\"heading\":{:.2}}}",
                !dead, pos[0], pos[1], pos[2], heading
            ),
        );
    }
    let roam = roam_json(view, gs, &mine);
    let live = format!(
        "{{\"avatars\":{},\"lines_played\":{},\"chat_in\":{},\"travels\":{},\"hits_sent\":{},\"actors_tracked\":{},\"actors_known\":{},\"actors_owned_local\":{},\"actors_remote\":{},\"actors_dormant\":{},\"creatures\":{},\"creatures_alive\":{},\"spawned\":{},\"npc_entities\":{},\"npc_entities_visible\":{},\"creature_entities\":{},\"creature_entities_visible\":{},\"puppet_actions\":{},\"doors_followed\":{},\"voice_out\":{},\"voice_in\":{},\"voiced_frames\":{},\"pings\":{},\"speaking_frames\":{},\"markers\":{},\"weather\":{},\"compat_refused\":{},\"compat_warned\":{},\"roam\":{roam}}}",
        view.map_or(0, |v| v.avatars.len()),
        counters.lines_played,
        counters.chat_in,
        counters.travels,
        counters.hits_sent,
        counters.tracked,
        view.map_or(0, |v| v.actor_table.len()),
        owned_local,
        remote,
        dormant,
        creatures,
        creatures_alive,
        view.map_or(0, |v| v.spawned.len()),
        counters.npc_entities,
        counters.npc_entities_visible,
        counters.creature_entities,
        counters.creature_entities_visible,
        counters.puppet_actions,
        counters.doors_followed,
        counters.voice_out,
        counters.voice_in,
        counters.voiced_frames,
        counters.pings,
        counters.speaking_frames,
        view.map_or(0, |v| v.markers.len()),
        gs.weather.current.map_or(0, |f| f.0),
        counters.compat_refused,
        counters.compat_warned
    );
    let sc = &gs.coop.live.scene.counters;
    let live = format!(
        "{},\"scene\":{{\"active\":{},\"phase\":{},\"locks\":{},\"movies\":{}}}}}",
        live.strip_suffix('}').unwrap_or(&live),
        sc.active,
        sc.phase,
        sc.locks,
        sc.movies
    );
    let live = format!(
        "{},\"full_refused\":{},\"glitch\":{}}}",
        live.strip_suffix('}').unwrap_or(&live),
        counters.full_refused,
        counters.glitch.json()
    );
    // host: the tether's zone membership per player (empty with no tether)
    let zone: BTreeMap<String, String> = gs
        .coop
        .live
        .tether_zone
        .iter()
        .map(|(p, inside)| (slot_key(*p), inside.to_string()))
        .collect();
    let live = format!(
        "{},\"tether_zone\":{}}}",
        live.strip_suffix('}').unwrap_or(&live),
        obj_of(&zone)
    );
    let tk = &gs.coop.live.scene.talk;
    let scene = format!(
        "{{\"active\":{},\"id\":{},\"phase\":{},\"kind\":{},\"locks\":{},\"controls\":{},\"opened\":{},\"closed\":{},\"last_reason\":{},\"movies\":{},\"slide\":{},\"committed\":{},\"elapsed_ms\":{},\"talk\":{{\"notes\":{},\"scenes\":{},\"lines\":{},\"dropped\":{},\"refused\":{},\"checked\":{},\"refusals\":{},\"watched_frames\":{},\"watched_ms\":{},\"watched_clock_ms\":{},\"clock_ms\":{},\"clock_lag_ms\":{}}},\"present\":{}}}",
        sc.active, sc.id, sc.phase, sc.kind, sc.locks, coop_core::scene::Locks::from_flags(&gs.controls_off).0, sc.opened, sc.closed, sc.last_reason, sc.movies, sc.slide, sc.committed, sc.elapsed_ms,
        tk.notes, tk.scenes, tk.lines, tk.dropped, tk.refused, tk.checked, tk.refusals,
        tk.watched_frames, tk.watched_ms, tk.watched_clock_ms, tk.clock_ms, tk.clock_lag_ms,
        world::coop::presentation::json(gs)
    );
    let time = format!(
        "{{\"skips\":{},\"hours\":{}}}",
        sc.time_skips, sc.skip_hours
    );
    let interact = gs.coop.interact.counters_json();
    let objects = objects_json(gs, order);
    // Per-player quests, one object per slot (same keys as `items`).
    let mut pq: BTreeMap<String, String> = BTreeMap::new();
    if gs.coop.bank.enabled() {
        let mut slots: Vec<u8> = gs.coop.bank.parked.keys().copied().collect();
        slots.push(gs.coop.bank.active);
        // 255 is the unassigned placeholder, never a player
        slots.retain(|s| *s != 255);
        for slot in slots {
            let Some(sh) = gs.shadow_of(slot) else {
                continue;
            };
            let mut m: BTreeMap<String, String> = BTreeMap::new();
            for (f, bits) in &sh.quest_flags {
                let st = sh.quest_stage.get(f).copied().unwrap_or(0);
                m.insert(
                    name(order, *f),
                    format!(
                        "{{\"stage\":{st},\"running\":{},\"completed\":{}}}",
                        bits & 1 != 0,
                        bits & 2 != 0
                    ),
                );
            }
            for (f, st) in &sh.quest_stage {
                m.entry(name(order, *f)).or_insert_with(|| {
                    format!("{{\"stage\":{st},\"running\":false,\"completed\":false}}")
                });
            }
            let key = match slot {
                0 => "host".to_string(),
                1 => "guest".to_string(),
                n => format!("guest{n}"),
            };
            pq.insert(key, obj(&m));
        }
    }
    let start = counters
        .start
        .as_ref()
        .map(|i| format!(",\"start\":{}", i.json()))
        .unwrap_or_default();
    // the story and social objects (topics, notes, map markers, karma,
    // reputations, factions, crime; the status line)
    let start = format!(
        "{start}{}{}",
        story_json(gs, order),
        counters
            .status
            .as_ref()
            .map(status_json)
            .unwrap_or_default()
    );
    let combat = gs.coop.live.combat.stats.json();
    // whom each enemy in combat attends to (`combat.targets.<enemy>` = a player key)
    let targets: BTreeMap<String, String> = world::coop::combat_live::enemy_targets(gs, me)
        .into_iter()
        .map(|(e, p)| (name(order, e), format!("\"{}\"", slot_key(p.0))))
        .collect();
    let combat = format!(
        "{},\"targets\":{}}}",
        combat.strip_suffix('}').unwrap_or(&combat),
        obj(&targets)
    );
    let d: BTreeMap<String, String> = disabled
        .into_iter()
        .map(|(k, v)| (k, v.to_string()))
        .collect();
    let campaign = campaign_json(gs, order);
    // the host's campaign rules in force (`rules.mode`, `rules.friendly_fire`)
    let start = match gs.coop.live.rules_choice {
        Some((mode, friendly_fire)) => {
            let r = world::coop::party_save::PartyRules {
                mode,
                friendly_fire,
            };
            format!(
                "{start},\"rules\":{{\"mode\":\"{}\",\"friendly_fire\":\"{}\"}}",
                r.mode_name(),
                r.friendly_fire_name()
            )
        }
        None => start,
    };
    // `hash.rev`: the revision that the hash belongs to. `hash.recent` (host):
    // `[rev, root]` pairs of the last revisions, newest first. A guest that
    // dumps a few revisions behind the host is compared at its own revision.
    let points = counters
        .hash_points
        .as_ref()
        .map(|p| {
            let rev = p.rev.map(|r| format!(",\"rev\":{r}")).unwrap_or_default();
            let recent: Vec<String> = p
                .recent
                .iter()
                .map(|(r, h)| format!("[{r},\"{h:016x}\"]"))
                .collect();
            let recent = if recent.is_empty() {
                String::new()
            } else {
                format!(",\"recent\":[{}]", recent.join(","))
            };
            format!("{rev}{recent}")
        })
        .unwrap_or_default();
    format!(
        "{{\"schema\":1,\"role\":\"{who}\",\"t\":{t:.3},\"seq\":{seq},\"quests\":{},\"player_quests\":{},\"globals\":{},\"items\":{},\"positions\":{},\"disabled\":{},\"dead\":{},\"live\":{live},\"actors\":{},\"scene\":{scene},\"time\":{time},\"interact\":{interact},\"objects\":{objects},\"campaign\":{campaign},\"combat\":{combat},\"hash\":{{\"shared\":\"{root:016x}\"{points}}}{start}}}\n",
        obj(&q),
        obj(&pq),
        obj(&g),
        obj(&items),
        obj(&pos),
        obj(&d),
        obj(&dead),
        obj(&actor_rows)
    )
}

/// The `live.roam` object: every known peer's cell, actors owned per peer,
/// dormant actors, the deferred-operation counters.
fn roam_json(view: Option<&LiveView>, gs: &GameState, who: &str) -> String {
    let key = |p: u8| match p {
        0 => "host".to_string(),
        1 => "guest".to_string(),
        n => format!("guest{n}"),
    };
    let Some(v) = view else {
        return "{}".to_string();
    };
    let s = &v.roam;
    let mut cells: BTreeMap<String, String> = s
        .peers
        .iter()
        .map(|(p, c, _)| (key(p.0), c.to_string()))
        .collect();
    if let Some(c) = gs.player_cell {
        cells.insert(who.to_string(), c.0.to_string());
    }
    let owned: BTreeMap<String, String> = s
        .owned
        .iter()
        .map(|(p, n)| (key(p.0), n.to_string()))
        .collect();
    // outdoor players' grid squares and how many grid cells each loads
    let grids: BTreeMap<String, String> = s
        .grids
        .iter()
        .map(|(p, x, y, n)| (key(p.0), format!("{{\"x\":{x},\"y\":{y},\"cells\":{n}}}")))
        .collect();
    let obj = |m: &BTreeMap<String, String>| -> String {
        let parts: Vec<String> = m
            .iter()
            .map(|(k, v)| format!("\"{}\":{}", esc(k), v))
            .collect();
        format!("{{{}}}", parts.join(","))
    };
    format!(
        "{{\"cells\":{},\"grids\":{},\"owned\":{},\"dormant\":{},\"remote_actors\":{},\"loaded\":{},\"deferred_pending\":{},\"deferred_queued\":{},\"deferred_replayed\":{},\"forwarded\":{},\"confirmed\":{},\"placed\":{}}}",
        obj(&cells),
        obj(&grids),
        obj(&owned),
        s.dormant,
        s.remote_actors,
        s.loaded.len(),
        s.deferred_pending,
        s.deferred_queued,
        s.deferred_replayed,
        s.forwarded,
        s.confirmed,
        s.place.is_some() as u8
    )
}

// ---- story, social state and this peer's own slot name ----------------

/// This peer's own slot name: `host`, `guest`, `guest2`... (a guest whose
/// slot is not known yet is `guest`).
fn own_key(role: Role, view: Option<&LiveView>) -> String {
    match (role, view) {
        (Role::Host, _) => "host".to_string(),
        (_, Some(v)) if v.me.0 != 0 => slot_key(v.me.0),
        _ => "guest".to_string(),
    }
}

fn obj_of(m: &BTreeMap<String, String>) -> String {
    let parts: Vec<String> = m
        .iter()
        .map(|(k, v)| format!("\"{}\":{}", esc(k), v))
        .collect();
    format!("{{{}}}", parts.join(","))
}

/// A person by name, the player as `player`.
fn who_name(order: &LoadOrder, f: FormId) -> String {
    if f == PLAYER_REF {
        "player".to_string()
    } else {
        name(order, f)
    }
}

/// The shared story state as read from this peer's game state, each key
/// with a leading comma: `topics.<topic>`, `notes.<note>`,
/// `map_markers.<marker>.{shown,travel,found}` (what `world::map` gives
/// the Pip-Boy's map), `karma`, `reputations.<rep>.{fame,infamy}`,
/// `factions.<faction>.<member>` (the rank scripts set, -1 removed),
/// `faction_relations.<faction>.<other>` (0 neutral, 1 enemy, 2 ally, 3
/// friend) and `crime` (`factions.<faction>.{minor,major,enemy}`,
/// `minor_total`, `major_total`, `enemies`, `player.{minor,major}`,
/// `murderer`, `steal_warnings`).
fn story_json(gs: &GameState, order: &LoadOrder) -> String {
    let set = |s: &std::collections::HashSet<FormId>| -> String {
        obj_of(
            &s.iter()
                .map(|f| (name(order, *f), "true".to_string()))
                .collect(),
        )
    };
    let mut markers: BTreeMap<String, String> = BTreeMap::new();
    for m in gs.map_markers.iter().chain(&gs.discovered) {
        let mk = world::map::marker(order, *m).unwrap_or(world::map::MapMarker {
            reference: *m,
            name: String::new(),
            position: [0.0; 3],
            radius: 0.0,
            flags: 0,
            kind: 0,
            arrival: None,
        });
        markers.insert(
            name(order, *m),
            format!(
                "{{\"shown\":{},\"travel\":{},\"found\":{}}}",
                world::map::shown(gs, &mk),
                world::map::can_travel(gs, &mk),
                gs.discovered.contains(m)
            ),
        );
    }
    let reps: BTreeMap<String, String> = gs
        .reputations
        .iter()
        .map(|(r, (fame, infamy))| {
            (
                name(order, *r),
                format!("{{\"fame\":{},\"infamy\":{}}}", num(*fame), num(*infamy)),
            )
        })
        .collect();
    let mut factions: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    for ((who, faction), rank) in &gs.faction_changes {
        factions
            .entry(name(order, *faction))
            .or_default()
            .insert(who_name(order, *who), rank.to_string());
    }
    let mut relations: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    for ((a, b), r) in &gs.faction_relations {
        relations
            .entry(name(order, *a))
            .or_default()
            .insert(name(order, *b), r.to_string());
    }
    let nested = |m: &BTreeMap<String, BTreeMap<String, String>>| -> String {
        obj_of(&m.iter().map(|(k, v)| (k.clone(), obj_of(v))).collect())
    };
    let mut crimes: BTreeMap<String, String> = BTreeMap::new();
    let (mut minor_total, mut major_total) = (0u64, 0u64);
    let with_crime = gs
        .faction_crimes
        .iter()
        .filter(|(_, (minor, major))| *minor != 0 || *major != 0)
        .map(|(f, _)| f)
        .chain(&gs.crime_enemies);
    for f in with_crime {
        let (minor, major) = gs.faction_crimes.get(f).copied().unwrap_or((0, 0));
        crimes.entry(name(order, *f)).or_insert_with(|| {
            minor_total += u64::from(minor);
            major_total += u64::from(major);
            format!(
                "{{\"minor\":{minor},\"major\":{major},\"enemy\":{}}}",
                gs.crime_enemies.contains(f)
            )
        });
    }
    let crime = format!(
        "{{\"factions\":{},\"minor_total\":{minor_total},\"major_total\":{major_total},\"enemies\":{},\"player\":{{\"minor\":{},\"major\":{}}},\"murderer\":{},\"steal_warnings\":{}}}",
        obj_of(&crimes),
        gs.crime_enemies.len(),
        gs.player_crimes.0,
        gs.player_crimes.1,
        gs.player_murderer,
        gs.steal_warnings
    );
    format!(
        ",\"topics\":{},\"notes\":{},\"map_markers\":{},\"karma\":{},\"reputations\":{},\"factions\":{},\"faction_relations\":{},\"crime\":{crime}",
        set(&gs.topics),
        set(&gs.notes),
        obj_of(&markers),
        num(world::reputation::karma(order, gs)),
        obj_of(&reps),
        nested(&factions),
        nested(&relations)
    )
}

/// The dump's `status` object (with a leading comma): what the status line
/// and the desync dashboard show. `players` counts this peer too, `peers`
/// holds the others by slot name (`name`, `rtt_ms`), `rtt_ms` is a guest's
/// round trip to the host (the host: its slowest guest's), and `line` and
/// `compact` are the texts the HUD shows.
fn status_json(s: &crate::session::Status) -> String {
    use crate::session::State;
    let state = match s.state {
        State::Hosting => "hosting",
        State::Connecting => "connecting",
        State::Joining => "joining",
        State::Connected => "connected",
        State::Reconnecting => "reconnecting",
        State::Rejected(_) => "rejected",
    };
    let peers: BTreeMap<String, String> = s
        .peers
        .iter()
        .filter(|p| p.player != s.player)
        .map(|p| {
            (
                slot_key(p.player.0),
                format!("{{\"name\":\"{}\",\"rtt_ms\":{}}}", esc(&p.name), p.rtt_ms),
            )
        })
        .collect();
    let rtt = s.rtt_ms.unwrap_or_else(|| {
        s.peers
            .iter()
            .filter(|p| !p.player.is_host())
            .map(|p| p.rtt_ms)
            .max()
            .unwrap_or(0)
    });
    format!(
        ",\"status\":{{\"state\":\"{state}\",\"me\":\"{}\",\"players\":{},\"peers\":{},\"rtt_ms\":{rtt},\"desync\":{},\"desyncs\":{},\"repairs\":{},\"snapshots\":{},\"rev\":{},\"pending_commands\":{},\"decode_errors\":{},\"line\":\"{}\",\"compact\":\"{}\"}}",
        slot_key(s.player.0),
        s.players(),
        obj_of(&peers),
        s.desync,
        s.desyncs,
        s.repairs,
        s.snapshots,
        s.rev,
        s.pending_commands,
        s.decode_errors,
        esc(&s.line()),
        esc(&s.compact())
    )
}
