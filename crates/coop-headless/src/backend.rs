//! Backend conformance: behavioural co-op tests written against a trait, so
//! the same checklist rows can run against the hand-written world layer
//! (`world::GameState`, [`WorldBackend`]) today and the translated engine
//! (`crates/engine`, `Mem`) when it carries the game.
//!
//! A [`CoopBackend`] hands out peers; a [`BackendPeer`] is one machine's
//! game world seen only through what co-op needs: set a quest stage, read it
//! back, export the shared state, apply a snapshot or delta from the host,
//! and keep per-player rows. The tests in [`TESTS`] use nothing else.
//! [`EngineBackend`] is the stub for the engine: it reports every test as
//! `PENDING (engine backend)`.

use std::sync::Arc;

use coop_core::delta::{Change, Delta, SnapEntry};
use coop_core::hash::ShardHashes;
use coop_core::ids::{FormId as CForm, Rev, Tick};
use coop_core::schema::Field;
use coop_policy::PolicyTable;
use world::coop::apply::{Ctx, Opts};
use world::coop::{Authority, ShadowPlan, WorldBackend};
use world::scripting::{GameState, Runner};

use crate::data::Data;

/// Why a conformance test did not pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// Held.
    Pass,
    /// Did not hold.
    Fail(String),
    /// The backend cannot run it yet.
    Pending(String),
}

/// One machine's world, as co-op sees it.
pub trait BackendPeer {
    /// Runs `SetStage` the way the game's rules do on this machine; false when refused.
    fn set_stage(&mut self, quest: &str, stage: u16) -> bool;
    /// Same, with player `slot`'s rows live (a guest's intent run on the host).
    fn set_stage_as(&mut self, slot: u8, quest: &str, stage: u16) -> bool;
    /// The shared stage of a quest.
    fn stage(&self, quest: &str) -> Option<u16>;
    /// A player's own stage of a per-player quest.
    fn player_stage(&self, slot: u8, quest: &str) -> Option<u16>;
    /// A world global's value.
    fn set_global(&mut self, global: &str, v: f32) -> bool;
    /// Reads a world global.
    fn global(&self, global: &str) -> Option<f32>;
    /// Every shared field.
    fn export(&self) -> Vec<(CForm, Field)>;
    /// Hashes of the shared state.
    fn shard_hashes(&self) -> ShardHashes;
    /// Replaces the shared state with a snapshot; returns (set, cleared).
    fn apply_snapshot(&mut self, entries: &[SnapEntry]) -> (u32, u32);
    /// Applies a delta; returns (set, cleared).
    fn apply_delta(&mut self, d: &Delta) -> (u32, u32);
    /// Adds a player to the party (their private rows).
    fn register_player(&mut self, slot: u8);
}

/// A game world that can host a co-op session.
pub trait CoopBackend: Sync {
    /// Name for reports.
    fn name(&self) -> &'static str;
    /// A new machine: the host (authority) or a guest (runs no quest script).
    fn peer<'a>(&self, d: &'a Data, host: bool) -> Result<Box<dyn BackendPeer + 'a>, String>;
}

/// The stub for the translated engine: nothing runs yet.
pub struct EngineBackend;

impl CoopBackend for EngineBackend {
    fn name(&self) -> &'static str {
        "engine"
    }
    fn peer<'a>(&self, _d: &'a Data, _host: bool) -> Result<Box<dyn BackendPeer + 'a>, String> {
        Err("PENDING (engine backend)".into())
    }
}

/// The hand-written world layer.
pub struct GameStateBackend {
    table: Arc<PolicyTable>,
}

impl GameStateBackend {
    /// With the bundled policy table.
    pub fn new() -> Result<GameStateBackend, String> {
        Ok(GameStateBackend {
            table: Arc::new(PolicyTable::bundled().map_err(|e| format!("{e:?}"))?),
        })
    }
}

struct WPeer<'a> {
    d: &'a Data,
    gs: GameState,
}

impl WPeer<'_> {
    fn id(&self, edid: &str) -> Option<esm::FormId> {
        self.d.order.form_by_editor_id(edid)
    }
}

impl BackendPeer for WPeer<'_> {
    fn set_stage(&mut self, quest: &str, stage: u16) -> bool {
        let Some(q) = self.id(quest) else {
            return false;
        };
        Runner::new(&self.d.order, &self.d.scripts, &mut self.gs).set_stage(q, stage)
    }
    fn set_stage_as(&mut self, slot: u8, quest: &str, stage: u16) -> bool {
        let Some(q) = self.id(quest) else {
            return false;
        };
        let mut g = self.gs.as_player(slot);
        Runner::new(&self.d.order, &self.d.scripts, &mut g).set_stage(q, stage)
    }
    fn stage(&self, quest: &str) -> Option<u16> {
        self.gs.stages.get(&self.id(quest)?).copied()
    }
    fn player_stage(&self, slot: u8, quest: &str) -> Option<u16> {
        let q = self.id(quest)?;
        self.gs.shadow_of(slot)?.quest_stage.get(&q).copied()
    }
    fn set_global(&mut self, global: &str, v: f32) -> bool {
        let Some(g) = self.id(global) else {
            return false;
        };
        self.gs.globals.insert(g, v);
        self.gs.emit(world::coop::Domain::Global, g);
        true
    }
    fn global(&self, global: &str) -> Option<f32> {
        self.gs.globals.get(&self.id(global)?).copied()
    }
    fn export(&self) -> Vec<(CForm, Field)> {
        self.gs.export_fields()
    }
    fn shard_hashes(&self) -> ShardHashes {
        WorldBackend::shard_hashes(&self.gs)
    }
    fn apply_snapshot(&mut self, entries: &[SnapEntry]) -> (u32, u32) {
        let opts = Opts {
            present: false,
            ctx: Some(Ctx {
                order: &self.d.order,
                scripts: &self.d.scripts,
            }),
        };
        let r = self.gs.apply_snapshot(entries, opts);
        (r.set, r.cleared)
    }
    fn apply_delta(&mut self, d: &Delta) -> (u32, u32) {
        let opts = Opts {
            present: true,
            ctx: Some(Ctx {
                order: &self.d.order,
                scripts: &self.d.scripts,
            }),
        };
        let r = self.gs.apply_delta(d, opts);
        (r.set, r.cleared)
    }
    fn register_player(&mut self, slot: u8) {
        self.gs.register_player(slot);
    }
}

impl CoopBackend for GameStateBackend {
    fn name(&self) -> &'static str {
        "world"
    }
    fn peer<'a>(&self, d: &'a Data, host: bool) -> Result<Box<dyn BackendPeer + 'a>, String> {
        let mut gs = GameState::new(&d.order);
        let table = Arc::new(coop_session::policy_ids::resolve_policy(&self.table, &d.order)?.0);
        if host {
            gs.set_authority(Authority::Host);
            gs.enable_coop_policy(table.clone(), 0);
            // the guest of these tests is player 1: the host keeps its rows
            gs.register_player(1);
            // these tests are about replication, not tether zones: the shared
            // quest's tether is off so SetStage applies at once
            if let Some(g) = gs.coop.gate.as_mut() {
                g.tether_mode = coop_policy::TetherMode::Off;
            }
        } else {
            gs.set_authority(Authority::Guest);
            gs.enable_shadows(ShadowPlan::from_policy(&table), 1);
        }
        Ok(Box::new(WPeer { d, gs }))
    }
}

/// Snapshot entries of a peer's shared state.
pub fn snapshot_of(p: &dyn BackendPeer) -> Vec<SnapEntry> {
    p.export()
        .into_iter()
        .enumerate()
        .map(|(i, (form, field))| SnapEntry {
            form,
            field,
            rev: Rev(i as u64 + 1),
        })
        .collect()
}

/// A delta that sets the given fields.
pub fn delta_of(fields: Vec<(CForm, Field)>, base: u64) -> Delta {
    let n = fields.len() as u64;
    Delta {
        tick: Tick(1),
        base_rev: Rev(base),
        rev: Rev(base + n),
        changes: fields
            .into_iter()
            .enumerate()
            .map(|(i, (form, field))| Change {
                form,
                field,
                rev_off: i as u64 + 1,
            })
            .collect(),
        complete: true,
    }
}

fn need<T>(r: Result<T, String>) -> Result<T, Verdict> {
    r.map_err(|e| {
        if e.starts_with("PENDING") {
            Verdict::Pending(e)
        } else {
            Verdict::Fail(e)
        }
    })
}

/// A shared quest the tests drive (a Prologue quest every copy has).
const SHARED_QUEST: &str = "VCG03";
/// A per-player quest in the policy table.
const PLAYER_QUEST: &str = "VCG01";
const STAGE: u16 = 10;

type Test = fn(&dyn CoopBackend, &Data, bool) -> Result<(), Verdict>;

fn bad(s: impl Into<String>) -> Verdict {
    Verdict::Fail(s.into())
}

fn snapshot_carries_stage(b: &dyn CoopBackend, d: &Data, mutate: bool) -> Result<(), Verdict> {
    let mut host = need(b.peer(d, true))?;
    let mut guest = need(b.peer(d, false))?;
    if !host.set_stage(SHARED_QUEST, STAGE) {
        return Err(bad("host refused SetStage"));
    }
    let mut snap = snapshot_of(&*host);
    if mutate {
        snap.clear();
    }
    guest.apply_snapshot(&snap);
    if guest.stage(SHARED_QUEST) != Some(STAGE) {
        return Err(bad(format!(
            "guest stage {:?}, want {STAGE}",
            guest.stage(SHARED_QUEST)
        )));
    }
    if guest.shard_hashes() != host.shard_hashes() {
        return Err(bad("shard hashes differ after the snapshot"));
    }
    Ok(())
}

fn delta_carries_change(b: &dyn CoopBackend, d: &Data, mutate: bool) -> Result<(), Verdict> {
    let mut host = need(b.peer(d, true))?;
    let mut guest = need(b.peer(d, false))?;
    guest.apply_snapshot(&snapshot_of(&*host));
    let before: std::collections::BTreeSet<String> =
        host.export().iter().map(|x| format!("{x:?}")).collect();
    host.set_stage(SHARED_QUEST, STAGE);
    let after = host.export();
    let changed: Vec<(CForm, Field)> = after
        .into_iter()
        .filter(|x| !before.contains(&format!("{x:?}")))
        .collect();
    if changed.is_empty() {
        return Err(bad("SetStage changed no shared field"));
    }
    let delta = delta_of(if mutate { Vec::new() } else { changed }, 0);
    guest.apply_delta(&delta);
    if guest.stage(SHARED_QUEST) != Some(STAGE) || guest.shard_hashes() != host.shard_hashes() {
        return Err(bad("guest does not equal the host after the delta"));
    }
    Ok(())
}

fn snapshot_is_idempotent(b: &dyn CoopBackend, d: &Data, mutate: bool) -> Result<(), Verdict> {
    let mut host = need(b.peer(d, true))?;
    let mut guest = need(b.peer(d, false))?;
    host.set_stage(SHARED_QUEST, STAGE);
    let snap = snapshot_of(&*host);
    guest.apply_snapshot(&snap);
    if mutate {
        // local drift on the guest: the second snapshot has to undo it
        guest.set_global("GameDaysPassed", 99.0);
    }
    let (set, cleared) = guest.apply_snapshot(&snap);
    if (set, cleared) != (0, 0) {
        return Err(bad(format!(
            "a second identical snapshot changed {set} slots and cleared {cleared}"
        )));
    }
    Ok(())
}

fn guest_runs_no_quest_script(b: &dyn CoopBackend, d: &Data, mutate: bool) -> Result<(), Verdict> {
    let mut guest = need(b.peer(d, mutate))?;
    let ran = guest.set_stage(SHARED_QUEST, STAGE);
    if ran || guest.stage(SHARED_QUEST).is_some() {
        return Err(bad("a guest ran SetStage"));
    }
    Ok(())
}

fn per_player_stays_private(b: &dyn CoopBackend, d: &Data, mutate: bool) -> Result<(), Verdict> {
    let mut host = need(b.peer(d, true))?;
    host.register_player(1);
    host.register_player(2);
    if !host.set_stage_as(if mutate { 0 } else { 1 }, PLAYER_QUEST, STAGE) {
        return Err(bad("the host refused player 1's stage"));
    }
    if host.player_stage(1, PLAYER_QUEST) != Some(STAGE) {
        return Err(bad(format!(
            "player 1 has stage {:?}",
            host.player_stage(1, PLAYER_QUEST)
        )));
    }
    if host.player_stage(2, PLAYER_QUEST).is_some() {
        return Err(bad("player 2's quest moved with player 1's"));
    }
    if host.stage(PLAYER_QUEST).is_some() {
        return Err(bad("a per-player quest wrote the shared stage"));
    }
    Ok(())
}

fn shadow_rows_reach_guest(b: &dyn CoopBackend, d: &Data, mutate: bool) -> Result<(), Verdict> {
    let mut host = need(b.peer(d, true))?;
    let mut guest = need(b.peer(d, false))?;
    host.register_player(1);
    host.set_stage_as(1, PLAYER_QUEST, STAGE);
    let mut snap = snapshot_of(&*host);
    if mutate {
        snap.truncate(snap.len() / 2);
    }
    guest.apply_snapshot(&snap);
    if guest.shard_hashes() != host.shard_hashes() {
        return Err(bad(
            "guest shard hashes differ: per-player rows are not replicated",
        ));
    }
    Ok(())
}

fn global_replicates(b: &dyn CoopBackend, d: &Data, mutate: bool) -> Result<(), Verdict> {
    let mut host = need(b.peer(d, true))?;
    let mut guest = need(b.peer(d, false))?;
    guest.apply_snapshot(&snapshot_of(&*host));
    if !host.set_global("GameDaysPassed", 12.0) && !host.set_global("TimeScale", 12.0) {
        return Err(bad("no clock global to set in this data"));
    }
    let name = if host.global("GameDaysPassed").is_some() {
        "GameDaysPassed"
    } else {
        "TimeScale"
    };
    let snap = snapshot_of(&*host);
    if !mutate {
        guest.apply_snapshot(&snap);
    }
    if guest.global(name).map(|v| (v * 64.0).floor())
        != host.global(name).map(|v| (v * 64.0).floor())
    {
        return Err(bad("a changed global did not reach the guest"));
    }
    Ok(())
}

/// The conformance tests: id and function.
pub const TESTS: &[(&str, Test)] = &[
    ("snapshot_carries_stage", snapshot_carries_stage),
    ("delta_carries_change", delta_carries_change),
    ("snapshot_is_idempotent", snapshot_is_idempotent),
    ("guest_runs_no_quest_script", guest_runs_no_quest_script),
    ("per_player_stays_private", per_player_stays_private),
    ("shadow_rows_reach_guest", shadow_rows_reach_guest),
    ("global_replicates", global_replicates),
];

/// The backend by name.
pub fn by_name(name: &str) -> Result<Box<dyn CoopBackend>, String> {
    match name {
        "world" => Ok(Box::new(GameStateBackend::new()?)),
        "engine" => Ok(Box::new(EngineBackend)),
        other => Err(format!("unknown backend {other} (world or engine)")),
    }
}

/// Runs one conformance test; `mutate` runs its twin (which must fail).
pub fn run_test(b: &dyn CoopBackend, d: &Data, id: &str, mutate: bool) -> Verdict {
    let Some((_, f)) = TESTS.iter().find(|(n, _)| *n == id) else {
        return Verdict::Pending(format!("no conformance test {id}"));
    };
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(b, d, mutate))) {
        Ok(Ok(())) => Verdict::Pass,
        Ok(Err(v)) => v,
        Err(_) => Verdict::Fail("panic".into()),
    }
}
