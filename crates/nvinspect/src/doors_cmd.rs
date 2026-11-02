//! `doors <CELL>` / `doors <WORLD> <X> <Y>`: the doors placed in a cell or
//! an outdoor square, with what the game's door rules (`world::doors`)
//! read: whether each leads elsewhere or swings where it stands, how it
//! starts, its lock, its base's flags and sounds, its model's `Open` and
//! `Close` sequences with their text keys, and the collision its leaf
//! swings with. `doors all` surveys every `DOOR` record's flags and model.

use std::collections::BTreeMap;
use std::io::Write;

use assets::Assets;
use esm::{FormId, FourCC, LoadOrder};
use world::scripting::{GameState, ScriptCache};

use crate::records::describe_id;
use crate::render_cmd::{single_cell, world_error};
use crate::CliError;

const DOOR: FourCC = FourCC::new(b"DOOR");
const MODL: FourCC = FourCC::new(b"MODL");
const FULL: FourCC = FourCC::new(b"FULL");

/// What a door model holds, as the viewer uses it.
struct DoorModel {
    sequences: Vec<nif::Sequence>,
    /// Keyframed collision parts: their node chain and collision flags.
    leaves: Vec<(String, u16)>,
    /// The first leaf's node chain, for where each sequence turns it.
    chain: Vec<(String, nif::Transform)>,
    error: Option<String>,
}

/// How far a sequence turns the leaf about the vertical, at its start and
/// end, degrees from the model's rest pose (the file's own: closed for the
/// game's door models).
fn leaf_turn(chain: &[(String, nif::Transform)], s: &nif::Sequence) -> Option<(f32, f32)> {
    if chain.is_empty() {
        return None;
    }
    let rest = nif::posed_chain(chain, &[]);
    let heading = |t: f32| {
        let posed = nif::posed_chain(chain, &[(s, t)]);
        let m = posed.then_child(&rest.inverse()).rotation;
        // The turned x axis's angle in the x-y plane.
        m[1][0].atan2(m[0][0]).to_degrees()
    };
    Some((heading(s.start), heading(s.stop)))
}

fn read_model(assets: &Assets, path: &str) -> DoorModel {
    let full = assets::mesh_path(path);
    let failed = |error: String| DoorModel {
        sequences: Vec::new(),
        leaves: Vec::new(),
        chain: Vec::new(),
        error: Some(error),
    };
    let bytes = match assets.read(&full) {
        Ok(Some(b)) => b,
        Ok(None) => return failed("not found".into()),
        Err(e) => return failed(e.to_string()),
    };
    match nif::Nif::parse(bytes) {
        Ok(nif) => {
            let parts: Vec<nif::CollisionPart> = nif
                .placed_collision()
                .map(|c| c.parts.into_iter().filter(|p| p.keyframed).collect())
                .unwrap_or_default();
            DoorModel {
                sequences: nif.sequences().unwrap_or_default(),
                leaves: parts
                    .iter()
                    .map(|p| {
                        let chain: Vec<&str> = p.nodes.iter().map(|(n, _)| n.as_str()).collect();
                        (chain.join(" > "), p.flags)
                    })
                    .collect(),
                chain: parts.first().map(|p| p.nodes.clone()).unwrap_or_default(),
                error: None,
            }
        }
        Err(e) => failed(e.to_string()),
    }
}

fn flag_words(flags: u8) -> String {
    let mut words = Vec::new();
    if flags & 0x01 != 0 {
        words.push("oblivion gate");
    }
    if flags & world::doors::AUTOMATIC != 0 {
        words.push("automatic");
    }
    if flags & world::doors::HIDDEN != 0 {
        words.push("hidden");
    }
    if flags & world::doors::MINIMAL_USE != 0 {
        words.push("minimal use");
    }
    if flags & world::doors::SLIDING != 0 {
        words.push("sliding");
    }
    if words.is_empty() {
        "none".to_string()
    } else {
        words.join(", ")
    }
}

fn sequence_line(s: &nif::Sequence) -> String {
    let keys: Vec<String> = s
        .text_keys
        .iter()
        .map(|(t, k)| format!("{t:.2} s \"{k}\""))
        .collect();
    format!(
        "{} {:.2} s{}",
        s.name,
        s.stop - s.start,
        if keys.is_empty() {
            String::new()
        } else {
            format!(" (keys: {})", keys.join(", "))
        }
    )
}

fn sound_name(order: &LoadOrder, id: Option<FormId>) -> String {
    match id {
        Some(id) => describe_id(order, id),
        None => "none".into(),
    }
}

/// The doors of a cell or square.
pub fn doors(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &Assets,
    target: &str,
    square: Option<(i32, i32)>,
) -> Result<(), CliError> {
    if target.eq_ignore_ascii_case("all") {
        return survey(out, order, assets);
    }
    let loaded = match square {
        Some(square) => {
            let world = world::find_worldspace(order, target)
                .map_err(world_error)?
                .ok_or_else(|| {
                    CliError::NotFound(format!(
                        "no worldspace matches '{target}'. `worlds` lists them."
                    ))
                })?;
            let grid = world::WorldGrid::load(order, world).map_err(world_error)?;
            grid.load_square(order, square)
                .map_err(world_error)?
                .ok_or_else(|| {
                    CliError::NotFound(format!("the worldspace has no square {square:?}"))
                })?
        }
        None => world::load_cell(order, single_cell(order, target)?).map_err(world_error)?,
    };
    let state = GameState::new(order);
    let scripts = ScriptCache::default();
    let mut doors: Vec<&world::Placement> = loaded
        .objects
        .iter()
        .filter(|o| o.base_type == DOOR)
        .collect();
    doors.sort_by_key(|d| d.form_id.0);
    writeln!(
        out,
        "{} doors in {} ({} that swing where they stand, {} load doors):",
        doors.len(),
        loaded.info.label(),
        doors.iter().filter(|d| d.teleport.is_none()).count(),
        doors.iter().filter(|d| d.teleport.is_some()).count()
    )?;
    let mut models: BTreeMap<String, DoorModel> = BTreeMap::new();
    for door in doors {
        let base = door.base;
        let name = order
            .get(base)
            .and_then(|r| r.record().ok())
            .and_then(|r| r.get(FULL).map(|s| esm::text::decode_cp1252(&s.data)))
            .unwrap_or_default();
        let [x, y, z] = door.position;
        writeln!(
            out,
            "\n  {} {} \"{}\" at {x:.0},{y:.0},{z:.0}, heading {:.0}°",
            door.form_id,
            describe_id(order, base),
            name.trim_end_matches('\0'),
            door.rotation[2].to_degrees()
        )?;
        match door.teleport {
            Some(t) => writeln!(
                out,
                "    a load door: leads to the far side of {} (no sequence plays; the game teleports)",
                t.door
            )?,
            None => {
                let state_now = world::doors::open_state(order, &state, door.form_id);
                writeln!(
                    out,
                    "    swings where it stands; starts {} (placed {}); GetOpenState {}",
                    if door.open_by_default { "open" } else { "closed" },
                    if world::doors::open_by_default(order, door.form_id) {
                        "open by default: XACT 8 / ONAM"
                    } else {
                        "without the open-by-default flag"
                    },
                    state_now as i32
                )?;
            }
        }
        if let Some(lock) = world::locks::placed_lock(order, door.form_id) {
            writeln!(
                out,
                "    locked: level {}{}",
                lock.level,
                lock.key
                    .map(|k| format!(", key {}", describe_id(order, k)))
                    .unwrap_or_default()
            )?;
        }
        let flags = world::doors::flags(order, base);
        let (open_sound, close_sound) = world::sound::door_sounds(order, base);
        writeln!(
            out,
            "    FNAM flags 0x{flags:02x} ({}); opening sound {}, closing sound {}",
            flag_words(flags),
            sound_name(order, open_sound),
            sound_name(order, close_sound)
        )?;
        if let Some(script) = world::scripting::script_of(order, door.form_id) {
            let blocks: Vec<String> = scripts
                .script(order, script)
                .map(|s| s.blocks.iter().map(|b| b.kind.clone()).collect())
                .unwrap_or_default();
            writeln!(
                out,
                "    script {}: blocks {}",
                describe_id(order, script),
                blocks.join(", ")
            )?;
        }
        let Some(model) = door.model.as_deref() else {
            writeln!(out, "    no model")?;
            continue;
        };
        let m = models
            .entry(model.to_ascii_lowercase())
            .or_insert_with(|| read_model(assets, model));
        writeln!(out, "    model {model}")?;
        if let Some(e) = &m.error {
            writeln!(out, "      can't be read: {e}")?;
            continue;
        }
        for s in &m.sequences {
            let turn = leaf_turn(&m.chain, s)
                .map(|(a, b)| format!("; turns the leaf from {a:.0}° to {b:.0}°"))
                .unwrap_or_default();
            writeln!(out, "      sequence {}{turn}", sequence_line(s))?;
        }
        if m.sequences.is_empty() {
            writeln!(out, "      no sequences: it flips at once")?;
        }
        for (chain, flags) in &m.leaves {
            writeln!(
                out,
                "      swinging collision on {chain} (collision object flags 0x{flags:x})"
            )?;
        }
        if m.leaves.is_empty() && door.teleport.is_none() {
            writeln!(
                out,
                "      no keyframed collision: nothing moves out of the way"
            )?;
        }
    }
    Ok(())
}

/// Every `DOOR` record: flags, sounds and models, counted.
fn survey(out: &mut impl Write, order: &LoadOrder, assets: &Assets) -> Result<(), CliError> {
    let mut by_flags: BTreeMap<u8, usize> = BTreeMap::new();
    let mut models: BTreeMap<String, DoorModel> = BTreeMap::new();
    let mut with_both = 0usize;
    let mut with_sound_keys = 0usize;
    let mut same_sound = 0usize;
    let mut records = 0usize;
    let mut flagged: Vec<String> = Vec::new();
    for rr in order.records_of_type(DOOR) {
        if rr.entry.header.is_deleted() {
            continue;
        }
        records += 1;
        let flags = world::doors::flags(order, rr.form_id);
        *by_flags.entry(flags).or_default() += 1;
        if flags != 0 {
            flagged.push(format!(
                "{} ({})",
                describe_id(order, rr.form_id),
                flag_words(flags)
            ));
        }
        let Ok(record) = rr.record() else { continue };
        let Some(model) = record
            .get(MODL)
            .map(|s| esm::text::decode_cp1252(&s.data))
            .map(|m| m.trim_end_matches('\0').to_string())
        else {
            continue;
        };
        let key = model.to_ascii_lowercase();
        let first = !models.contains_key(&key);
        let m = models
            .entry(key)
            .or_insert_with(|| read_model(assets, &model));
        if !first {
            continue;
        }
        let find = |name: &str| {
            m.sequences
                .iter()
                .find(|s| s.name.eq_ignore_ascii_case(name))
        };
        if let (Some(open), Some(close)) = (find("Open"), find("Close")) {
            with_both += 1;
            let key_sound = |s: &nif::Sequence| {
                s.text_keys.iter().find_map(|(_, k)| {
                    k.get(..6)
                        .filter(|h| h.eq_ignore_ascii_case("sound:"))
                        .map(|_| k[6..].trim().to_string())
                })
            };
            let (ko, kc) = (key_sound(open), key_sound(close));
            if ko.is_some() || kc.is_some() {
                with_sound_keys += 1;
            }
            let (so, sc) = world::sound::door_sounds(order, rr.form_id);
            let named = |id: Option<FormId>| {
                id.and_then(|i| order.get(i))
                    .and_then(|r| r.editor_id().ok().flatten())
            };
            if ko.is_some_and(|k| named(so).is_some_and(|n| n.eq_ignore_ascii_case(&k)))
                || kc.is_some_and(|k| named(sc).is_some_and(|n| n.eq_ignore_ascii_case(&k)))
            {
                same_sound += 1;
            }
        }
    }
    writeln!(
        out,
        "{records} DOOR records, {} distinct models",
        models.len()
    )?;
    writeln!(out, "FNAM flags:")?;
    for (flags, n) in &by_flags {
        writeln!(out, "  0x{flags:02x} ({}): {n}", flag_words(*flags))?;
    }
    if !flagged.is_empty() {
        writeln!(out, "  flagged: {}", flagged.join("; "))?;
    }
    writeln!(
        out,
        "Models with both Open and Close sequences: {with_both}; of those {with_sound_keys} name a \
         sound in a text key, {same_sound} the same sound as the record's SNAM/ANAM"
    )?;
    let unreadable: Vec<&String> = models
        .iter()
        .filter(|(_, m)| m.error.is_some())
        .map(|(p, _)| p)
        .collect();
    if !unreadable.is_empty() {
        writeln!(out, "Models that can't be read: {}", unreadable.len())?;
    }
    let mut lengths: BTreeMap<String, usize> = BTreeMap::new();
    for m in models.values() {
        if let Some(o) = m
            .sequences
            .iter()
            .find(|s| s.name.eq_ignore_ascii_case("Open"))
        {
            *lengths
                .entry(format!("{:.2} s", o.stop - o.start))
                .or_default() += 1;
        }
    }
    writeln!(out, "Open sequence lengths: {lengths:?}")?;
    let no_leaf = models
        .values()
        .filter(|m| m.error.is_none() && m.leaves.is_empty())
        .count();
    writeln!(
        out,
        "Models without keyframed collision (nothing swings out of the way): {no_leaf}"
    )?;
    Ok(())
}
