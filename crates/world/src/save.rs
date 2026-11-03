//! Saving and loading a game: the [`GameState`] as text, one fact a line
//! (`stage 00104C1C 55`), plus where the player stands. Form IDs are as
//! the load order numbers them, so a save belongs to the load order it
//! was made with. This is this reimplementation's own format, not the
//! game's `.fos`.

use std::collections::{BTreeMap, BTreeSet};

use esm::FormId;
use script::interp::Locals;
use script::VarKind;

use crate::scripting::GameState;

pub mod camera;

const HEADER: &str = "nv-rs save 1";

/// Replace a custom nv-rs save only after its complete contents reach disk.
/// This is file handling for our own format, not the native `.fos` writer.
/// The temporary file stays beside the destination so rename never crosses
/// filesystems. A failed write or replacement leaves the previous save alone.
pub fn save_to_path(
    path: &std::path::Path,
    state: &GameState,
    player: Option<PlayerPlace>,
) -> std::io::Result<()> {
    use std::io::Write;
    let text = save(state, player);
    replace_file(path, |file| file.write_all(text.as_bytes()))
}

fn replace_file(
    path: &std::path::Path,
    write: impl FnOnce(&mut std::fs::File) -> std::io::Result<()>,
) -> std::io::Result<()> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let name = path.file_name().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "save path has no filename",
        )
    })?;
    // create_new avoids clobbering another writer or a file left by a crash.
    let (temporary, mut file) = loop {
        let mut temporary_name = name.to_os_string();
        temporary_name.push(format!(
            ".{}.{}.tmp",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let temporary = path.with_file_name(temporary_name);
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
        {
            Ok(file) => break (temporary, file),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e),
        }
    };
    let result = write(&mut file).and_then(|()| file.sync_all());
    drop(file);
    let result = result.and_then(|()| std::fs::rename(&temporary, path));
    if result.is_err() {
        let _ = std::fs::remove_file(&temporary);
    }
    result
}

/// Where the player is in a save: the cell, its worldspace when outdoors,
/// the feet and the heading (radians clockwise from north).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerPlace {
    pub cell: FormId,
    pub world: Option<FormId>,
    pub position: [f32; 3],
    pub heading: f32,
}

fn id(f: FormId) -> String {
    format!("{:08X}", f.0)
}

fn var_kind(kind: VarKind) -> char {
    match kind {
        VarKind::Integer => 'i',
        VarKind::Float => 'f',
        VarKind::Ref => 'r',
    }
}

fn read_var_kind(word: Option<&&str>) -> Option<VarKind> {
    match word {
        Some(&"i") => Some(VarKind::Integer),
        Some(&"f") => Some(VarKind::Float),
        Some(&"r") => Some(VarKind::Ref),
        _ => None,
    }
}

/// The game as text.
pub fn save(state: &GameState, player: Option<PlayerPlace>) -> String {
    let mut out = String::new();
    let mut line = |s: String| {
        out.push_str(&s);
        out.push('\n');
    };
    line(HEADER.to_string());
    if let Some(camera) = &state.saved_camera {
        line(format!("camera {}", camera::encode(camera)));
    }
    if let Some(p) = player {
        line(format!(
            "player {} {} {} {} {} {}",
            id(p.cell),
            p.world.map_or("-".to_string(), id),
            p.position[0],
            p.position[1],
            p.position[2],
            p.heading
        ));
    }
    line(format!("level {}", state.player_level));
    if state.level_up_pending {
        line("levelup".to_string());
    }
    line(format!("dice {}", state.dice));
    // Sorted, so saves of the same game compare equal.
    let sorted = |set: &std::collections::HashSet<FormId>| {
        let mut v: Vec<FormId> = set.iter().copied().collect();
        v.sort();
        v
    };
    let mut stages: Vec<_> = state.stages.iter().collect();
    stages.sort();
    for (q, s) in stages {
        line(format!("stage {} {s}", id(*q)));
    }
    let mut done: Vec<_> = state.stages_done.iter().collect();
    done.sort();
    for (q, s) in done {
        line(format!("done {} {s}", id(*q)));
    }
    for (word, set) in [
        ("running", &state.running),
        ("completed", &state.completed),
        ("failed", &state.failed),
        ("said", &state.said),
        ("topic", &state.topics),
        ("note", &state.notes),
        ("perk", &state.perks),
        ("talked", &state.talked_to),
        ("destroyed", &state.destroyed),
        ("stocked", &state.stocked),
    ] {
        for f in sorted(set) {
            line(format!("{word} {}", id(f)));
        }
    }
    let globals: BTreeMap<_, _> = state.globals.iter().collect();
    for (g, v) in globals {
        line(format!("global {} {v}", id(*g)));
    }
    let variables: BTreeMap<_, _> = state.variables.iter().collect();
    for (owner, locals) in variables {
        for (name, kind, value) in locals.iter() {
            line(format!(
                "var {} {name} {} {value}",
                id(*owner),
                var_kind(kind)
            ));
        }
    }
    for ((q, i), c) in &state.objectives {
        line(format!("objective {} {i} {}", id(*q), u8::from(*c)));
    }
    let disabled: BTreeMap<_, _> = state.disabled.iter().collect();
    for (r, d) in disabled {
        line(format!("disabled {} {}", id(*r), u8::from(*d)));
    }
    let items: BTreeMap<_, _> = state.items.iter().collect();
    for ((h, i), n) in items {
        line(format!("item {} {} {n}", id(*h), id(*i)));
    }
    let packages: BTreeMap<_, _> = state.script_packages.iter().collect();
    for (a, p) in packages {
        line(format!("package {} {}", id(*a), id(*p)));
    }
    let positions: BTreeMap<_, _> = state.positions.iter().collect();
    for (r, (p, h)) in positions {
        line(format!(
            "position {} {} {} {} {h}",
            id(*r),
            p[0],
            p[1],
            p[2]
        ));
    }
    let furniture: BTreeMap<_, _> = state.furniture.iter().collect();
    for (a, f) in furniture {
        line(format!("furniture {} {}", id(*a), id(*f)));
    }
    let values: BTreeMap<_, _> = state.actor_values.iter().collect();
    for ((a, av), v) in values {
        line(format!("av {} {av} {v}", id(*a)));
    }
    let timers: BTreeMap<_, _> = state.quest_timers.iter().collect();
    for (q, t) in timers {
        line(format!("timer {} {t}", id(*q)));
    }
    let delays: BTreeMap<_, _> = state.quest_delays.iter().collect();
    for (q, t) in delays {
        line(format!("delay {} {t}", id(*q)));
    }
    let spaces: BTreeMap<_, _> = state.spaces.iter().collect();
    for (r, (space, cell)) in spaces {
        line(format!("space {} {} {}", id(*r), id(*space), id(*cell)));
    }
    let scales: BTreeMap<_, _> = state.scales.iter().collect();
    for (r, s) in scales {
        line(format!("scale {} {s}", id(*r)));
    }
    let moved: BTreeMap<_, _> = state.havok_moved.iter().collect();
    for (r, (turn, at)) in moved {
        let numbers: Vec<String> = turn
            .iter()
            .flatten()
            .chain(at.iter())
            .map(|v| v.to_string())
            .collect();
        line(format!("havokmove {} {}", id(*r), numbers.join(" ")));
    }
    let moving: BTreeMap<_, _> = state.havok_velocity.iter().collect();
    for (r, (v, w)) in moving {
        let numbers: Vec<String> = v.iter().chain(w.iter()).map(|x| x.to_string()).collect();
        line(format!("havokvel {} {}", id(*r), numbers.join(" ")));
    }
    for (word, set) in [
        ("unconscious", &state.unconscious),
        ("marker", &state.map_markers),
        ("markertravel", &state.map_marker_travel),
        ("found", &state.discovered),
        ("announced", &state.quests_announced),
        ("teammate", &state.teammates),
        ("picked", &state.picked),
    ] {
        for f in sorted(set) {
            line(format!("{word} {}", id(f)));
        }
    }
    // The sky's weather state, as the game saves it (`0063e9f0`): the
    // climate, current, fading-out, picked and forced weathers ("-" for
    // none), when the current began and how far its fade has got, the
    // player's weather region and each region's rolled weather.
    let w = &state.weather;
    let opt = |f: Option<FormId>| f.map_or("-".to_string(), id);
    line(format!(
        "sky {} {} {} {} {} {} {} {}",
        opt(w.climate),
        opt(w.current),
        opt(w.previous),
        opt(w.picked),
        opt(w.forced),
        w.started,
        w.fade,
        opt(w.region)
    ));
    for (r, weather) in &w.region_weathers {
        line(format!("regionweather {} {}", id(*r), id(*weather)));
    }
    for m in &state.modifiers {
        line(format!("modifier {}", id(*m)));
    }
    if let Some(name) = &state.player_name {
        // The rest of the line, spaces and all.
        line(format!("name {name}"));
    }
    if let Some(female) = state.player_female {
        line(format!("female {}", u8::from(female)));
    }
    let ranks: BTreeMap<_, _> = state.perk_ranks.iter().collect();
    for (p, r) in ranks {
        line(format!("perkrank {} {r}", id(*p)));
    }
    for (av, n) in &state.skill_points {
        line(format!("skillpoints {av} {n}"));
    }
    for (stat, n) in &state.misc_stats {
        line(format!("stat {stat} {n}"));
    }
    for (rep, (fame, infamy)) in &state.reputations {
        line(format!("reputation {} {fame} {infamy}", id(*rep)));
    }
    let mut enemies: Vec<FormId> = state.crime_enemies.iter().copied().collect();
    enemies.sort();
    for f in enemies {
        line(format!("crimeenemy {}", id(f)));
    }
    for (f, (minor, major)) in &state.faction_crimes {
        line(format!("factioncrimes {} {minor} {major}", id(*f)));
    }
    line(format!(
        "playercrimes {} {} {}",
        state.player_crimes.0, state.player_crimes.1, state.steal_warnings
    ));
    if let Some(day) = state.last_theft_day {
        line(format!("lasttheft {day}"));
    }
    if state.player_murderer {
        line("murderer".to_string());
    }
    for s in &state.tag_skills {
        line(format!("tag {s}"));
    }
    for (slot, s) in &state.tag_slots {
        line(format!("tagslot {slot} {s}"));
    }
    let damage: BTreeMap<_, _> = state.damage.iter().collect();
    for (who, lost) in damage {
        line(format!("damage {} {lost}", id(*who)));
    }
    for f in sorted(&state.dead) {
        line(format!("dead {}", id(f)));
    }
    let relations: BTreeMap<_, _> = state.faction_relations.iter().collect();
    for ((a, b), code) in relations {
        line(format!("relation {} {} {code}", id(*a), id(*b)));
    }
    let memberships: BTreeMap<_, _> = state.faction_changes.iter().collect();
    for ((who, faction), rank) in memberships {
        line(format!("member {} {} {rank}", id(*who), id(*faction)));
    }
    let fights: BTreeMap<_, _> = state.combat.iter().collect();
    for (who, target) in fights {
        line(format!("fight {} {}", id(*who), id(*target)));
    }
    // Combat groups joined, knock states, `ForceFlee`'s flees ("-" for no
    // place).
    let joined: BTreeMap<_, _> = state.combat_groups.joined.iter().collect();
    for (who, group) in joined {
        line(format!("joined {} {}", id(*who), id(*group)));
    }
    let knocks: BTreeMap<_, _> = state.knocks.state.iter().collect();
    for (who, knock) in knocks {
        line(format!("knock {} {knock}", id(*who)));
    }
    let flees: BTreeMap<_, _> = state.forced_flee.iter().collect();
    for (who, f) in flees {
        let opt = |f: Option<FormId>| f.map_or("-".to_string(), id);
        line(format!("flee {} {} {}", id(*who), opt(f.cell), opt(f.to)));
    }
    let equipped: BTreeMap<_, _> = state.equipped.iter().collect();
    for (who, items) in equipped {
        for item in items {
            line(format!("equipped {} {}", id(*who), id(*item)));
        }
    }
    let taken_off: BTreeMap<_, _> = state.taken_off.iter().collect();
    for (who, items) in taken_off {
        for (item, by) in items {
            match by {
                Some(by) => line(format!("takenoff {} {} {}", id(*who), id(*item), id(*by))),
                None => line(format!("takenoff {} {}", id(*who), id(*item))),
            }
        }
    }
    let ammo_loaded: BTreeMap<_, _> = state.ammo_loaded.iter().collect();
    for (who, ammo) in ammo_loaded {
        line(format!("ammoloaded {} {}", id(*who), id(*ammo)));
    }
    let value_damage: BTreeMap<_, _> = state.value_damage.iter().collect();
    for ((who, av), d) in value_damage {
        line(format!("valuedamage {} {av} {d}", id(*who)));
    }
    let weapon_health: BTreeMap<_, _> = state.weapon_health.iter().collect();
    for ((who, weapon), h) in weapon_health {
        line(format!("weaponhealth {} {} {h}", id(*who), id(*weapon)));
    }
    for (i, item) in state.hotkeys.iter().enumerate() {
        if let Some(item) = item {
            line(format!("hotkey {i} {}", id(*item)));
        }
    }
    let locked: BTreeSet<_> = state.equip_locked.iter().collect();
    for (who, item) in locked {
        line(format!("equiplocked {} {}", id(*who), id(*item)));
    }
    let dropped: BTreeSet<_> = state.dropped.iter().collect();
    for (who, weapon) in dropped {
        line(format!("dropped {} {}", id(*who), id(*weapon)));
    }
    if let Some(q) = state.active_quest {
        line(format!("activequest {}", id(q)));
    }
    // The local map's seen points (`SeenData::SaveGame`, `0087a330`).
    let mut seen: Vec<_> = state.seen.bits.iter().collect();
    seen.sort_by_key(|(k, _)| **k);
    for (k, bits) in seen {
        let (kind, space, x, y) = match *k {
            crate::local_map::SeenKey::Exterior(s, x, y) => ('e', s, x, y),
            crate::local_map::SeenKey::Interior(s, x, y) => ('i', s, x, y),
        };
        let words: Vec<String> = bits.iter().map(|w| format!("{w:08X}")).collect();
        let full = u8::from(state.seen.fully.contains(k));
        line(format!(
            "seen {kind} {} {x} {y} {full} {}",
            id(space),
            words.join(" ")
        ));
    }
    for l in crate::radio::save_lines(&state.radio, &|f| id(f)) {
        line(l);
    }
    if let Some(m) = state.custom_marker {
        let [x, y, z] = m.position;
        line(format!("custommarker {} {x} {y} {z}", id(m.space)));
    }
    let locks: BTreeMap<_, _> = state.locks.iter().collect();
    for (r, level) in locks {
        let level = level.map_or("-".to_string(), |l| l.to_string());
        line(format!("lock {} {level}", id(*r)));
    }
    let broken: BTreeMap<_, _> = state.broken_locks.iter().collect();
    for (r, n) in broken {
        line(format!("brokenlock {} {n}", id(*r)));
    }
    let terminals: BTreeMap<_, _> = state.terminal_states.iter().collect();
    for (r, t) in terminals {
        line(format!(
            "terminal {} {} {}",
            id(*r),
            u8::from(t.hacked),
            t.lockouts
        ));
    }
    // An effect, then its script's variables.
    for e in &state.active_effects {
        line(
            format!(
                "effect {} {} {} {} {} {} {} {} {} {} {} {}",
                id(e.target),
                id(e.source),
                id(e.effect),
                e.actor_value,
                e.magnitude,
                e.remaining,
                u8::from(e.detrimental),
                u8::from(e.recover),
                e.archetype,
                e.resist,
                e.script.map_or("-".to_string(), id),
                u8::from(e.started),
            ) + &if e.caster.is_some() || e.item.is_some() {
                format!(
                    " {} {} {}",
                    e.part,
                    e.caster.map_or("-".to_string(), id),
                    e.item.map_or("-".to_string(), |i| i.to_string())
                )
            } else if e.part >= 0 {
                format!(" {}", e.part)
            } else {
                String::new()
            },
        );
        for (name, kind, value) in e.locals.iter() {
            line(format!("effectvar {name} {} {value}", var_kind(kind)));
        }
    }
    if state.controls_off.iter().any(|&c| c) {
        let flags: Vec<&str> = state
            .controls_off
            .iter()
            .map(|&c| if c { "1" } else { "0" })
            .collect();
        line(format!("controls {}", flags.join(" ")));
    }
    crate::script_functions::save_lines(state, &mut line);
    crate::living::save_lines(state, &mut line);
    crate::more_functions::save_lines(state, &mut line);
    crate::caravan::save_lines(state, &mut line);
    crate::casino::save_lines(state, &mut line);
    crate::tutorial::save_lines(state, &mut line);
    crate::weapon_mods::save_lines(state, &mut line);
    crate::inventory_stacks::save_lines(&state.inventory_stacks, &mut line);
    out
}

/// A saved game back: the state (nothing queued to show) and where the
/// player was.
pub fn load(text: &str) -> Result<(GameState, Option<PlayerPlace>), String> {
    let mut lines = text.lines().enumerate();
    match lines.next() {
        Some((_, h)) if h.trim() == HEADER => {}
        _ => return Err("not an nv-rs save".into()),
    }
    let mut state = GameState::default();
    let mut player = None;
    for (n, raw) in lines {
        let parts: Vec<&str> = raw.split_whitespace().collect();
        let bad = || format!("line {}: can't read '{raw}'", n + 1);
        let form = |i: usize| -> Result<FormId, String> {
            parts
                .get(i)
                .and_then(|s| u32::from_str_radix(s, 16).ok())
                .map(FormId)
                .ok_or_else(bad)
        };
        let num = |i: usize| -> Result<f64, String> {
            parts
                .get(i)
                .and_then(|s| s.parse::<f64>().ok())
                .ok_or_else(bad)
        };
        let Some(&word) = parts.first() else {
            continue;
        };
        match word {
            "camera" => {
                if parts.len() != 2 || state.saved_camera.is_some() {
                    return Err(bad());
                }
                state.saved_camera = Some(camera::decode(parts[1])?);
            }
            "player" => {
                player = Some(PlayerPlace {
                    cell: form(1)?,
                    world: if parts.get(2) == Some(&"-") {
                        None
                    } else {
                        Some(form(2)?)
                    },
                    position: [num(3)? as f32, num(4)? as f32, num(5)? as f32],
                    heading: num(6)? as f32,
                });
            }
            "level" => state.player_level = num(1)? as u16,
            "dice" => state.dice = parts.get(1).and_then(|s| s.parse().ok()).ok_or_else(bad)?,
            "stage" => {
                state.stages.insert(form(1)?, num(2)? as u16);
            }
            "done" => {
                state.stages_done.insert((form(1)?, num(2)? as u16));
            }
            "running" => {
                state.running.insert(form(1)?);
            }
            "completed" => {
                state.completed.insert(form(1)?);
            }
            "failed" => {
                state.failed.insert(form(1)?);
            }
            "said" => {
                state.said.insert(form(1)?);
            }
            "topic" => {
                state.topics.insert(form(1)?);
            }
            "note" => {
                state.notes.insert(form(1)?);
            }
            "perk" => {
                state.perks.insert(form(1)?);
            }
            "talked" => {
                state.talked_to.insert(form(1)?);
            }
            "destroyed" => {
                state.destroyed.insert(form(1)?);
            }
            "stocked" => {
                state.stocked.insert(form(1)?);
            }
            "global" => {
                state.globals.insert(form(1)?, num(2)? as f32);
            }
            "var" => {
                let kind = read_var_kind(parts.get(3)).ok_or_else(bad)?;
                let name = parts.get(2).ok_or_else(bad)?;
                state
                    .variables
                    .entry(form(1)?)
                    .or_insert_with(Locals::default)
                    .insert(name, kind, num(4)?);
            }
            "objective" => {
                state
                    .objectives
                    .insert((form(1)?, num(2)? as i32), num(3)? != 0.0);
            }
            "disabled" => {
                state.disabled.insert(form(1)?, num(2)? != 0.0);
            }
            "item" => {
                state.items.insert((form(1)?, form(2)?), num(3)? as i32);
            }
            "package" => {
                state.script_packages.insert(form(1)?, form(2)?);
            }
            "position" => {
                state.positions.insert(
                    form(1)?,
                    (
                        [num(2)? as f32, num(3)? as f32, num(4)? as f32],
                        num(5)? as f32,
                    ),
                );
            }
            "furniture" => {
                state.furniture.insert(form(1)?, form(2)?);
            }
            "av" => {
                state
                    .actor_values
                    .insert((form(1)?, num(2)? as u16), num(3)?);
            }
            "timer" => {
                state.quest_timers.insert(form(1)?, num(2)? as f32);
            }
            "delay" => {
                state.quest_delays.insert(form(1)?, num(2)? as f32);
            }
            "space" => {
                state.spaces.insert(form(1)?, (form(2)?, form(3)?));
            }
            "scale" => {
                state.scales.insert(form(1)?, num(2)? as f32);
            }
            "havokmove" => {
                let mut v = [0.0f32; 12];
                for (k, x) in v.iter_mut().enumerate() {
                    *x = num(2 + k)? as f32;
                }
                let turn = [[v[0], v[1], v[2]], [v[3], v[4], v[5]], [v[6], v[7], v[8]]];
                state
                    .havok_moved
                    .insert(form(1)?, (turn, [v[9], v[10], v[11]]));
            }
            "havokvel" => {
                let mut v = [0.0f32; 6];
                for (k, x) in v.iter_mut().enumerate() {
                    *x = num(2 + k)? as f32;
                }
                state
                    .havok_velocity
                    .insert(form(1)?, ([v[0], v[1], v[2]], [v[3], v[4], v[5]]));
            }
            "unconscious" => {
                state.unconscious.insert(form(1)?);
            }
            "marker" => {
                state.map_markers.insert(form(1)?);
            }
            "markertravel" => {
                state.map_marker_travel.insert(form(1)?);
            }
            "found" => {
                state.discovered.insert(form(1)?);
            }
            "announced" => {
                state.quests_announced.insert(form(1)?);
            }
            "teammate" => {
                state.teammates.insert(form(1)?);
            }
            "picked" => {
                state.picked.insert(form(1)?);
            }
            "perkrank" => {
                state.perk_ranks.insert(form(1)?, num(2)? as u8);
            }
            "levelup" => state.level_up_pending = true,
            "skillpoints" => {
                state.skill_points.insert(num(1)? as u16, num(2)? as u32);
            }
            "stat" => {
                state.misc_stats.insert(num(1)? as u8, num(2)? as u32);
            }
            "reputation" => {
                state
                    .reputations
                    .insert(form(1)?, (num(2)? as f32, num(3)? as f32));
            }
            "crimeenemy" => {
                state.crime_enemies.insert(form(1)?);
            }
            "factioncrimes" => {
                state
                    .faction_crimes
                    .insert(form(1)?, (num(2)? as u32, num(3)? as u32));
            }
            "playercrimes" => {
                state.player_crimes = (num(1)? as u32, num(2)? as u32);
                state.steal_warnings = num(3)? as u32;
            }
            "lasttheft" => state.last_theft_day = Some(num(1)? as u32),
            "murderer" => state.player_murderer = true,
            // Older saves: a forced weather.
            "weather" => state.weather.forced = Some(form(1)?),
            "sky" => {
                let opt = |i: usize| -> Result<Option<FormId>, String> {
                    if parts.get(i) == Some(&"-") {
                        Ok(None)
                    } else {
                        form(i).map(Some)
                    }
                };
                let w = &mut state.weather;
                w.climate = opt(1)?;
                w.current = opt(2)?;
                w.previous = opt(3)?;
                w.picked = opt(4)?;
                w.forced = opt(5)?;
                w.started = num(6)? as f32;
                w.fade = num(7)? as f32;
                w.region = opt(8)?;
            }
            "regionweather" => {
                state.weather.region_weathers.insert(form(1)?, form(2)?);
            }
            "modifier" => state.modifiers.push(form(1)?),
            "name" => state.player_name = Some(raw.trim_start()[4..].trim().to_string()),
            "female" => state.player_female = Some(num(1)? != 0.0),
            "tag" => {
                state.tag_skills.insert(num(1)? as u16);
            }
            "tagslot" => {
                state.tag_slots.insert(num(1)? as u8, num(2)? as u16);
            }
            "damage" => {
                state.damage.insert(form(1)?, num(2)?);
            }
            "dead" => {
                state.dead.insert(form(1)?);
            }
            "fight" => {
                state.combat.insert(form(1)?, form(2)?);
            }
            "joined" => {
                state.combat_groups.joined.insert(form(1)?, form(2)?);
            }
            "knock" => {
                state.knocks.state.insert(form(1)?, num(2)? as u8);
            }
            "flee" => {
                let opt = |i: usize| form(i).ok();
                state.forced_flee.insert(
                    form(1)?,
                    crate::ai::flee::ForcedFlee {
                        cell: opt(2),
                        to: opt(3),
                    },
                );
            }
            "relation" => {
                state
                    .faction_relations
                    .insert((form(1)?, form(2)?), num(3)? as u8);
            }
            "member" => {
                state
                    .faction_changes
                    .insert((form(1)?, form(2)?), num(3)? as i8);
            }
            "equipped" => state.equipped.entry(form(1)?).or_default().push(form(2)?),
            "takenoff" => state
                .taken_off
                .entry(form(1)?)
                .or_default()
                .push((form(2)?, form(3).ok())),
            "ammoloaded" => {
                state.ammo_loaded.insert(form(1)?, form(2)?);
            }
            "valuedamage" => {
                state
                    .value_damage
                    .insert((form(1)?, num(2)? as u16), num(3)?);
            }
            "weaponhealth" => {
                state
                    .weapon_health
                    .insert((form(1)?, form(2)?), num(3)? as f32);
            }
            "hotkey" => {
                let slot = num(1)? as usize;
                if slot < 8 {
                    state.hotkeys[slot] = Some(form(2)?);
                }
            }
            "equiplocked" => {
                state.equip_locked.insert((form(1)?, form(2)?));
            }
            "dropped" => {
                state.dropped.insert((form(1)?, form(2)?));
            }
            "activequest" => state.active_quest = Some(form(1)?),
            // The radio (`FalloutRadio::SaveGame`, `008366e0`): on, the tuned
            // station, the stations found.
            "radio" => {
                state.radio.on = num(1)? != 0.0;
                state.radio.active = form(2).ok();
                // The tuned station's state, so it plays again.
                if let Some(r) = state.radio.active.filter(|_| state.radio.on) {
                    state.radio.restore_station(r);
                }
            }
            "radiofound" => state.radio.discovered.push(form(1)?),
            "seen" => {
                let space = form(2)?;
                let (x, y) = (num(3)? as i32, num(4)? as i32);
                let key = if parts.get(1) == Some(&"i") {
                    crate::local_map::SeenKey::Interior(space, x, y)
                } else {
                    crate::local_map::SeenKey::Exterior(space, x, y)
                };
                let mut bits = [0u32; 8];
                for (i, b) in bits.iter_mut().enumerate() {
                    *b = parts
                        .get(6 + i)
                        .and_then(|s| u32::from_str_radix(s, 16).ok())
                        .unwrap_or(0);
                }
                state.seen.bits.insert(key, bits);
                if num(5)? != 0.0 {
                    state.seen.fully.insert(key);
                }
            }
            "custommarker" => {
                state.custom_marker = Some(crate::map::CustomMarker {
                    space: form(1)?,
                    position: [num(2)? as f32, num(3)? as f32, num(4)? as f32],
                });
            }
            "lock" => {
                let level = if parts.get(2) == Some(&"-") {
                    None
                } else {
                    Some(num(2)? as u8)
                };
                state.locks.insert(form(1)?, level);
            }
            "brokenlock" => {
                state.broken_locks.insert(form(1)?, num(2)? as u32);
            }
            "terminal" => {
                state.terminal_states.insert(
                    form(1)?,
                    crate::terminal::TerminalState {
                        hacked: num(2)? != 0.0,
                        lockouts: num(3)? as u8,
                    },
                );
            }
            "effect" => {
                let flag = |i: usize| num(i).map(|v| v != 0.0);
                state.active_effects.push(crate::magic::ActiveEffect {
                    target: form(1)?,
                    source: form(2)?,
                    effect: form(3)?,
                    actor_value: num(4)? as i32,
                    magnitude: num(5)? as f32,
                    // "inf" reads as infinity.
                    remaining: num(6)? as f32,
                    detrimental: flag(7)?,
                    recover: flag(8)?,
                    archetype: num(9)? as u32,
                    resist: num(10)? as i32,
                    script: if parts.get(11) == Some(&"-") {
                        None
                    } else {
                        Some(form(11)?)
                    },
                    started: flag(12)?,
                    locals: Locals::default(),
                    // Older saves have no part, caster or item.
                    part: match parts.get(13) {
                        Some(_) => num(13)? as i32,
                        None => -1,
                    },
                    caster: match parts.get(14) {
                        Some(&"-") | None => None,
                        Some(_) => Some(form(14)?),
                    },
                    item: match parts.get(15) {
                        Some(&"-") | None => None,
                        Some(_) => Some(num(15)? as usize),
                    },
                });
            }
            "effectvar" => {
                let kind = read_var_kind(parts.get(2)).ok_or_else(bad)?;
                let name = parts.get(1).ok_or_else(bad)?;
                let value = num(3)?;
                state
                    .active_effects
                    .last_mut()
                    .ok_or_else(bad)?
                    .locals
                    .insert(name, kind, value);
            }
            "controls" => {
                for (i, off) in state.controls_off.iter_mut().enumerate() {
                    *off = num(i + 1)? != 0.0;
                }
            }
            _ => match crate::living::load_line(&mut state, raw)
                .or_else(|| crate::script_functions::load_line(&mut state, raw))
                .or_else(|| crate::more_functions::load_line(&mut state, raw))
                .or_else(|| crate::caravan::load_line(&mut state, raw))
                .or_else(|| crate::casino::load_line(&mut state, raw))
                .or_else(|| crate::tutorial::load_line(&mut state, raw))
                .or_else(|| crate::weapon_mods::load_line(&mut state, raw))
                .or_else(|| crate::inventory_stacks::load_line(&mut state.inventory_stacks, raw))
            {
                Some(Ok(())) => {}
                Some(Err(_)) | None => return Err(bad()),
            },
        }
    }
    Ok((state, player))
}

#[cfg(test)]
mod file_tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn knock_states_flees_and_combat_groups_go_through_a_save() {
        let mut state = GameState::default();
        state.knocks.state.insert(FormId(0x0010_0001), 3);
        state.forced_flee.insert(
            FormId(0x0010_0002),
            crate::ai::flee::ForcedFlee {
                cell: None,
                to: Some(FormId(0x0010_0003)),
            },
        );
        state
            .combat_groups
            .joined
            .insert(FormId(0x0010_0004), FormId(0x0010_0005));
        let text = save(&state, None);
        let (back, _) = load(&text).unwrap();
        assert_eq!(back.knocks, state.knocks);
        assert_eq!(back.forced_flee, state.forced_flee);
        assert_eq!(back.combat_groups, state.combat_groups);
        assert_eq!(save(&back, None), text);
    }

    #[test]
    fn havok_moved_objects_keep_their_pose_through_a_save() {
        let mut state = GameState::default();
        let turn = [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
        state
            .havok_moved
            .insert(FormId(0x0010_ABCD), (turn, [-68250.5, 5800.25, 8390.0]));
        // One still flying when saved keeps its velocities.
        state.havok_velocity.insert(
            FormId(0x0010_ABCD),
            ([120.5, -3.0, 250.0], [0.0, 2.5, -1.0]),
        );
        let text = save(&state, None);
        let (back, _) = load(&text).unwrap();
        assert_eq!(back.havok_moved, state.havok_moved);
        assert_eq!(back.havok_velocity, state.havok_velocity);
        assert_eq!(save(&back, None), text);
    }

    #[test]
    fn failed_save_write_preserves_the_previous_save_and_cleans_up() {
        let data = testdata::functions::functions("failed-save-write");
        let path = data.path().join("save with spaces.txt");
        let mut state = GameState::default();
        state.stages.insert(FormId(0x104c1c), 55);
        save_to_path(&path, &state, None).unwrap();
        let before = std::fs::read(&path).unwrap();
        let files_before = std::fs::read_dir(data.path()).unwrap().count();
        let result = replace_file(&path, |file| {
            file.write_all(b"nv-rs save 1\nstage ")?;
            Err(std::io::Error::other(
                "injected failure after partial write",
            ))
        });
        assert!(result.is_err());
        assert_eq!(std::fs::read(&path).unwrap(), before);
        assert_eq!(
            std::fs::read_dir(data.path()).unwrap().count(),
            files_before
        );
        state.stages.insert(FormId(0x104c1c), 60);
        save_to_path(&path, &state, None).unwrap();
        let (restored, _) = load(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(restored.stages[&FormId(0x104c1c)], 60);
        assert_eq!(
            std::fs::read_dir(data.path()).unwrap().count(),
            files_before
        );
    }

    #[cfg(windows)]
    #[test]
    fn locked_save_stays_readable_when_windows_refuses_replacement() {
        use std::os::windows::fs::OpenOptionsExt;
        let data = testdata::functions::functions("locked-save-replace");
        let path = data.path().join("quicksave.txt");
        save_to_path(&path, &GameState::default(), None).unwrap();
        let before = std::fs::read(&path).unwrap();
        // Share reads but not deletion, as when another process holds a save.
        let reader = std::fs::OpenOptions::new()
            .read(true)
            .share_mode(1)
            .open(&path)
            .unwrap();
        let state = GameState {
            player_level: 8,
            ..Default::default()
        };
        assert!(save_to_path(&path, &state, None).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), before);
        drop(reader);
        save_to_path(&path, &state, None).unwrap();
        assert_eq!(
            load(&std::fs::read_to_string(path).unwrap())
                .unwrap()
                .0
                .player_level,
            8
        );
    }

    #[test]
    fn failed_save_replacement_preserves_destination_and_cleans_up() {
        let data = testdata::functions::functions("failed-save-replace");
        let path = data.path().join("occupied");
        std::fs::create_dir(&path).unwrap();
        std::fs::write(path.join("keep.txt"), b"untouched").unwrap();
        let files_before = std::fs::read_dir(data.path()).unwrap().count();
        assert!(save_to_path(&path, &GameState::default(), None).is_err());
        assert_eq!(std::fs::read(path.join("keep.txt")).unwrap(), b"untouched");
        assert_eq!(
            std::fs::read_dir(data.path()).unwrap().count(),
            files_before
        );
    }
}
