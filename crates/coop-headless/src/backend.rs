//! Backend conformance: behavioural co-op tests written against a trait, so
//! the same checklist rows can run against the hand-written world layer
//! (`world::GameState`, [`WorldBackend`]) today and the translated engine
//! (`crates/engine`, `Mem`) when it carries the game.
//!
//! A [`CoopBackend`] hands out peers; a [`BackendPeer`] is one machine's
//! game world seen only through what co-op needs: set a quest stage, read it
//! back, export the shared state, apply a snapshot or delta from the host,
//! and keep per-player rows. The tests in [`TESTS`] use nothing else.
//! [`EngineBackend`] is the engine: one game on the upstream engine in a child
//! process for each peer ([`crate::engine_peer`]), when `COOP_ENGINE_PEER`
//! names the peer server's program. Without it every test is
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
    /// Whether this backend keeps per-player rows. A test of them is PENDING on a backend
    /// that does not yet.
    fn has_player_rows(&self) -> bool {
        true
    }
    /// The shared state before this game's first frame of play. `None` on a backend that
    /// does not keep it.
    fn start_state(&self) -> Option<Vec<(CForm, Field)>> {
        None
    }
    /// Runs this many frames of the game. False on a backend that has no frames.
    fn frames(&mut self, _n: u32) -> bool {
        false
    }
    /// Runs a console line as a script of the game would run it: the guest's gate sees it.
    /// A test's stand-in for a script that the game runs.
    fn script_line(&mut self, _line: &str) -> bool {
        false
    }
    /// Runs a console line as the adapter's own: the gate lets it pass.
    fn adapter_line(&mut self, _line: &str) -> bool {
        false
    }
    /// What the guest's gate counted. `None` when this game has no gate.
    fn gate_stats(&self) -> Option<GateStats> {
        None
    }
    /// The changes of shared quests that the gate turned to the host, each one time. The
    /// queue is empty after.
    fn intents(&mut self) -> Vec<String> {
        Vec::new()
    }
    /// The globals that the game moves itself (its clock), each with the difference from
    /// the host that is no leak.
    fn set_moving(&mut self, _limits: &[(&str, f32)]) -> bool {
        false
    }
    /// Compares each shared slot with the host's last value. With `restore`, the host's
    /// values come back. `None` when this backend has no audit, or before a snapshot.
    fn audit(&mut self, _restore: bool) -> Option<AuditReport> {
        None
    }
    /// The stage of a quest that each player runs alone, in this game.
    fn local_stage(&self, _quest: &str) -> Option<u16> {
        None
    }
}

/// What a guest's gate counted.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GateStats {
    /// Script runs that reached the gate.
    pub runs: u64,
    /// Script runs that the gate left to the host.
    pub stopped: u64,
    /// Changes of a shared quest that became an intent.
    pub guarded: u64,
    /// The gate saw no script for five seconds of play.
    pub blind: bool,
}

/// One audit of a guest's shared state.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AuditReport {
    /// How many slots were not the host's value.
    pub leaks: u32,
    /// How many slots got the host's value back.
    pub restored: u32,
    /// The script that reached the gate last (a form id).
    pub last_script: u32,
    /// The leaks by name: form id and field kind (the first 64).
    pub named: Vec<(u32, u8)>,
}

/// Which story code a guest's game runs itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuestRules<'r> {
    /// The campaign policy's rules: each per-player quest and each `guest_local` script.
    Policy,
    /// The policy's rules, and these quests local too (editor ids).
    PolicyAnd(&'r [&'r str]),
    /// A gate with no local quest and no local script.
    Closed,
    /// No gate: the guest's game runs each script.
    Open,
}

/// A game world that can host a co-op session.
pub trait CoopBackend: Sync {
    /// Name for reports.
    fn name(&self) -> &'static str;
    /// A new machine: the host (authority) or a guest (runs no quest script).
    fn peer<'a>(&self, d: &'a Data, host: bool) -> Result<Box<dyn BackendPeer + 'a>, String>;
    /// A new guest with these rules for its script gate. PENDING on a backend whose guest
    /// has no script gate.
    fn guest<'a>(
        &self,
        _d: &'a Data,
        _rules: GuestRules,
    ) -> Result<Box<dyn BackendPeer + 'a>, String> {
        Err("PENDING (this backend has no script gate)".into())
    }
}

/// The game's own program on the upstream engine, one child process for each peer
/// ([`crate::engine_peer`]). PENDING when no peer server is named.
pub struct EngineBackend;

/// The scripts that each machine runs itself: the policy's `guest_local` lists, each script
/// by its form id in this load order.
fn local_scripts(table: &PolicyTable, d: &Data) -> Result<Vec<u32>, String> {
    let mut scripts = Vec::new();
    for row in table.rows() {
        for name in &row.guest_local {
            // `Plugin.esm:LOCALID`: the script's id in its own plugin.
            let Some((plugin, local)) = name.rsplit_once(':') else {
                return Err(format!("{}: bad guest_local entry {name}", row.edid));
            };
            let local = u32::from_str_radix(local, 16)
                .map_err(|_| format!("{}: bad guest_local id {name}", row.edid))?;
            let index = d
                .order
                .plugins()
                .iter()
                .find(|p| p.name.eq_ignore_ascii_case(plugin))
                .map(|p| u32::from(p.load_index));
            // A plugin that is not loaded has no script to run.
            if let Some(index) = index {
                scripts.push((index << 24) | (local & 0x00FF_FFFF));
            }
        }
    }
    scripts.sort_unstable();
    scripts.dedup();
    Ok(scripts)
}

/// What two games of a load order share, and a guest's gate for it. Shared: each quest that
/// the policy does not give to each player separately, and each global. The gate: the
/// policy's per-player quests and its `guest_local` scripts.
fn policy_setup(
    d: &Data,
) -> Result<(crate::engine_peer::Catalog, crate::engine_peer::Gate), String> {
    let bundled = PolicyTable::bundled().map_err(|e| format!("{e:?}"))?;
    let table = coop_session::policy_ids::resolve_policy(&bundled, &d.order)?.0;
    let own = ShadowPlan::from_policy(&table).quests;
    let ids = |kind: &[u8; 4]| -> Vec<u32> {
        d.order
            .records_of_type(esm::FourCC::new(kind))
            .map(|r| r.form_id)
            .filter(|f| !own.contains(f))
            .map(|f| f.0)
            .collect()
    };
    let scripts = local_scripts(&table, d)?;
    let catalog = crate::engine_peer::Catalog {
        quests: ids(b"QUST"),
        globals: ids(b"GLOB"),
        local_scripts: scripts.clone(),
    };
    let gate = crate::engine_peer::Gate {
        quests: own.iter().map(|f| f.0).collect(),
        scripts,
    };
    Ok((catalog, gate))
}

impl EngineBackend {
    fn start<'a>(
        &self,
        d: &'a Data,
        host: bool,
        gate: Option<crate::engine_peer::Gate>,
    ) -> Result<Box<dyn BackendPeer + 'a>, String> {
        let Some(program) = crate::engine_peer::program() else {
            return Err("PENDING (engine backend)".into());
        };
        let catalog = policy_setup(d)?.0;
        let peer =
            crate::engine_peer::EnginePeer::start(d, &program, host, &catalog, gate.as_ref())?;
        Ok(Box::new(peer))
    }
}

impl CoopBackend for EngineBackend {
    fn name(&self) -> &'static str {
        "engine"
    }
    fn peer<'a>(&self, d: &'a Data, host: bool) -> Result<Box<dyn BackendPeer + 'a>, String> {
        if host {
            self.start(d, true, None)
        } else {
            // A guest of a session always has its gate.
            self.guest(d, GuestRules::Policy)
        }
    }
    fn guest<'a>(
        &self,
        d: &'a Data,
        rules: GuestRules,
    ) -> Result<Box<dyn BackendPeer + 'a>, String> {
        if crate::engine_peer::program().is_none() {
            return Err("PENDING (engine backend)".into());
        }
        let gate = match rules {
            GuestRules::Open => None,
            GuestRules::Closed => Some(crate::engine_peer::Gate::default()),
            GuestRules::Policy => Some(policy_setup(d)?.1),
            GuestRules::PolicyAnd(more) => {
                let mut gate = policy_setup(d)?.1;
                for quest in more {
                    let form = d
                        .order
                        .form_by_editor_id(quest)
                        .ok_or(format!("no quest {quest} in this data"))?;
                    gate.quests.push(form.0);
                }
                Some(gate)
            }
        };
        self.start(d, false, gate)
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

pub(crate) fn need<T>(r: Result<T, String>) -> Result<T, Verdict> {
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

/// One conformance test: the backend, the game's data, and the twin flag.
pub type Test = fn(&dyn CoopBackend, &Data, bool) -> Result<(), Verdict>;

pub(crate) fn bad(s: impl Into<String>) -> Verdict {
    Verdict::Fail(s.into())
}

/// A test of per-player rows is PENDING on a backend that does not keep them yet.
fn need_player_rows(p: &dyn BackendPeer) -> Result<(), Verdict> {
    if p.has_player_rows() {
        Ok(())
    } else {
        Err(Verdict::Pending(
            "PENDING (engine backend: per-player rows)".into(),
        ))
    }
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
    need_player_rows(&*host)?;
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
    need_player_rows(&*host)?;
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

/// The tests of a guest's script gate (`crate::gate_rows`): id and function. They need a
/// backend that runs the game's own scripts.
pub const GATE_TESTS: &[(&str, Test)] = crate::gate_rows::TESTS;

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
    let Some((_, f)) = TESTS.iter().chain(GATE_TESTS).find(|(n, _)| *n == id) else {
        return Verdict::Pending(format!("no conformance test {id}"));
    };
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(b, d, mutate))) {
        Ok(Ok(())) => Verdict::Pass,
        Ok(Err(v)) => v,
        Err(_) => Verdict::Fail("panic".into()),
    }
}
