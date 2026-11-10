//! The state dump that the test harness (`tools/coop-run`) reads.
//!
//! One JSON object (schema 1). The sections of shared state have the keys of the older
//! program's dump (`coop-session/src/dump.rs`): `quests.<id>.stage/running/completed`,
//! `player_quests`, `globals`, `positions` and `hash.shared`. A key is an editor id where the
//! form has one, else the form id as 8 hex digits. So one comparison reads each program.
//!
//! The sections that only the older program writes are named in `only_world`. The sections
//! `gate`, `intents` and `session` (the connection's state and counters) are this program's
//! own.
//!
//! `hash.steady` is this program's own too: the hash of the shared state without the globals
//! that the game moves by itself (the session's volatile keys, the game's clock). A guest's
//! game advances its own clock between the host's values, so `hash.shared` of a guest differs
//! from the host's root until the clock of a guest follows the host's clock only. Until then
//! `hash.steady` shows that each other slot is equal.

use std::collections::BTreeMap;

use coop_core::hash::ShardHashes;
use coop_core::ids::FormId;
use coop_core::schema::{flag, Field};
use coop_engine::gate::Counters;
use coop_game::Names;
use coop_session::json::esc;

/// The sections of the older program's dump that this program does not write yet.
pub const ONLY_WORLD: &[&str] = &[
    "items",
    "disabled",
    "dead",
    "live",
    "interact",
    "objects",
    "campaign",
    "combat",
    "start",
    "status",
    "topics",
    "notes",
    "map_markers",
    "karma",
    "reputations",
    "factions",
    "faction_relations",
    "crime",
    "rules",
];

/// What one dump shows.
pub struct View<'a> {
    /// This program is the host.
    pub host: bool,
    /// The slot of this program's player (0: the host).
    pub slot: u8,
    /// Seconds since the program started.
    pub t: f64,
    /// The number of this dump.
    pub seq: u64,
    /// The game is in play and the session is ready.
    pub ready: bool,
    /// The game's shared state ([`coop_game::Shared::export_shared`]).
    pub export: &'a [(FormId, Field)],
    /// The names of the game's data.
    pub names: &'a dyn Names,
    /// The cell of this program's player.
    pub cell: Option<FormId>,
    /// The feet of this program's player.
    pub place: Option<[f32; 3]>,
    /// The revision of the shared state that the game holds (`None`: not known).
    pub rev: Option<u64>,
    /// A host's last revisions with their roots, newest first.
    pub recent: Vec<(u64, u64)>,
    /// What the guest gate counted (`None`: this game has no gate).
    pub gate: Option<Counters>,
    /// The intents that went to the host, and those that had no command.
    pub intents: (u64, u64),
    /// The session's own status, as JSON (an object).
    pub session: String,
}

/// The key of a form: its editor id, else 8 hex digits.
fn key(names: &dyn Names, form: FormId) -> String {
    names
        .name_of(form)
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| format!("{:08X}", form.0))
}

/// A float as JSON: a value that is not finite becomes 0.
fn num(v: f32) -> String {
    if v.is_finite() {
        format!("{v}")
    } else {
        "0".into()
    }
}

/// An object from keys and values that are JSON already.
fn object(m: &BTreeMap<String, String>) -> String {
    let parts: Vec<String> = m
        .iter()
        .map(|(k, v)| format!("\"{}\":{v}", esc(k)))
        .collect();
    format!("{{{}}}", parts.join(","))
}

/// The key of a slot's rows: `host`, `guest`, then `guest2` and later.
pub fn slot_key(slot: u8) -> String {
    match slot {
        0 => "host".into(),
        1 => "guest".into(),
        n => format!("guest{n}"),
    }
}

/// The hash of a game's shared state: equal on two games that agree.
pub fn shared_root(export: &[(FormId, Field)]) -> u64 {
    ShardHashes::of_fields(export.iter().map(|(form, field)| (*form, field))).root()
}

/// The hash of a game's shared state without the globals that the game moves by itself: the
/// session's volatile keys ([`coop_core::compat::VOLATILE_KEYS`]), by their editor ids.
pub fn steady_root(export: &[(FormId, Field)], names: &dyn Names) -> u64 {
    let moving: Vec<FormId> = coop_core::compat::VOLATILE_KEYS
        .iter()
        .filter_map(|key| names.form_of(key))
        .collect();
    ShardHashes::of_fields(
        export
            .iter()
            .filter(|(form, _)| !moving.contains(form))
            .map(|(form, field)| (*form, field)),
    )
    .root()
}

/// The dump as one line of JSON.
pub fn state_json(v: &View<'_>) -> String {
    // (stage, running, completed) for each quest that has a stage or a flag.
    let mut quests: BTreeMap<String, (u16, bool, bool)> = BTreeMap::new();
    let mut globals: BTreeMap<String, String> = BTreeMap::new();
    for (form, field) in v.export {
        match field {
            Field::QuestStage { stage } => {
                quests.entry(key(v.names, *form)).or_default().0 = *stage
            }
            Field::QuestFlags { flags } => {
                let quest = quests.entry(key(v.names, *form)).or_default();
                quest.1 = flags & flag::RUNNING != 0;
                quest.2 = flags & flag::COMPLETED != 0;
            }
            Field::Global { bits } => {
                globals.insert(key(v.names, *form), num(f32::from_bits(*bits)));
            }
            _ => {}
        }
    }
    let quests: BTreeMap<String, String> = quests
        .into_iter()
        .map(|(k, (stage, running, completed))| {
            (
                k,
                format!("{{\"stage\":{stage},\"running\":{running},\"completed\":{completed}}}"),
            )
        })
        .collect();
    let mut positions: BTreeMap<String, String> = BTreeMap::new();
    if let Some([x, y, z]) = v.place {
        positions.insert(
            slot_key(v.slot),
            format!("[{},{},{}]", num(x), num(y), num(z)),
        );
    }
    let rev = v.rev.map(|r| format!(",\"rev\":{r}")).unwrap_or_default();
    let recent: Vec<String> = v
        .recent
        .iter()
        .map(|(r, h)| format!("[{r},\"{h:016x}\"]"))
        .collect();
    let recent = if recent.is_empty() {
        String::new()
    } else {
        format!(",\"recent\":[{}]", recent.join(","))
    };
    let gate = match v.gate {
        Some(c) => format!(
            "{{\"runs\":{},\"stopped\":{},\"guarded\":{},\"last_script\":\"{:08X}\",\"quiet_ms\":{}}}",
            c.runs, c.stopped, c.guarded, c.last_script, c.quiet_ms
        ),
        None => "null".into(),
    };
    let only: Vec<String> = ONLY_WORLD.iter().map(|s| format!("\"{s}\"")).collect();
    format!(
        "{{\"schema\":1,\"program\":\"nv-coop\",\"role\":\"{}\",\"t\":{:.3},\"seq\":{},\"ready\":{},\
         \"quests\":{},\"player_quests\":{{}},\"globals\":{},\"positions\":{},\"cell\":\"{}\",\
         \"hash\":{{\"shared\":\"{:016x}\",\"steady\":\"{:016x}\"{rev}{recent}}},\"gate\":{gate},\
         \"intents\":{{\"sent\":{},\"open\":{}}},\"session\":{},\"only_world\":[{}]}}",
        if v.host { "host" } else { "guest" },
        v.t,
        v.seq,
        v.ready,
        object(&quests),
        object(&globals),
        object(&positions),
        esc(&v.cell.map(|c| key(v.names, c)).unwrap_or_default()),
        shared_root(v.export),
        steady_root(v.export, v.names),
        v.intents.0,
        v.intents.1,
        v.session,
        only.join(",")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use coop_engine::game::NameTable;

    const QUEST: FormId = FormId(0x0015_d912);
    const NO_NAME: FormId = FormId(0x0010_4c1c);
    const HOUR: FormId = FormId(0x38);

    fn names() -> NameTable {
        NameTable::parse(
            "P 00 FalloutNV.esm\nF QUST 0015D912 VCG03\nF QUST 00104C1C\nF GLOB 00000038 GameHour\n\
             F CELL 00103DF9 GSDocMitchellHouse\n",
        )
        .unwrap()
    }

    fn export() -> Vec<(FormId, Field)> {
        vec![
            (QUEST, Field::QuestStage { stage: 10 }),
            (
                QUEST,
                Field::QuestFlags {
                    flags: flag::RUNNING,
                },
            ),
            (
                NO_NAME,
                Field::QuestFlags {
                    flags: flag::RUNNING | flag::COMPLETED,
                },
            ),
            (HOUR, Field::global(8.5)),
        ]
    }

    fn view<'a>(export: &'a [(FormId, Field)], names: &'a NameTable) -> View<'a> {
        View {
            host: false,
            slot: 1,
            t: 1.5,
            seq: 3,
            ready: true,
            export,
            names,
            cell: Some(FormId(0x0010_3df9)),
            place: Some([1.0, 2.5, -3.0]),
            rev: Some(7),
            recent: Vec::new(),
            gate: Some(Counters {
                runs: 9,
                stopped: 2,
                guarded: 1,
                last_script: 0x0010_4c02,
                quiet_ms: 16,
            }),
            intents: (1, 0),
            session: "{}".into(),
        }
    }

    #[test]
    fn the_dump_has_the_shared_keys_of_the_contract() {
        let names = names();
        let export = export();
        let text = state_json(&view(&export, &names));
        for part in [
            "\"schema\":1",
            "\"role\":\"guest\"",
            "\"seq\":3",
            "\"quests\":{\"00104C1C\":{\"stage\":0,\"running\":true,\"completed\":true},\
             \"VCG03\":{\"stage\":10,\"running\":true,\"completed\":false}}",
            "\"globals\":{\"GameHour\":8.5}",
            "\"positions\":{\"guest\":[1,2.5,-3]}",
            "\"cell\":\"GSDocMitchellHouse\"",
            "\"rev\":7",
            ",\"steady\":\"",
            "\"gate\":{\"runs\":9,\"stopped\":2,\"guarded\":1,\"last_script\":\"00104C02\",\"quiet_ms\":16}",
            "\"intents\":{\"sent\":1,\"open\":0}",
            "\"only_world\":[\"items\"",
        ] {
            assert!(text.contains(part), "{part} is not in {text}");
        }
        assert!(!text.contains('\n'));
    }

    /// The hash is equal for two games that agree, in each order of the list, and it
    /// differs when one slot differs.
    #[test]
    fn the_hash_follows_the_state_and_not_the_order() {
        let a = export();
        let mut b = export();
        b.reverse();
        assert_eq!(shared_root(&a), shared_root(&b));
        b[0] = (HOUR, Field::global(8.75));
        assert_ne!(shared_root(&a), shared_root(&b));
        // The steady hash does not follow the game's clock, and it follows each other slot.
        let names = names();
        assert_eq!(steady_root(&a, &names), steady_root(&b, &names));
        let mut c = a.clone();
        c[0] = (QUEST, Field::QuestStage { stage: 20 });
        assert_ne!(steady_root(&a, &names), steady_root(&c, &names));
        // A host's dump names its role, has no gate, and gives its last roots.
        let mut host = view(&a, &names);
        host.host = true;
        host.slot = 0;
        host.gate = None;
        host.recent = vec![(7, 0xabc), (6, 0xdef)];
        let text = state_json(&host);
        assert!(text.contains("\"gate\":null"), "{text}");
        assert!(text.contains("\"positions\":{\"host\":"), "{text}");
        assert!(
            text.contains(",\"recent\":[[7,\"0000000000000abc\"],[6,\"0000000000000def\"]]"),
            "{text}"
        );
    }
}
