//! Canonical export of every shared domain as coop-core fields.
//!
//! The result is a `BTreeMap` keyed by [`Key`], so iteration order is
//! sorted and never depends on `HashMap` order. Default values are omitted
//! (a missing key and a tombstone mean the same thing).

use std::collections::BTreeMap;

use coop_core::ids::FormId as CForm;
use coop_core::schema::{Field, Key};
use esm::FormId;

use super::{quantize_clock, Domain};
use crate::dialogue::PLAYER_REF;
use crate::scripting::GameState;

/// Custom tags (sub-index of `Field::Custom`).
pub mod tag {
    /// `said` line.
    pub const SAID: u16 = 1;
    /// `talked_to` person.
    pub const TALKED: u16 = 2;
    /// `notes`.
    pub const NOTE: u16 = 3;
    /// `destroyed`.
    pub const DESTROYED: u16 = 4;
    /// `stocked` holder.
    pub const STOCKED: u16 = 5;
    /// `map_markers`.
    pub const MARKER: u16 = 6;
    /// `perks` (data: rank).
    pub const PERK: u16 = 7;
    /// Player level (data: u16 le), on `PLAYER_REF`.
    pub const LEVEL: u16 = 8;
    /// Reputation (data: fame f32 bits, infamy f32 bits).
    pub const REPUTATION: u16 = 9;
    /// A dead actor (not a player): in the shared hash, so a lost kill
    /// message is noticed and repaired.
    pub const DEAD: u16 = 10;
    /// A faction's relations (`SetEnemy`, `SetAlly`), on the faction (data:
    /// other faction u32 le and reaction u8, per entry, sorted).
    pub const RELATION: u16 = 11;
    /// A faction's crime, on the faction (data: minor u32 le, major u32 le,
    /// holds the player as an enemy u8).
    pub const CRIME: u16 = 12;
    /// The player's crime, on `PLAYER_REF` (data: minor u32 le, major u32
    /// le, steal warnings u32 le, murderer u8).
    pub const PLAYER_CRIME: u16 = 13;
    /// Done stages of a quest: `STAGES_BASE + stage / STAGES_CHUNK`.
    pub const STAGES_BASE: u16 = 0x1000;
    /// Stages per chunk (2 bytes each, well under the 512 byte limit).
    pub const STAGES_CHUNK: u16 = 200;
    /// Faction members: `FACTION_BASE + (person & 0xFF)`.
    pub const FACTION_BASE: u16 = 0x2000;
    /// Entries kept per faction chunk (5 bytes each).
    pub const FACTION_MAX: usize = 100;
}

/// Quest flag bits.
pub mod flag {
    /// In `running`.
    pub const RUNNING: u32 = 1;
    /// In `completed`.
    pub const COMPLETED: u32 = 2;
    /// In `failed`.
    pub const FAILED: u32 = 4;
    /// In `quests_announced`.
    pub const ANNOUNCED: u32 = 8;
}

/// The exported fields.
pub type Fields = BTreeMap<Key, Field>;

/// Converts a world form id.
pub fn cf(f: FormId) -> CForm {
    CForm(f.0)
}

/// Converts back.
pub fn wf(f: CForm) -> FormId {
    FormId(f.0)
}

fn custom(tag: u16, data: Vec<u8>) -> Field {
    Field::Custom { tag, data }
}

/// True when the field is the default ("absent") value of its slot.
pub fn is_tombstone(f: &Field) -> bool {
    match f {
        Field::QuestFlags { flags } => *flags == 0,
        Field::Objective { state, .. } => *state == 0,
        Field::Topic { known } => !known,
        Field::ItemCount { count, .. } => *count == 0,
        Field::ScriptVar { bits, .. } => *bits == 0,
        Field::Karma { value } | Field::ActorValue { value, .. } => *value == 0,
        Field::Custom { data, .. } => data.is_empty(),
        _ => false,
    }
}

/// The default-valued field a tombstone write uses for the slot of `f`.
pub fn tombstone_of(f: &Field) -> Field {
    match f {
        Field::QuestFlags { .. } => Field::QuestFlags { flags: 0 },
        Field::Objective { index, .. } => Field::Objective {
            index: *index,
            state: 0,
        },
        Field::Topic { .. } => Field::Topic { known: false },
        Field::ItemCount { item, .. } => Field::ItemCount {
            item: *item,
            count: 0,
        },
        Field::ScriptVar { index, .. } => Field::ScriptVar {
            index: *index,
            bits: 0,
        },
        Field::Karma { .. } => Field::Karma { value: 0 },
        Field::ActorValue { av, .. } => Field::ActorValue { av: *av, value: 0 },
        Field::Custom { tag, .. } => Field::Custom {
            tag: *tag,
            data: Vec::new(),
        },
        other => other.clone(),
    }
}

/// Which domain a key belongs to.
pub fn domain_of(k: &Key) -> Domain {
    if super::shadow::slot_of_shadow_form(wf(k.form)).is_some() {
        return Domain::Shadow;
    }
    match k.kind {
        0 => Domain::Stage,
        1 => Domain::Flags,
        2 => Domain::Objective,
        3 => Domain::Global,
        4 => Domain::Topic,
        5 => Domain::Items,
        6 => Domain::Disabled,
        7 => Domain::Vars,
        8 => Domain::Karma,
        12 => Domain::Experience,
        _ => match k.sub as u16 {
            tag::SAID => Domain::Said,
            tag::TALKED => Domain::TalkedTo,
            tag::NOTE => Domain::Note,
            tag::DESTROYED | tag::DEAD => Domain::Destroyed,
            tag::STOCKED => Domain::Stocked,
            tag::MARKER => Domain::MapMarker,
            tag::PERK => Domain::Perk,
            tag::LEVEL => Domain::Level,
            tag::REPUTATION => Domain::Reputation,
            tag::RELATION => Domain::FactionRelation,
            tag::CRIME | tag::PLAYER_CRIME => Domain::Crime,
            t if t >= tag::FACTION_BASE => Domain::Faction,
            _ => Domain::StagesDone,
        },
    }
}

/// Every shared field of `gs`, sorted by key.
pub fn export(gs: &GameState) -> Fields {
    let mut m = Fields::new();
    let mut put = |form: FormId, f: Field| {
        if !is_tombstone(&f) {
            m.insert(f.key(cf(form)), f);
        }
    };
    let plan = gs.coop.bank.plan.as_ref();
    let own = |q: &FormId| plan.is_some_and(|p| p.quests.contains(q));
    let shadowed_var = |owner: &FormId, name: &str| {
        plan.and_then(|p| p.quest_vars.get(owner))
            .is_some_and(|v| v.iter().any(|n| n == name))
    };
    for (q, s) in gs.stages.iter().filter(|(q, _)| !own(q)) {
        put(*q, Field::QuestStage { stage: *s });
    }
    let mut done: BTreeMap<(FormId, u16), Vec<u16>> = BTreeMap::new();
    for (q, s) in gs.stages_done.iter().filter(|(q, _)| !own(q)) {
        done.entry((*q, *s / tag::STAGES_CHUNK))
            .or_default()
            .push(*s);
    }
    for ((q, chunk), mut stages) in done {
        stages.sort_unstable();
        let data = stages.iter().flat_map(|s| s.to_le_bytes()).collect();
        put(q, custom(tag::STAGES_BASE + chunk, data));
    }
    let quests = gs
        .running
        .iter()
        .chain(&gs.completed)
        .chain(&gs.failed)
        .chain(&gs.quests_announced);
    for q in quests.filter(|q| !own(q)) {
        let bits = [
            (gs.running.contains(q), flag::RUNNING),
            (gs.completed.contains(q), flag::COMPLETED),
            (gs.failed.contains(q), flag::FAILED),
            (gs.quests_announced.contains(q), flag::ANNOUNCED),
        ]
        .iter()
        .filter(|(on, _)| *on)
        .fold(0, |a, (_, b)| a | b);
        put(*q, Field::QuestFlags { flags: bits });
    }
    for ((q, i), done) in gs.objectives.iter().filter(|((q, _), _)| !own(q)) {
        if let Ok(index) = u16::try_from(*i) {
            put(
                *q,
                Field::Objective {
                    index,
                    state: if *done { 3 } else { 1 },
                },
            );
        }
    }
    for (q, i) in gs
        .set_by_scripts
        .hidden_completed
        .iter()
        .filter(|(q, _)| !own(q))
    {
        if let Ok(index) = u16::try_from(*i) {
            put(*q, Field::Objective { index, state: 2 });
        }
    }
    for (g, v) in &gs.globals {
        let v = gs.coop_clock_exported(*g, *v).unwrap_or_else(|| {
            if gs.coop.clock.contains(g) {
                quantize_clock(*v)
            } else {
                *v
            }
        });
        put(*g, Field::global(v));
    }
    for t in &gs.topics {
        put(*t, Field::Topic { known: true });
    }
    for (set, t) in [
        (&gs.said, tag::SAID),
        (&gs.talked_to, tag::TALKED),
        (&gs.notes, tag::NOTE),
        (&gs.destroyed, tag::DESTROYED),
        (&gs.stocked, tag::STOCKED),
    ] {
        for f in set
            .iter()
            .filter(|f| !(t == tag::STOCKED && plan.is_some() && **f == PLAYER_REF))
        {
            put(*f, custom(t, vec![1]));
        }
    }
    // a map marker: bit 1 revealed (`ShowMap`), bit 2 found (`discovered`,
    // which unlocks fast travel whatever the marker's own flags say)
    let mut marks: BTreeMap<FormId, u8> = BTreeMap::new();
    for m in &gs.map_markers {
        *marks.entry(*m).or_default() |= 1;
    }
    for m in &gs.discovered {
        *marks.entry(*m).or_default() |= 2;
    }
    for (m, bits) in marks {
        put(m, custom(tag::MARKER, vec![bits]));
    }
    export_factions_crime(gs, &mut put);
    for f in gs
        .dead
        .iter()
        .filter(|f| **f != PLAYER_REF && super::facts::player_of(**f).is_none())
    {
        put(*f, custom(tag::DEAD, vec![1]));
    }
    for (r, d) in &gs.disabled {
        put(*r, Field::RefDisabled { disabled: *d });
    }
    for ((holder, item), n) in gs
        .items
        .iter()
        .filter(|((h, _), _)| !(plan.is_some() && *h == PLAYER_REF))
    {
        put(
            *holder,
            Field::ItemCount {
                item: cf(*item),
                count: *n,
            },
        );
    }
    // a quest that each player runs alone (`own`) keeps its script variables local too: each
    // machine's copy of the script writes them (a timer reads 10.9 on one and 9.9 on another)
    for (owner, locals) in gs.variables.iter().filter(|(q, _)| !own(q)) {
        for (i, (name, _, v)) in locals.iter().enumerate() {
            if shadowed_var(owner, name) {
                continue;
            }
            if let Ok(index) = u16::try_from(i) {
                put(*owner, Field::script_var(index, v));
            }
        }
    }
    let av = |id: u16| {
        gs.actor_values
            .get(&(PLAYER_REF, id))
            .copied()
            .unwrap_or(0.0)
    };
    put(
        PLAYER_REF,
        Field::Karma {
            value: av(crate::reputation::KARMA) as i32,
        },
    );
    let (xp_own, perks_own) = plan.map_or((false, false), |p| (p.level_xp, p.perks));
    if !xp_own {
        put(
            PLAYER_REF,
            Field::ActorValue {
                av: crate::experience::XP,
                value: av(crate::experience::XP) as i32,
            },
        );
        put(
            PLAYER_REF,
            custom(tag::LEVEL, gs.player_level.to_le_bytes().to_vec()),
        );
    }
    for p in gs.perks.iter().filter(|_| !perks_own) {
        let rank = gs.perk_ranks.get(p).copied().unwrap_or(1);
        put(*p, custom(tag::PERK, vec![rank]));
    }
    for (r, (fame, infamy)) in &gs.reputations {
        if *fame != 0.0 || *infamy != 0.0 {
            let mut d = fame.to_bits().to_le_bytes().to_vec();
            d.extend(infamy.to_bits().to_le_bytes());
            put(*r, custom(tag::REPUTATION, d));
        }
    }
    let mut fac: BTreeMap<(FormId, u16), Vec<(u32, i8)>> = BTreeMap::new();
    for ((faction, who), rank) in &gs.faction_changes {
        fac.entry((*faction, tag::FACTION_BASE + (who.0 & 0xFF) as u16))
            .or_default()
            .push((who.0, *rank));
    }
    for ((faction, t), mut members) in fac {
        members.sort_unstable();
        members.truncate(tag::FACTION_MAX);
        let mut d = Vec::new();
        for (w, r) in members {
            d.extend(w.to_le_bytes());
            d.push(r as u8);
        }
        put(faction, custom(t, d));
    }
    super::shadow::export_into(gs, &mut m);
    m
}

/// Faction relations and crime: the party's, one value on every peer.
fn export_factions_crime(gs: &GameState, put: &mut impl FnMut(FormId, Field)) {
    let mut rel: BTreeMap<FormId, Vec<(u32, u8)>> = BTreeMap::new();
    for ((a, b), r) in &gs.faction_relations {
        rel.entry(*a).or_default().push((b.0, *r));
    }
    for (a, mut others) in rel {
        others.sort_unstable();
        others.truncate(tag::FACTION_MAX);
        let mut d = Vec::new();
        for (b, r) in others {
            d.extend(b.to_le_bytes());
            d.push(r);
        }
        put(a, custom(tag::RELATION, d));
    }
    let factions: std::collections::BTreeSet<FormId> = gs
        .faction_crimes
        .keys()
        .chain(&gs.crime_enemies)
        .copied()
        .collect();
    for f in factions {
        let (minor, major) = gs.faction_crimes.get(&f).copied().unwrap_or((0, 0));
        let enemy = gs.crime_enemies.contains(&f);
        if minor == 0 && major == 0 && !enemy {
            continue;
        }
        let mut d = minor.to_le_bytes().to_vec();
        d.extend(major.to_le_bytes());
        d.push(u8::from(enemy));
        put(f, custom(tag::CRIME, d));
    }
    let (minor, major) = gs.player_crimes;
    if minor != 0 || major != 0 || gs.steal_warnings != 0 || gs.player_murderer {
        let mut d = minor.to_le_bytes().to_vec();
        d.extend(major.to_le_bytes());
        d.extend(gs.steal_warnings.to_le_bytes());
        d.push(u8::from(gs.player_murderer));
        put(PLAYER_REF, custom(tag::PLAYER_CRIME, d));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_is_order_independent_and_skips_defaults() {
        let mut a = GameState::default();
        let mut b = GameState::default();
        for i in 1..50u32 {
            a.stages.insert(FormId(i), i as u16);
            a.topics.insert(FormId(i + 1000));
        }
        for i in (1..50u32).rev() {
            b.stages.insert(FormId(i), i as u16);
            b.topics.insert(FormId(i + 1000));
        }
        assert_eq!(export(&a), export(&b));
        a.items.insert((FormId(1), FormId(2)), 0);
        assert_eq!(export(&a), export(&b));
        a.items.insert((FormId(1), FormId(2)), 3);
        assert_ne!(export(&a), export(&b));
    }

    #[test]
    fn factions_crime_and_found_markers_round_trip() {
        let mut a = GameState::default();
        a.faction_relations.insert((FormId(5), FormId(6)), 1);
        a.faction_relations.insert((FormId(6), FormId(5)), 2);
        a.faction_crimes.insert(FormId(7), (1, 2));
        a.crime_enemies.insert(FormId(8));
        a.player_crimes = (3, 4);
        a.steal_warnings = 1;
        a.player_murderer = true;
        a.map_markers.insert(FormId(9));
        a.discovered.insert(FormId(10));
        let fields = export(&a);
        let ds: Vec<Domain> = fields.keys().map(domain_of).collect();
        assert!(ds.contains(&Domain::FactionRelation));
        assert!(ds.contains(&Domain::Crime));
        let mut b = GameState::default();
        super::super::apply::reconcile(&mut b, &fields, Default::default());
        assert_eq!(export(&b), fields);
        assert_eq!(b.faction_relations, a.faction_relations);
        assert_eq!(b.faction_crimes.get(&FormId(7)), Some(&(1, 2)));
        assert!(b.crime_enemies.contains(&FormId(8)));
        assert_eq!((b.player_crimes, b.steal_warnings), ((3, 4), 1));
        assert!(b.player_murderer);
        // revealed only, found only
        assert!(b.map_markers.contains(&FormId(9)) && !b.discovered.contains(&FormId(9)));
        assert!(b.discovered.contains(&FormId(10)) && !b.map_markers.contains(&FormId(10)));
        // back to nothing: every slot is cleared
        super::super::apply::reconcile(&mut b, &Fields::new(), Default::default());
        assert!(b.faction_relations.is_empty() && b.faction_crimes.is_empty());
        assert!(b.crime_enemies.is_empty() && b.discovered.is_empty());
        assert_eq!((b.player_crimes, b.player_murderer), ((0, 0), false));
    }

    #[test]
    fn domains_resolve() {
        let mut s = GameState::default();
        s.said.insert(FormId(9));
        s.stages_done.insert((FormId(1), 10));
        s.faction_changes.insert((FormId(5), FormId(6)), 1);
        let ds: Vec<Domain> = export(&s).keys().map(domain_of).collect();
        assert!(ds.contains(&Domain::Said));
        assert!(ds.contains(&Domain::StagesDone));
        assert!(ds.contains(&Domain::Faction));
    }
}
