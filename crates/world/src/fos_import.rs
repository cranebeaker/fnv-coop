//! Starting nv-rs from one of the original game's saves (`.fos`): the
//! parts `crates/fos` decodes, mapped onto [`GameState`] where nv-rs keeps
//! the same state (docs/FOS_SAVES.md, "Import"). Read-only: the save is
//! never written.
//!
//! - Form ids: the save's plugin list is matched to the load order by name
//!   (ignoring case), as the game's loader does (`00847660`, `00846d80`);
//!   forms whose plugin isn't loaded are dropped and counted. Forms made
//!   in game (`0xFF......`) keep their ids.
//! - Script variables are saved by index (`SLSD`); their names come from
//!   the owning script's `SCVR`, their kind from `SLSD`'s flags (or the
//!   save, for references).
//! - Inventories are saved as changes against the holder's record
//!   (`ItemChange` count); with `CHANGE_REFR_LEVELED_INVENTORY` the
//!   record's leveled lists were already resolved by the game and appear
//!   among the changes, so they aren't rolled again. Each holder's changes
//!   are kept stack by stack ([`crate::inventory_stacks`], taken from
//!   `fos::inventory_source`); the counts, worn items and weapon
//!   conditions nv-rs plays with are derived from them.
//! - What isn't imported is listed in [`GAPS`].

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use esm::{FormId, FourCC, LoadOrder};
use fos::decode::{self, Extra, InitialData, Value};
use fos::inventory_source::{InventorySource, ItemEnvelope, RecordState, Resolution};
use fos::{save_type as t, ChangeForm, RefId, Save};
use script::VarKind;

use crate::dialogue::{PLAYER_BASE, PLAYER_REF};
use crate::inventory_stacks::{SavedExtra, SavedInventory, SavedItem, SavedResolution, SavedStack};
use crate::save::PlayerPlace;
use crate::scripting::GameState;

const CELL: FourCC = FourCC::new(b"CELL");
const WRLD: FourCC = FourCC::new(b"WRLD");
const SLSD: FourCC = FourCC::new(b"SLSD");
const SCVR: FourCC = FourCC::new(b"SCVR");
const XMRK: FourCC = FourCC::new(b"XMRK");
const FNAM: FourCC = FourCC::new(b"FNAM");

/// Form flag 0x800: disabled (`00440da0` tests it), and 0x20: deleted
/// (a reference picked up). nv-rs keeps both as "disabled" (its own
/// pick-up does the same).
const FORM_DISABLED: u32 = 0x800;
const FORM_DELETED: u32 = 0x20;

/// What isn't imported, and why (for the doc and the viewer's report).
pub const GAPS: &[&str] = &[
    "active effects (each effect's own data is kept as an uninterpreted block)",
    "AI processes, packages, combat and pathing (nv-rs re-evaluates AI on loading)",
    "the actors leveled lists picked (nv-rs picks again when it spawns them)",
    "forms made in game other than references (created actor bases, packages)",
    "projectiles in flight",
    "the Havok poses of moved clutter (the block isn't decoded; their place is)",
    "doors' open state (the change flag is known, where the state is kept isn't)",
    "the player's skills and other actor values scripts modified, beyond S.P.E.C.I.A.L., experience and karma",
    "companion perks, the hardcore needs and radiation",
    "caravan cards and casino winnings, terminals' states, weapon mods",
    "item conditions outside weapons, and placed items' counts",
    "the player's controls turned off, hot key ammo",
    "global data 4 to 7 and 9 (process lists, combat, interface, effects, actor causes)",
];

/// A save imported into nv-rs.
pub struct Import {
    pub state: GameState,
    /// Where the player stands, as nv-rs's own saves give it.
    pub place: Option<PlayerPlace>,
    pub report: Report,
}

/// What the import did, counted by kind (`"quests"`, `"references
/// moved"`...), with the save's plugins nv-rs doesn't have and change
/// forms that didn't decode.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Report {
    pub counts: BTreeMap<&'static str, usize>,
    pub missing_plugins: Vec<String>,
    /// Forms whose plugin isn't loaded, dropped.
    pub dropped: usize,
    pub failures: Vec<String>,
}

impl Report {
    fn add(&mut self, what: &'static str) {
        *self.counts.entry(what).or_default() += 1;
    }
}

/// The save's form ids in nv-rs's load order.
struct Ids<'a> {
    /// Save plugin index → nv-rs load index.
    plugins: Vec<Option<u8>>,
    form_ids: &'a [u32],
}

impl Ids<'_> {
    fn new<'s>(order: &LoadOrder, save: &'s Save<'_>) -> (Ids<'s>, Vec<String>) {
        let mut missing = Vec::new();
        let plugins = save
            .plugins
            .iter()
            .map(|name| {
                let found = if order.is_single() {
                    // One plugin on its own keeps its local numbering.
                    order
                        .plugins()
                        .first()
                        .and_then(|p| p.name.eq_ignore_ascii_case(name).then_some(p.load_index))
                } else {
                    order
                        .plugins()
                        .iter()
                        .find(|p| p.name.eq_ignore_ascii_case(name))
                        .map(|p| p.load_index)
                };
                if found.is_none() {
                    missing.push(name.clone());
                }
                found
            })
            .collect();
        (
            Ids {
                plugins,
                form_ids: &save.form_ids,
            },
            missing,
        )
    }

    /// A form id as the save has it, in nv-rs's load order.
    fn form(&self, raw: u32) -> Option<FormId> {
        let top = raw >> 24;
        if top == 0xFF {
            return Some(FormId(raw));
        }
        let index = (*self.plugins.get(top as usize)?)?;
        Some(FormId(u32::from(index) << 24 | raw & 0xFF_FFFF))
    }

    fn of(&self, r: RefId) -> Option<FormId> {
        self.form(r.form_id(self.form_ids)?)
    }
}

/// Builds the state a save's bytes describe: a new game's state
/// (start-game quests running, globals and the player's record items)
/// with everything the save changed laid over it.
pub fn import(order: &LoadOrder, bytes: &[u8]) -> Result<Import, String> {
    let save = Save::parse(bytes).map_err(|e| e.to_string())?;
    let source = InventorySource::parse(Arc::from(bytes)).map_err(|e| e.to_string())?;
    import_parsed(order, &save, &source)
}

/// [`import`] with the save and its inventory source parsed from the same
/// bytes.
fn import_parsed(
    order: &LoadOrder,
    save: &Save<'_>,
    source: &InventorySource,
) -> Result<Import, String> {
    let mut state = GameState::new(order);
    let mut report = Report::default();
    let (ids, missing) = Ids::new(order, save);
    report.missing_plugins = missing;
    // Each reference's inventory record, by where its data starts.
    let stacks: HashMap<usize, &RecordState> = source
        .records()
        .iter()
        .map(|r| (r.source().data_file_span().start, r.state()))
        .collect();
    let mut cx = Cx {
        order,
        ids: &ids,
        report: &mut report,
        cells: HashMap::new(),
        stacks: &stacks,
    };
    state.player_level = u16::try_from(save.header.level.max(1)).unwrap_or(1);
    if let Some(g) = save.global_data(3) {
        globals(&mut cx, &mut state, g)?;
    }
    if let Some(g) = save.global_data(0) {
        let stats = decode::misc_stats(g).map_err(|e| e.to_string())?;
        for (i, v) in stats.into_iter().enumerate() {
            if let Ok(i) = u8::try_from(i) {
                state.misc_stats.insert(i, v);
            }
        }
    }
    if let Some(Ok(sky)) = save.global_data(8).map(decode::sky) {
        weather(&cx, &mut state, &sky);
    }
    if let Some(Ok(radio)) = save.global_data(10).map(decode::radio) {
        state.radio.on = radio.on;
        state.radio.active = cx.ids.of(radio.active);
        state.radio.lost_station = cx.ids.of(radio.lost);
        state.radio.discovered = radio
            .discovered
            .iter()
            .filter_map(|&r| cx.ids.of(r))
            .collect();
    }
    let mut place = None;
    // References last: the player's values add onto the base's.
    let is_ref = |cf: &&ChangeForm<'_>| decode::is_reference(cf.save_type);
    let forms = save
        .change_forms
        .iter()
        .filter(|cf| !is_ref(cf))
        .chain(save.change_forms.iter().filter(is_ref));
    for cf in forms {
        let Some(id) = cx.ids.of(cf.ref_id) else {
            cx.report.dropped += 1;
            continue;
        };
        let result = match cf.save_type {
            t::QUST => quest(&mut cx, &mut state, id, cf),
            t::INFO => {
                if cf.flags & 0x8000_0000 != 0 {
                    state.said.insert(id);
                    cx.report.add("topics said");
                }
                Ok(())
            }
            t::NPC_ if id == PLAYER_BASE => player_base(&mut cx, &mut state, cf),
            t::CLAS => class(&mut cx, &mut state, cf),
            t::FACT => faction(&mut cx, &mut state, id, cf),
            t::CHAL => decode::pair(cf)
                .map(|(progress, flags)| {
                    state
                        .more
                        .challenges
                        .progress
                        .insert(id, (progress as i32, flags));
                    cx.report.add("challenges");
                })
                .map_err(|e| e.to_string()),
            t::REPU => decode::pair(cf)
                .map(|(fame, infamy)| {
                    state
                        .reputations
                        .insert(id, (f32::from_bits(fame), f32::from_bits(infamy)));
                    cx.report.add("reputations");
                })
                .map_err(|e| e.to_string()),
            t::CELL => cell(&mut cx, &mut state, id, cf),
            t::FLST => decode::base_form(cf)
                .map(|b| {
                    if let Some(added) = b.added {
                        let forms = added.iter().filter_map(|&r| cx.ids.of(r)).collect();
                        state.set_by_scripts.list_additions.insert(id, forms);
                        cx.report.add("form lists");
                    }
                })
                .map_err(|e| e.to_string()),
            t::REFR => decode::reference(cf)
                .map_err(|e| e.to_string())
                .map(|r| reference(&mut cx, &mut state, id, cf, &r.initial, &r.data)),
            t::ACHR | t::ACRE => {
                actor(&mut cx, &mut state, id, cf).map(|p| place = p.or(place.take()))
            }
            _ => Ok(()),
        };
        if let Err(e) = result {
            let name = cf.type_name().unwrap_or("?");
            cx.report.failures.push(format!("{name} {id}: {e}"));
        }
    }
    if place.is_none() {
        place = location_place(&mut cx, &mut state, save);
    }
    Ok(Import {
        state,
        place,
        report,
    })
}

/// What the importers share.
struct Cx<'a, 'b> {
    order: &'a LoadOrder,
    ids: &'a Ids<'b>,
    report: &'a mut Report,
    /// The cells of each worldspace by grid square, loaded when needed.
    cells: HashMap<FormId, Option<crate::WorldGrid>>,
    /// The inventory source's record for each reference, by the file
    /// offset of its data.
    stacks: &'a HashMap<usize, &'a RecordState>,
}

impl Cx<'_, '_> {
    /// The record type of a form.
    fn kind(&self, id: FormId) -> Option<FourCC> {
        self.order.get(id).map(|r| r.entry.header.kind)
    }

    /// The cell at a place: an interior cell itself, or a worldspace's
    /// cell at the position's grid square.
    fn cell_at(&mut self, space: FormId, position: [f32; 3]) -> Option<FormId> {
        match self.kind(space)? {
            k if k == CELL => Some(space),
            k if k == WRLD => {
                let order = self.order;
                let grid = self
                    .cells
                    .entry(space)
                    .or_insert_with(|| crate::WorldGrid::load(order, space).ok());
                grid.as_ref()?.cell_at(crate::square_of(position))
            }
            _ => None,
        }
    }
}

/// Global data 3: every saved global (game time among them).
fn globals(
    cx: &mut Cx<'_, '_>,
    state: &mut GameState,
    g: &fos::GlobalData<'_>,
) -> Result<(), String> {
    for (r, v) in decode::globals(g).map_err(|e| e.to_string())? {
        if let Some(id) = cx.ids.of(r) {
            state.globals.insert(id, v);
            cx.report.add("globals");
        }
    }
    Ok(())
}

/// Global data 8: the sky's weathers (`Sky` (Xbox PDB) +0x10 .. +0x1c,
/// +0xf0, +0xf4, +0x110) onto `world::weather`'s state: current, fading
/// out, the climate's pick (`pDefaultWeather`), a script's override, the
/// hour the current one began and how far its fade has got.
fn weather(cx: &Cx<'_, '_>, state: &mut GameState, sky: &decode::Sky) {
    let w = &mut state.weather;
    w.current = cx.ids.of(sky.current);
    w.previous = cx.ids.of(sky.last);
    w.picked = cx.ids.of(sky.default);
    w.forced = cx.ids.of(sky.override_weather);
    w.started = sky.last_update;
    w.fade = sky.weather_pct;
    w.sped_up = (w.previous.is_some() && sky.accel_begin_pct > 0.0).then_some(sky.accel_begin_pct);
}

/// A quest: its run flags, stages, objectives, script delay and
/// variables (`0060e810`). The current stage is the highest done, as the
/// game's loader makes it (`0060d670`).
fn quest(
    cx: &mut Cx<'_, '_>,
    state: &mut GameState,
    id: FormId,
    cf: &ChangeForm<'_>,
) -> Result<(), String> {
    use decode::quest_flag as qf;
    let q = decode::quest(cf).map_err(|e| e.to_string())?;
    cx.report.add("quests");
    if let Some(flags) = q.flags {
        let set = |on: bool, members: &mut std::collections::HashSet<FormId>| {
            if on {
                members.insert(id);
            } else {
                members.remove(&id);
            }
        };
        set(flags & qf::ENABLED != 0, &mut state.running);
        set(flags & qf::COMPLETED != 0, &mut state.completed);
        set(flags & qf::FAILED != 0, &mut state.failed);
    }
    if let Some(stages) = &q.stages {
        for s in stages.iter().filter(|s| s.done) {
            state.stages_done.insert((id, u16::from(s.index)));
        }
        if let Some(current) = q.current_stage() {
            state.stages.insert(id, u16::from(current));
        }
    }
    if let Some(objectives) = &q.objectives {
        for o in objectives {
            let key = (id, o.index as i32);
            match o.state {
                1 => {
                    state.objectives.insert(key, false);
                }
                3 => {
                    state.objectives.insert(key, true);
                }
                2 => {
                    state.set_by_scripts.hidden_completed.insert(key);
                }
                _ => {}
            }
        }
    }
    if let Some(delay) = q.script_delay {
        state.quest_delays.insert(id, delay);
    }
    if let Some(locals) = &q.script {
        if let Some(script) = crate::scripting::script_of(cx.order, id) {
            variables(cx, state, id, script, locals);
        }
    }
    Ok(())
}

/// A script's variables by index: name and kind (`SLSD`: index, then at
/// 16 the flags, 0x01 a whole number; `SCVR` the name).
fn script_variables(order: &LoadOrder, script: FormId) -> HashMap<u32, (String, VarKind)> {
    let mut out = HashMap::new();
    let Some(record) = order.get(script).and_then(|r| r.record().ok()) else {
        return out;
    };
    let mut current: Option<(u32, VarKind)> = None;
    for sub in &record.subrecords {
        if sub.kind == SLSD && sub.data.len() >= 17 {
            let index = u32::from_le_bytes([sub.data[0], sub.data[1], sub.data[2], sub.data[3]]);
            let kind = if sub.data[16] & 0x01 != 0 {
                VarKind::Integer
            } else {
                VarKind::Float
            };
            current = Some((index, kind));
        } else if sub.kind == SCVR {
            if let Some((index, kind)) = current.take() {
                out.insert(index, (sub.zstring(), kind));
            }
        }
    }
    out
}

/// Script locals saved by index, named through the owner's script.
fn variables(
    cx: &mut Cx<'_, '_>,
    state: &mut GameState,
    owner: FormId,
    script: FormId,
    locals: &decode::ScriptLocals,
) {
    let names = script_variables(cx.order, script);
    let entry = state.variables.entry(owner).or_default();
    for v in &locals.variables {
        let Some((name, kind)) = names.get(&v.id) else {
            continue;
        };
        let (kind, value) = match v.value {
            Value::Number(n) => (*kind, n),
            Value::Ref(r) => (VarKind::Ref, cx.ids.of(r).map_or(0.0, |f| f64::from(f.0))),
        };
        entry.insert(name, kind, value);
        cx.report.add("script variables");
    }
}

/// The player's base (`NPC_` 0x7): name, sex, S.P.E.C.I.A.L. and level.
fn player_base(
    cx: &mut Cx<'_, '_>,
    state: &mut GameState,
    cf: &ChangeForm<'_>,
) -> Result<(), String> {
    let b = decode::actor_base_form(cf).map_err(|e| e.to_string())?;
    if let Some(name) = &b.name {
        state.player_name = Some(name.clone());
    }
    if let Some(g) = b.gender {
        state.player_female = Some(g != 0);
    }
    if let Some(a) = b.attributes {
        for (av, v) in crate::chargen::SPECIAL.iter().zip(a) {
            state.actor_values.insert((PLAYER_REF, *av), f64::from(v));
        }
        cx.report.add("S.P.E.C.I.A.L.");
    }
    if let Some(level) = b.level().filter(|&l| l > 0) {
        state.player_level = level as u16;
    }
    if let Some(karma) = b.karma() {
        state
            .actor_values
            .insert((PLAYER_REF, 23), f64::from(karma));
    }
    Ok(())
}

/// The class whose tag skills changed is the player's (`CLASS_TAG_SKILLS`).
fn class(cx: &mut Cx<'_, '_>, state: &mut GameState, cf: &ChangeForm<'_>) -> Result<(), String> {
    if let Some(tags) = decode::class(cf).map_err(|e| e.to_string())? {
        state.tag_skills = tags
            .iter()
            .filter_map(|&av| u16::try_from(av).ok())
            .collect();
        state.tag_slots = (0u8..)
            .zip(tags)
            .filter_map(|(slot, av)| Some((slot, u16::try_from(av).ok()?)))
            .collect();
        cx.report.add("tag skills");
    }
    Ok(())
}

/// A faction's crimes (`iMajorCrime`, `iMinorCrime`; nv-rs keeps (minor,
/// major)) and reactions (group reaction: 0 neutral, 1 enemy, 2 ally, 3
/// friend).
fn faction(
    cx: &mut Cx<'_, '_>,
    state: &mut GameState,
    id: FormId,
    cf: &ChangeForm<'_>,
) -> Result<(), String> {
    let f = decode::faction(cf).map_err(|e| e.to_string())?;
    if let Some((major, minor)) = f.crimes {
        state
            .faction_crimes
            .insert(id, (minor.max(0) as u32, major.max(0) as u32));
        cx.report.add("faction crimes");
    }
    for (other, _, reaction) in f.reactions.iter().flatten() {
        if let (Some(other), Ok(r)) = (cx.ids.of(*other), u8::try_from(*reaction)) {
            state.faction_relations.insert((id, other), r);
        }
    }
    Ok(())
}

/// A cell's local map fog, owner and name.
fn cell(
    cx: &mut Cx<'_, '_>,
    state: &mut GameState,
    id: FormId,
    cf: &ChangeForm<'_>,
) -> Result<(), String> {
    let rr = cx.order.get(id);
    let interior = rr.as_ref().map(|r| cx.order.world_of(r).is_none());
    let c = decode::cell_data(cf, interior).map_err(|e| e.to_string())?;
    if let Some(owner) = c.owner.and_then(|o| cx.ids.of(o)) {
        state.set_by_scripts.cell_owners.insert(id, owner);
    }
    if let Some(seen) = &c.seen {
        let words = |b: &[u8; 32]| {
            let mut w = [0u32; 8];
            for (i, w) in w.iter_mut().enumerate() {
                *w = u32::from_le_bytes([b[4 * i], b[4 * i + 1], b[4 * i + 2], b[4 * i + 3]]);
            }
            w
        };
        let mut put = |key: crate::local_map::SeenKey, b: &[u8; 32]| {
            let w = words(b);
            let count: u32 = w.iter().map(|x| x.count_ones()).sum();
            state.seen.bits.insert(key, w);
            if count >= crate::local_map::FULLY_SEEN {
                state.seen.fully.insert(key);
            }
        };
        match seen {
            decode::Seen::Interior(parts) => {
                for (x, y, b) in parts {
                    put(
                        crate::local_map::SeenKey::Interior(id, i32::from(*x), i32::from(*y)),
                        b,
                    );
                }
            }
            decode::Seen::Exterior(b) => {
                let grid = rr.as_ref().and_then(|r| {
                    let world = cx.order.world_of(r)?;
                    let record = r.record().ok()?;
                    let xclc = record
                        .get(FourCC::new(b"XCLC"))
                        .filter(|s| s.data.len() >= 8)?;
                    let d = &xclc.data;
                    Some((
                        world,
                        i32::from_le_bytes([d[0], d[1], d[2], d[3]]),
                        i32::from_le_bytes([d[4], d[5], d[6], d[7]]),
                    ))
                });
                if let Some((world, x, y)) = grid {
                    put(crate::local_map::SeenKey::Exterior(world, x, y), b);
                }
            }
        }
        cx.report.add("cells seen");
    }
    Ok(())
}

/// A map marker's flags in its record (`XMRK` then `FNAM`).
fn marker_flags(order: &LoadOrder, reference: FormId) -> Option<u8> {
    let record = order.get(reference)?.record().ok()?;
    let mut seen = false;
    for sub in &record.subrecords {
        if sub.kind == XMRK {
            seen = true;
        } else if seen && sub.kind == FNAM {
            return sub.data.first().copied();
        }
    }
    None
}

/// Whether a reference's record is disabled or deleted.
fn record_off(order: &LoadOrder, id: FormId) -> bool {
    order
        .get(id)
        .is_some_and(|r| r.entry.header.flags & (FORM_DISABLED | FORM_DELETED) != 0)
}

/// A reference's place, disabled state, scale, extra data and inventory.
fn reference(
    cx: &mut Cx<'_, '_>,
    state: &mut GameState,
    id: FormId,
    cf: &ChangeForm<'_>,
    initial: &InitialData,
    data: &decode::ReferenceData<'_>,
) {
    if let Some(flags) = data.form_flags {
        let off = flags & (FORM_DISABLED | FORM_DELETED) != 0;
        if off != record_off(cx.order, id) || id.0 >> 24 == 0xFF {
            state.disabled.insert(id, off);
            cx.report.add("references enabled or disabled");
        }
    }
    if let Some(scale) = data.scale {
        state.scales.insert(id, scale);
    }
    match *initial {
        InitialData::Created {
            space,
            position,
            rotation,
            base,
            ..
        } => {
            let base = template_of(cx, data).or_else(|| cx.ids.of(base));
            let space = cx.ids.of(space);
            if let (Some(base), Some(space)) = (base, space) {
                if base.0 >> 24 != 0xFF && !state.disabled.get(&id).copied().unwrap_or(false) {
                    let cell = cx.cell_at(space, position).unwrap_or(space);
                    let count = data
                        .extra
                        .iter()
                        .flatten()
                        .find_map(|e| match e {
                            Extra::Count(n) => Some(i32::from(*n)),
                            _ => None,
                        })
                        .unwrap_or(1);
                    let placed = &mut state.more.placed;
                    placed.refs.insert(
                        id,
                        crate::more_functions::placed::Made {
                            base,
                            space,
                            cell,
                            position,
                            rotation,
                            count,
                        },
                    );
                    let low = id.0.wrapping_sub(crate::more_functions::placed::FIRST_ID);
                    if low < 0x00FF_FFFF {
                        placed.next = placed.next.max(low + 1);
                    }
                    cx.report.add("references made in game");
                } else {
                    cx.report.add("references made in game, not placed");
                }
            }
        }
        InitialData::Location {
            space,
            position,
            rotation,
        }
        | InitialData::Moved {
            space,
            position,
            rotation,
            ..
        } => {
            state.positions.insert(id, (position, rotation[2]));
            if let InitialData::Moved { .. } = initial {
                if let Some(space) = cx.ids.of(space) {
                    let cell = cx.cell_at(space, position).unwrap_or(space);
                    state.spaces.insert(id, (space, cell));
                }
            }
            cx.report.add("references moved");
        }
        _ => {}
    }
    for e in data.extra.iter().flatten() {
        match e {
            Extra::Script { script, locals } => {
                if let Some(script) = cx.ids.of(*script) {
                    variables(cx, state, id, script, locals);
                }
            }
            Extra::Lock(l) => {
                let level = (l.flags & 0x01 != 0).then_some(l.level);
                state.locks.insert(id, level);
                if l.tries > 0 {
                    state.broken_locks.insert(id, l.tries);
                }
                cx.report.add("locks");
            }
            Extra::MapMarker(flags) => {
                let record = marker_flags(cx.order, id).unwrap_or(0);
                if flags & 0x03 == 0x03 && record & 0x03 != 0x03 {
                    state.discovered.insert(id);
                    cx.report.add("map markers found");
                } else if flags & 0x01 != 0 && record & 0x01 == 0 {
                    state.map_markers.insert(id);
                    cx.report.add("map markers shown");
                }
            }
            Extra::Ownership(owner) if cf.flags & 0x40 != 0 => {
                if let Some(owner) = cx.ids.of(*owner) {
                    state.set_by_scripts.owners.insert(id, owner);
                }
            }
            Extra::Ghost => {
                state.more.ghosts.insert(id);
            }
            Extra::FactionChanges(list) => {
                for (faction, rank) in list {
                    if let Some(faction) = cx.ids.of(*faction) {
                        state.faction_changes.insert((id, faction), *rank);
                    }
                }
            }
            _ => {}
        }
    }
    if data.inventory.is_some() {
        // The same record, read again with its stacks' bytes kept
        // (`fos::inventory_source`).
        let stacks = cx.stacks;
        match stacks.get(&cf.data_offset) {
            Some(RecordState::Decoded(items)) => {
                let leveled_resolved = cf.flags & 0x0800_0000 != 0;
                inventory(cx, state, id, leveled_resolved, cf.version, items);
            }
            other => {
                let why = match other {
                    Some(RecordState::PreservedUndecoded(f)) => format!("{f:?}"),
                    Some(_) => "no inventory".to_string(),
                    None => "no record".to_string(),
                };
                cx.report
                    .failures
                    .push(format!("inventory of {id}: stacks not read ({why})"));
            }
        }
    }
}

/// A leveled actor's pick (`ExtraLeveledCreature::pTemplate`), a form
/// nv-rs has where the reference's own base was made in game.
fn template_of(cx: &Cx<'_, '_>, data: &decode::ReferenceData<'_>) -> Option<FormId> {
    data.extra.iter().flatten().find_map(|e| match e {
        Extra::LeveledCreature { template, .. } => cx.ids.of(*template),
        _ => None,
    })
}

/// One saved item change as nv-rs keeps it: its identity resolved into
/// nv-rs's load order, its count delta, and each stack's count, raw
/// health bits and worn state, other extras as their original bytes.
fn saved_item(cx: &Cx<'_, '_>, e: &ItemEnvelope) -> SavedItem {
    let (resolution, form) = match e.identity().resolution() {
        Resolution::Null => (SavedResolution::Null, None),
        Resolution::ResolvedInSave(raw) => (SavedResolution::InSave(raw), cx.ids.form(raw)),
        Resolution::MissingIndex { index } => (SavedResolution::Missing(index), None),
    };
    let stacks = e
        .stacks()
        .iter()
        .map(|s| SavedStack {
            extras: s
                .extras()
                .iter()
                .map(|x| match x.decoded() {
                    Extra::Count(n) => SavedExtra::Count(*n),
                    Extra::Health(h) => SavedExtra::Health(h.to_bits()),
                    Extra::Worn => SavedExtra::Worn,
                    _ => SavedExtra::Opaque {
                        kind: x.kind(),
                        bytes: x.original_payload().to_vec(),
                    },
                })
                .collect(),
        })
        .collect();
    SavedItem {
        saved: e.identity().saved().0,
        resolution,
        form,
        count_delta: e.count_delta(),
        stacks,
    }
}

/// A holder's items: its record's (leveled lists rolled by nv-rs unless
/// the game resolved them) and the save's changes, kept stack by stack;
/// from those, the counts, stacks worn are equipped, and weapons' health
/// becomes their condition (the last stack's with health, as before the
/// stacks were kept: which stack's condition the game shows isn't traced).
fn inventory(
    cx: &mut Cx<'_, '_>,
    state: &mut GameState,
    holder: FormId,
    leveled_resolved: bool,
    version: u8,
    changes: &[ItemEnvelope],
) {
    state.items.retain(|(h, _), _| *h != holder);
    state.stocked.insert(holder);
    let level = state.player_level;
    for (form, n) in crate::scripting::record_contents(cx.order, holder) {
        let leveled = cx.kind(form).is_some_and(crate::leveled::is_leveled);
        if leveled && leveled_resolved {
            continue;
        }
        let picked = crate::leveled::resolve(cx.order, form, n, level, &mut || state.roll());
        for (item, count) in picked {
            *state.items.entry((holder, item)).or_insert(0) += count;
        }
    }
    state.equipped.remove(&holder);
    let saved = SavedInventory {
        version,
        items: changes.iter().map(|e| saved_item(cx, e)).collect(),
    };
    for c in &saved.items {
        let Some(item) = c.form else {
            continue;
        };
        *state.items.entry((holder, item)).or_insert(0) += c.count_delta;
        for stack in &c.stacks {
            for e in &stack.extras {
                match e {
                    SavedExtra::Worn => state.equipped.entry(holder).or_default().push(item),
                    SavedExtra::Health(bits) => {
                        let h = f32::from_bits(*bits);
                        if let Some(w) = crate::combat::Weapon::load(cx.order, item) {
                            if w.health > 0 {
                                state
                                    .weapon_health
                                    .insert((holder, item), (h / w.health as f32).clamp(0.0, 1.0));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    state.items.retain(|_, n| *n > 0);
    state.inventory_stacks.insert(holder, saved);
    cx.report.add("inventories");
}

/// An actor: the reference parts, life state, actor values and flags;
/// the player's own data too. Returns the player's place.
fn actor(
    cx: &mut Cx<'_, '_>,
    state: &mut GameState,
    id: FormId,
    cf: &ChangeForm<'_>,
) -> Result<Option<PlayerPlace>, String> {
    let is_player = id == PLAYER_REF;
    let a = decode::actor_form(cf, is_player).map_err(|e| e.to_string())?;
    let actor = &a.actor;
    reference(cx, state, id, cf, &a.initial, &actor.mobile.data);
    match actor.life_state {
        Some(1 | 2) => {
            state.dead.insert(id);
            cx.report.add("dead");
        }
        Some(3) => {
            state.unconscious.insert(id);
        }
        Some(5) => {
            state.set_by_scripts.restrained.insert(id);
        }
        _ => {}
    }
    let f = &actor.fields;
    if f.teammate {
        state.teammates.insert(id);
    }
    if f.ignore_crime {
        state.set_by_scripts.ignoring_crime.insert(id);
    }
    if f.force_sneak {
        state.more.forced_sneak.insert(id);
    }
    if f.critical_stage != 0 {
        state
            .more
            .critical_stage
            .insert(id, f.critical_stage as i32);
    }
    for (whom, amount) in actor.dispositions.iter().flatten() {
        if cx.ids.of(*whom) == Some(PLAYER_REF) {
            *state.more.dispositions.0.entry(id).or_insert(0) += *amount as i32;
        }
    }
    // Actor values: set (the override) or the record's, plus what's been
    // added; damage comes off the health and the other values.
    let mut set: BTreeMap<u16, f64> = BTreeMap::new();
    for &(av, v) in actor.override_modifiers.iter().flatten() {
        set.insert(u16::from(av), f64::from(v));
    }
    for &(av, v) in actor.permanent_modifiers.iter().flatten() {
        let av = u16::from(av);
        if is_player && (32..=45).contains(&av) {
            continue;
        }
        let base = match set.get(&av) {
            Some(&b) => b,
            None => base_value(cx, state, id, av),
        };
        set.insert(av, base + f64::from(v));
    }
    for (av, v) in set {
        state.actor_values.insert((id, av), v);
    }
    let damage = actor
        .mobile
        .process
        .as_ref()
        .and_then(|p| p.damage_modifiers.clone())
        .unwrap_or_default();
    for (av, v) in damage {
        hurt(state, id, u16::from(av), v);
    }
    if !is_player {
        return Ok(None);
    }
    if let Some(p) = &a.player {
        player(cx, state, p, f);
    }
    let Some((space, position, rotation)) = a.initial.place() else {
        return Ok(None);
    };
    let Some(space) = cx.ids.of(space) else {
        return Ok(None);
    };
    Ok(player_place(cx, state, space, position, rotation[2]))
}

/// An actor value from the actor's record, as nv-rs reads it.
fn base_value(cx: &Cx<'_, '_>, state: &GameState, who: FormId, av: u16) -> f64 {
    let facts = crate::scripting::Facts {
        order: cx.order,
        state,
        speaker: None,
    };
    facts.base_actor_value(who, av).unwrap_or(0.0)
}

/// A damage modifier (negative): health lost, or another value damaged.
/// Radiation and the hardcore needs count up and aren't imported.
fn hurt(state: &mut GameState, who: FormId, av: u16, v: f32) {
    if v >= 0.0 || crate::magic::COUNTERS.contains(&av) {
        return;
    }
    let lost = -f64::from(v);
    if av == crate::combat::av::HEALTH {
        state.damage.insert(who, lost);
    } else {
        state.value_damage.insert((who, av), lost);
    }
}

/// The player's own data: actor values scripts changed (experience and
/// S.P.E.C.I.A.L. added onto the base), damage, perks, the active quest,
/// notes, topics, hot keys, crimes and flags.
fn player(cx: &mut Cx<'_, '_>, state: &mut GameState, p: &decode::Player, f: &decode::ActorFields) {
    for (av, &v) in p.script_values.iter().enumerate() {
        let av = av as u16;
        if v == 0.0 {
            continue;
        }
        let special = crate::chargen::SPECIAL.contains(&av);
        if special || av == crate::experience::XP || av == 23 {
            let base = state
                .actor_values
                .get(&(PLAYER_REF, av))
                .copied()
                .unwrap_or_else(|| base_value(cx, state, PLAYER_REF, av));
            state
                .actor_values
                .insert((PLAYER_REF, av), base + f64::from(v));
        }
    }
    for (av, &v) in p.damage_values.iter().enumerate() {
        hurt(state, PLAYER_REF, av as u16, v);
    }
    for &(perk, rank) in &p.perks {
        let Some(perk) = cx.ids.of(perk) else {
            continue;
        };
        for _ in 0..rank {
            crate::perks::add(cx.order, state, perk);
        }
        cx.report.add("perks");
    }
    state.active_quest = cx.ids.of(p.active_quest);
    state
        .notes
        .extend(p.notes.iter().filter_map(|&n| cx.ids.of(n)));
    state
        .topics
        .extend(p.topics.iter().filter_map(|&n| cx.ids.of(n)));
    for (slot, &raw) in state.hotkeys.iter_mut().zip(&p.hotkeys) {
        *slot = (raw != 0).then(|| cx.ids.form(raw)).flatten();
    }
    state.player_crimes = (f.minor_crimes, f.major_crimes);
    state.steal_warnings = p.steal_warnings;
    state.more.in_chargen = p.chargen;
    let ft = &mut state.set_by_scripts.fast_travel;
    ft.enabled = p.fast_travel & 0x01 != 0;
    ft.keep = p.fast_travel & 0x02 != 0;
    ft.can_wait = p.can_wait;
    cx.report.add("player");
}

/// The player's place: an interior cell, or a worldspace and the cell at
/// the position.
fn player_place(
    cx: &mut Cx<'_, '_>,
    state: &mut GameState,
    space: FormId,
    position: [f32; 3],
    heading: f32,
) -> Option<PlayerPlace> {
    let cell = cx.cell_at(space, position)?;
    let world = (cx.kind(space) == Some(WRLD)).then_some(space);
    state.player_cell = Some(cell);
    state.player_world = world;
    state.player_position = Some(position);
    state.positions.remove(&PLAYER_REF);
    Some(PlayerPlace {
        cell,
        world,
        position,
        heading,
    })
}

/// Without a player change form, the location global data
/// (`SaveLocationData`, `0084c490`) gives the place.
fn location_place(
    cx: &mut Cx<'_, '_>,
    state: &mut GameState,
    save: &Save<'_>,
) -> Option<PlayerPlace> {
    let l = decode::location(save.global_data(1)?).ok()?;
    let space = cx.ids.of(l.player_space)?;
    player_place(cx, state, space, l.player_position, 0.0)
}

#[cfg(test)]
mod tests;
