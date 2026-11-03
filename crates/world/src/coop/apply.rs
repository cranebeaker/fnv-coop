//! Applying host fields to a guest `GameState`.
//!
//! Writes the state directly: no script runs, no leveled list is rolled, no
//! result script fires. The only side effects are local presentation events
//! (journal text, "quest added" announcements, objective notices), and only
//! when `Opts::present` is set (not for a join snapshot).

use std::collections::BTreeSet;

use coop_core::schema::{Field, Key};
use esm::{FormId, LoadOrder};
use script::interp::Locals;
use script::VarKind;

use super::export::{flag, tag, wf, Fields};
use crate::dialogue::PLAYER_REF;
use crate::scripting::{Event, Facts, GameState, ScriptCache};

/// Game data for looking up presentation text.
#[derive(Clone, Copy)]
pub struct Ctx<'a> {
    /// The load order.
    pub order: &'a LoadOrder,
    /// Parsed scripts and quests.
    pub scripts: &'a ScriptCache,
}

/// How to apply.
#[derive(Clone, Copy, Default)]
pub struct Opts<'a> {
    /// Queue local presentation events (announcements, journal lines).
    pub present: bool,
    /// Game data: gives journal/objective text and real variable names.
    pub ctx: Option<Ctx<'a>>,
}

/// One change: `None` removes the slot (back to its default).
pub type Change = (Key, Option<Field>);

/// What an apply did.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ApplyReport {
    /// Slots set.
    pub set: u32,
    /// Slots cleared.
    pub cleared: u32,
    /// Quest announcements queued.
    pub announced: u32,
}

/// Applies `changes` to `gs`.
pub fn apply_changes(gs: &mut GameState, changes: &[Change], opts: Opts) -> ApplyReport {
    let mut r = ApplyReport::default();
    let mut announce: BTreeSet<FormId> = BTreeSet::new();
    let mut journal: Vec<(FormId, u16)> = Vec::new();
    let mut objectives: Vec<(FormId, i32, bool)> = Vec::new();
    if changes
        .iter()
        .any(|(k, _)| super::shadow::slot_of_shadow_form(wf(k.form)).is_some())
    {
        // a variable the host holds per player has no shared row, so this machine may not
        // have the quest's variables at all yet: lay them out as the script declares them
        let quests: Vec<FormId> = gs
            .coop
            .bank
            .plan
            .as_ref()
            .map(|p| p.quest_vars.keys().copied().collect())
            .unwrap_or_default();
        for q in quests {
            ensure_locals(gs, q, opts.ctx);
        }
    }
    super::shadow::apply_changes(gs, changes);
    for (key, field) in changes {
        let form = wf(key.form);
        if super::shadow::slot_of_shadow_form(form).is_some() {
            continue;
        }
        match field {
            Some(_) => r.set += 1,
            None => r.cleared += 1,
        }
        apply_one(
            gs,
            form,
            key,
            field.as_ref(),
            opts,
            &mut announce,
            &mut journal,
            &mut objectives,
        );
    }
    if let (Some(c), true) = (opts.ctx, gs.coop.campaign.unnamed_vars) {
        super::campaign::names::heal(c, gs);
    }
    if opts.present {
        for (q, s) in journal {
            if let Some(text) = opts.ctx.and_then(|c| journal_text(c, gs, q, s)) {
                gs.events.push(Event::Journal { quest: q, text });
            }
        }
        for (q, i, completed) in objectives {
            let text = opts
                .ctx
                .and_then(|c| c.scripts.quest(c.order, q))
                .and_then(|quest| quest.objective(i).map(|o| o.text.clone()))
                .unwrap_or_default();
            gs.events.push(Event::Objective {
                quest: q,
                text,
                completed,
            });
        }
        for q in announce {
            crate::quest_text::announce(gs, q);
            r.announced += 1;
        }
    }
    r
}

/// Makes the shared domains of `gs` equal `want` (level triggered: whatever
/// differs is set, whatever is extra is cleared).
pub fn reconcile(gs: &mut GameState, want: &Fields, opts: Opts) -> ApplyReport {
    let have = super::export::export(gs);
    let mut changes: Vec<Change> = Vec::new();
    for (k, f) in want {
        if have.get(k) != Some(f) {
            changes.push((*k, Some(f.clone())));
        }
    }
    for k in have.keys() {
        if !want.contains_key(k) {
            changes.push((*k, None));
        }
    }
    apply_changes(gs, &changes, opts)
}

fn journal_text(c: Ctx, gs: &GameState, q: FormId, stage: u16) -> Option<String> {
    let quest = c.scripts.quest(c.order, q)?;
    let st = quest.stage(stage)?;
    let facts = Facts {
        order: c.order,
        state: gs,
        speaker: None,
    };
    st.entries
        .iter()
        .find(|e| facts.conditions_pass(&e.conditions, PLAYER_REF, PLAYER_REF))
        .and_then(|e| e.text.clone())
}

fn set_in<T: std::hash::Hash + Eq>(set: &mut std::collections::HashSet<T>, v: T, on: bool) {
    if on {
        set.insert(v);
    } else {
        set.remove(&v);
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_one(
    gs: &mut GameState,
    form: FormId,
    key: &Key,
    field: Option<&Field>,
    opts: Opts,
    announce: &mut BTreeSet<FormId>,
    journal: &mut Vec<(FormId, u16)>,
    objectives: &mut Vec<(FormId, i32, bool)>,
) {
    match (key.kind, field) {
        (0, Some(Field::QuestStage { stage })) => {
            if gs.stages.insert(form, *stage) != Some(*stage) {
                journal.push((form, *stage));
            }
        }
        (0, _) => {
            gs.stages.remove(&form);
        }
        (1, f) => {
            let bits = match f {
                Some(Field::QuestFlags { flags }) => *flags,
                _ => 0,
            };
            let had = |gs: &GameState, b: u32| match b {
                flag::RUNNING => gs.running.contains(&form),
                flag::COMPLETED => gs.completed.contains(&form),
                flag::FAILED => gs.failed.contains(&form),
                _ => gs.quests_announced.contains(&form),
            };
            for b in [
                flag::RUNNING,
                flag::COMPLETED,
                flag::FAILED,
                flag::ANNOUNCED,
            ] {
                let on = bits & b != 0;
                if on && !had(gs, b) && b != flag::RUNNING {
                    announce.insert(form);
                }
                match b {
                    flag::RUNNING => set_in(&mut gs.running, form, on),
                    flag::COMPLETED => set_in(&mut gs.completed, form, on),
                    flag::FAILED => set_in(&mut gs.failed, form, on),
                    _ => set_in(&mut gs.quests_announced, form, on),
                }
            }
        }
        (2, f) => {
            let index = key.sub as i32;
            let k = (form, index);
            let state = match f {
                Some(Field::Objective { state, .. }) => *state,
                _ => 0,
            };
            let before = gs.objectives.get(&k).copied();
            gs.objectives.remove(&k);
            gs.set_by_scripts.hidden_completed.remove(&k);
            match state {
                1 => {
                    gs.objectives.insert(k, false);
                }
                2 => {
                    gs.set_by_scripts.hidden_completed.insert(k);
                }
                3 => {
                    gs.objectives.insert(k, true);
                }
                _ => {}
            }
            if state == 1 && before.is_none() {
                objectives.push((form, index, false));
            } else if state == 3 && before != Some(true) {
                // (Updates batch: shown and completed in one go reads as done.)
                objectives.push((form, index, true));
            }
        }
        (3, Some(Field::Global { bits })) => {
            // a guest's clock follows the host's smoothly (`scene::follow_clock`)
            let v = f32::from_bits(*bits);
            if !gs.coop_clock_applied(form, v) {
                gs.globals.insert(form, v);
            }
        }
        (4, f) => set_in(
            &mut gs.topics,
            form,
            matches!(f, Some(Field::Topic { known: true })),
        ),
        (5, f) => {
            let item = esm::FormId(key.sub);
            match f {
                Some(Field::ItemCount { count, .. }) if *count != 0 => {
                    gs.items.insert((form, item), *count);
                }
                _ => {
                    gs.items.remove(&(form, item));
                }
            }
        }
        (6, Some(Field::RefDisabled { disabled })) => {
            gs.disabled.insert(form, *disabled);
        }
        (6, _) => {
            gs.disabled.remove(&form);
        }
        (7, f) => {
            let bits = match f {
                Some(Field::ScriptVar { bits, .. }) => *bits,
                _ => 0,
            };
            set_var(gs, form, key.sub as usize, f64::from_bits(bits), opts.ctx);
        }
        (8, f) => av(gs, PLAYER_REF, crate::reputation::KARMA, f),
        (12, f) => av(gs, form, key.sub as u16, f),
        (255, f) => custom(gs, form, key.sub as u16, f),
        _ => {}
    }
}

fn av(gs: &mut GameState, who: FormId, id: u16, f: Option<&Field>) {
    match f {
        Some(Field::Karma { value }) | Some(Field::ActorValue { value, .. }) if *value != 0 => {
            gs.actor_values.insert((who, id), f64::from(*value));
        }
        _ => {
            gs.actor_values.remove(&(who, id));
        }
    }
}

/// Makes sure `owner` has its script's variables (all zero) when the script is known.
fn ensure_locals(gs: &mut GameState, owner: FormId, ctx: Option<Ctx>) {
    if gs.variables.contains_key(&owner) {
        return;
    }
    let base = ctx.and_then(|c| {
        let s = crate::scripting::script_of(c.order, owner)?;
        c.scripts.script(c.order, s).map(|s| Locals::new(&s))
    });
    if let Some(base) = base {
        gs.variables.insert(owner, base);
    }
}

fn set_var(gs: &mut GameState, owner: FormId, index: usize, value: f64, ctx: Option<Ctx>) {
    if !gs.variables.contains_key(&owner) {
        if value == 0.0 {
            return;
        }
        ensure_locals(gs, owner, ctx);
        gs.variables.entry(owner).or_default();
    }
    let Some(locals) = gs.variables.get_mut(&owner) else {
        return;
    };
    let len = locals.iter().count();
    for i in len..=index {
        locals.insert(&format!("coopvar{i}"), VarKind::Float, 0.0);
    }
    let invented = len <= index;
    let slot = locals.iter().nth(index).map(|(n, k, _)| (n.to_string(), k));
    if let Some((name, kind)) = slot {
        locals.insert(&name, kind, value);
    }
    // healed with the real names as soon as game data is at hand
    gs.coop.campaign.unnamed_vars |= invented;
}

/// Faction relations and crime (`export::export_factions_crime`).
fn factions_crime(gs: &mut GameState, form: FormId, t: u16, data: &[u8]) {
    let word = |o: usize| {
        data.get(o..o + 4)
            .map_or(0, |b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    };
    match t {
        tag::RELATION => {
            gs.faction_relations.retain(|(a, _), _| *a != form);
            for c in data.chunks_exact(5) {
                let other = u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
                gs.faction_relations.insert((form, FormId(other)), c[4]);
            }
        }
        tag::CRIME => {
            if data.len() >= 9 {
                gs.faction_crimes.insert(form, (word(0), word(4)));
                set_in(&mut gs.crime_enemies, form, data[8] != 0);
            } else {
                gs.faction_crimes.remove(&form);
                gs.crime_enemies.remove(&form);
            }
        }
        _ => {
            gs.player_crimes = (word(0), word(4));
            gs.steal_warnings = word(8);
            gs.player_murderer = data.get(12).is_some_and(|b| *b != 0);
        }
    }
}

fn custom(gs: &mut GameState, form: FormId, t: u16, f: Option<&Field>) {
    let data: &[u8] = match f {
        Some(Field::Custom { data, .. }) => data,
        _ => &[],
    };
    let on = !data.is_empty();
    match t {
        tag::SAID => set_in(&mut gs.said, form, on),
        tag::TALKED => set_in(&mut gs.talked_to, form, on),
        tag::NOTE => set_in(&mut gs.notes, form, on),
        tag::DESTROYED => set_in(&mut gs.destroyed, form, on),
        tag::DEAD => {
            // the hash-borne death is the same death the live layer replays:
            // one `Died` event whichever arrives first
            if on {
                if gs.dead.insert(form) {
                    gs.combat.remove(&form);
                    gs.events.push(Event::Died {
                        who: form,
                        by: FormId(0),
                    });
                }
            } else {
                gs.dead.remove(&form);
            }
        }
        tag::STOCKED => set_in(&mut gs.stocked, form, on),
        tag::MARKER => {
            // bit 1 revealed, bit 2 found (`export`)
            let bits = data.first().copied().unwrap_or(0);
            set_in(&mut gs.map_markers, form, bits & 1 != 0);
            set_in(&mut gs.discovered, form, bits & 2 != 0);
        }
        tag::RELATION | tag::CRIME | tag::PLAYER_CRIME => factions_crime(gs, form, t, data),
        tag::PERK => {
            gs.perk_ranks.remove(&form);
            set_in(&mut gs.perks, form, on);
            if let Some(&rank) = data.first() {
                if rank > 1 {
                    gs.perk_ranks.insert(form, rank);
                }
            }
        }
        tag::LEVEL => {
            if data.len() >= 2 {
                gs.player_level = u16::from_le_bytes([data[0], data[1]]);
            }
        }
        tag::REPUTATION => {
            if data.len() >= 8 {
                let b = |o: usize| {
                    f32::from_bits(u32::from_le_bytes(data[o..o + 4].try_into().unwrap()))
                };
                gs.reputations.insert(form, (b(0), b(4)));
            } else {
                gs.reputations.remove(&form);
            }
        }
        t if t >= tag::FACTION_BASE => {
            let low = u32::from(t - tag::FACTION_BASE);
            gs.faction_changes
                .retain(|(fac, who), _| !(*fac == form && who.0 & 0xFF == low));
            for c in data.chunks_exact(5) {
                let who = u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
                gs.faction_changes.insert((form, FormId(who)), c[4] as i8);
            }
        }
        t if t >= tag::STAGES_BASE => {
            let chunk = t - tag::STAGES_BASE;
            gs.stages_done
                .retain(|(q, s)| !(*q == form && *s / tag::STAGES_CHUNK == chunk));
            for c in data.chunks_exact(2) {
                gs.stages_done
                    .insert((form, u16::from_le_bytes([c[0], c[1]])));
            }
        }
        _ => {}
    }
}
