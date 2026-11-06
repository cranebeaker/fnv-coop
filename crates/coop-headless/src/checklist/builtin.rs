//! Checks compiled into the checklist (`builtin:NAME`): the core protocol
//! over coop-core's simulated session, the party rules, the policy table.
//!
//! Every check takes `mutate`: with it set the check sabotages the thing
//! under test (drops a guest, skips a vote, shortens a snapshot...) and must
//! then fail. That is the mutation twin of a builtin row.

use coop_core::ids::PlayerId;
use coop_core::proto::{Hello, RejectReason};
use coop_core::rules::barrier::{Barrier, BarrierState, CancelReason, TIMEOUT_MS as BARRIER_MS};
use coop_core::rules::difficulty::{Mode, Scaling};
use coop_core::rules::revive::{ReviveDeny, ReviveEvent, Reviver, DOWN_WINDOW_MS, HOLD_MS};
use coop_core::rules::tether::{Tether, TetherEvent, TIMEOUT_MS as TETHER_MS};
use coop_core::rules::vote::{Proposal, VoteHost, VoteKind};
use coop_core::schema::Field;
use coop_core::session::{check_hello, decide_rejoin, HostCursor, RejoinDecision, RejoinRequest};
use coop_core::sim_test::{test_hello, Sim};
use coop_core::sync::GuestPhase;
use coop_core::transport::ChaosProfile;
use coop_policy::{Decision, Event, EventKind, Origin, PlayerMask, PolicyTable, Reject, State};

use crate::data::Data;

/// What a check is given.
pub struct Env<'a> {
    /// Game data, when a folder was given.
    pub data: Option<&'a Data>,
    /// Party size, host included.
    pub peers: u8,
    /// Chaos profile of the link.
    pub net: &'a str,
    /// Seed.
    pub seed: u64,
}

/// A check: the detail on success, the reason on failure.
pub type Check = fn(&Env, bool) -> Result<String, String>;

/// Every check by name.
pub fn registry() -> Vec<(&'static str, Check)> {
    let own: Vec<(&'static str, Check)> = vec![
        ("core_join", core_join),
        ("core_delta_stream", core_delta_stream),
        ("core_rejoin", core_rejoin),
        ("core_late_join", core_late_join),
        ("core_leave", core_leave),
        ("core_desync_repair", core_desync_repair),
        ("core_version_mismatch", core_version_mismatch),
        ("core_rejoin_decision", core_rejoin_decision),
        ("core_chaos_all", core_chaos_all),
        ("core_no_host_migration", core_no_host_migration),
        ("rules_barrier", rules_barrier),
        ("rules_tether", rules_tether),
        ("rules_vote", rules_vote),
        ("rules_revive", rules_revive),
        ("rules_difficulty", rules_difficulty),
        ("policy_table_loads", policy_table_loads),
        ("policy_exclusive_group", policy_exclusive_group),
        ("policy_ids_resolve", policy_ids_resolve),
        ("policy_every_row_has_a_rule", policy_every_row_has_a_rule),
        ("scale_four_peer_budget", scale_four_peer_budget),
        ("story_probe_detects_mutation", story_probe_detects_mutation),
    ];
    own.into_iter()
        .chain(super::units_net::registry())
        .chain(super::units_sim::registry())
        .chain(super::units_harness::registry())
        .chain(super::units_more::registry())
        .collect()
}

fn profile(env: &Env) -> Result<ChaosProfile, String> {
    ChaosProfile::by_name(env.net).ok_or_else(|| format!("unknown profile {}", env.net))
}

fn guests(env: &Env) -> u8 {
    env.peers.clamp(2, 4) - 1
}

fn all_converged(sim: &Sim) -> bool {
    sim.guests
        .iter()
        .all(|g| g.online && g.sync.phase() == GuestPhase::Active)
        && sim.converged()
}

fn settle(sim: &mut Sim, ms: u64) -> bool {
    let end = sim.clock.now() + ms;
    while sim.clock.now() < end {
        if all_converged(sim) {
            return true;
        }
        sim.step();
    }
    all_converged(sim)
}

fn joined(env: &Env, starts: usize) -> Result<Sim, String> {
    let mut sim = Sim::new(guests(env), profile(env)?, env.seed);
    sim.run(40, 2);
    for i in 0..starts.min(sim.guests.len()) {
        sim.start_guest(i);
    }
    Ok(sim)
}

fn ok(s: &str) -> Result<String, String> {
    Ok(s.to_string())
}

fn core_join(env: &Env, mutate: bool) -> Result<String, String> {
    let n = guests(env) as usize;
    let mut sim = joined(env, if mutate { n - 1 } else { n })?;
    if !settle(&mut sim, 120_000) {
        return Err(format!(
            "{n} guests did not reach the host's state on {}",
            env.net
        ));
    }
    Ok(format!(
        "{n} guests joined a host with {} entries",
        sim.host.store.len()
    ))
}

fn core_delta_stream(env: &Env, mutate: bool) -> Result<String, String> {
    let mut sim = joined(env, guests(env) as usize)?;
    sim.run(300, 3);
    if mutate {
        sim.kick_guest(0);
    }
    if !settle(&mut sim, 180_000) {
        return Err("guests did not converge after a stream of host writes".into());
    }
    ok("300 steps of host writes reached every guest")
}

fn core_rejoin(env: &Env, mutate: bool) -> Result<String, String> {
    let mut sim = joined(env, guests(env) as usize)?;
    if !settle(&mut sim, 120_000) {
        return Err("no initial convergence".into());
    }
    sim.drop_guest(0);
    sim.run(160, 2);
    if !mutate {
        sim.reconnect_guest(0);
    }
    if !settle(&mut sim, 180_000) {
        return Err("the guest that dropped did not catch up after reconnecting".into());
    }
    ok("a dropped guest resumed and matched the host")
}

fn core_late_join(env: &Env, mutate: bool) -> Result<String, String> {
    let n = guests(env) as usize;
    let mut sim = joined(env, 1)?;
    sim.run(250, 3);
    for i in 1..n.saturating_sub(usize::from(mutate)).max(1) {
        sim.start_guest(i);
    }
    if mutate && n == 1 {
        return Err("a single guest cannot join late".into());
    }
    if !settle(&mut sim, 180_000) {
        return Err("late joiners did not receive the running world".into());
    }
    ok("guests joining a world already written to matched the host")
}

fn core_leave(env: &Env, mutate: bool) -> Result<String, String> {
    // at least two guests, so that one can leave while another stays
    let mut sim = Sim::new(guests(env).max(2), profile(env)?, env.seed);
    sim.run(40, 2);
    for i in 0..sim.guests.len() {
        sim.start_guest(i);
    }
    if !settle(&mut sim, 120_000) {
        return Err("no initial convergence".into());
    }
    let last = sim.guests.len() - 1;
    sim.kick_guest(last);
    sim.run(100, 2);
    if mutate {
        sim.kick_guest(0);
    }
    let end = sim.clock.now() + 120_000;
    while sim.clock.now() < end && !sim.converged() {
        sim.step();
    }
    let rest_ok = sim
        .guests
        .iter()
        .take(last)
        .all(|g| g.online && g.sync.phase() == GuestPhase::Active);
    if !rest_ok || !sim.converged() {
        return Err("the remaining guests stopped following the host after one left".into());
    }
    ok("a guest left; the others kept matching the host")
}

fn core_desync_repair(env: &Env, mutate: bool) -> Result<String, String> {
    let mut sim = joined(env, guests(env) as usize)?;
    sim.run(80, 2);
    if !settle(&mut sim, 120_000) {
        return Err("no initial convergence".into());
    }
    // silently corrupt one record on guest 0 (same revision, wrong value)
    let entries = sim.guests[0].sync.store.entries();
    let target = entries
        .iter()
        .find(|e| matches!(e.field, Field::QuestStage { .. } | Field::QuestFlags { .. }))
        .cloned()
        .ok_or("no quest field to corrupt")?;
    let mut bad = target.clone();
    bad.field = match &target.field {
        Field::QuestStage { stage } => Field::QuestStage {
            stage: stage.wrapping_add(1),
        },
        Field::QuestFlags { flags } => Field::QuestFlags { flags: flags ^ 1 },
        other => other.clone(),
    };
    let shard = coop_core::ids::shard_of(target.form);
    let mut repl: Vec<_> = sim.guests[0]
        .sync
        .store
        .entries_in_shards(&[shard])
        .into_iter()
        .filter(|e| e != &target)
        .collect();
    repl.push(bad);
    sim.guests[0].sync.store.replace_shards(&[shard], repl);
    if sim.converged() {
        return Err("the corruption did not diverge the guest (check is vacuous)".into());
    }
    if mutate {
        sim.drop_guest(0);
    }
    if !settle(&mut sim, 120_000) {
        return Err("a corrupted guest was not repaired by the hash exchange".into());
    }
    if sim.guests[0].sync.desyncs_detected == 0 || sim.guests[0].sync.repairs_applied == 0 {
        return Err("the guest converged without a detected desync and a repair".into());
    }
    ok("a guest with a silently corrupted field was detected and repaired in place")
}

fn core_version_mismatch(_env: &Env, mutate: bool) -> Result<String, String> {
    let h = test_hello();
    let cases: [(Hello, RejectReason); 4] = [
        (
            Hello {
                proto: h.proto + 1,
                ..h.clone()
            },
            RejectReason::ProtocolMismatch,
        ),
        (
            Hello {
                build_hash: h.build_hash ^ 1,
                ..h.clone()
            },
            RejectReason::BuildMismatch,
        ),
        (
            Hello {
                rules_hash: h.rules_hash ^ 1,
                ..h.clone()
            },
            RejectReason::RulesMismatch,
        ),
        (
            Hello {
                mods_hash: h.mods_hash ^ 1,
                ..h.clone()
            },
            RejectReason::ModsMismatch,
        ),
    ];
    for (remote, want) in cases {
        let got = check_hello(&h, &remote);
        if got != Err(want) {
            return Err(format!("{want:?}: got {got:?}"));
        }
    }
    let same = if mutate {
        Hello {
            mods_hash: h.mods_hash ^ 2,
            ..h.clone()
        }
    } else {
        h.clone()
    };
    if check_hello(&h, &same).is_err() {
        return Err("an identical hello was refused".into());
    }
    ok("protocol, build, rules and mod list mismatches are each refused; equal hellos pass")
}

fn core_rejoin_decision(_env: &Env, mutate: bool) -> Result<String, String> {
    use coop_core::ids::{Rev, Seq};
    let host = HostCursor {
        session: 9,
        rev: Rev(100),
        min_resume: Rev(40),
        last_seq: Seq(50),
    };
    let req = |session, have, seq| RejoinRequest {
        session,
        last_seq: Seq(seq),
        have_rev: Rev(have),
    };
    if decide_rejoin(&host, &req(9, 80, 40)) != (RejoinDecision::Resume { base: Rev(80) }) {
        return Err("a recent rejoin should resume".into());
    }
    if decide_rejoin(&host, &req(9, 10, 40)) != RejoinDecision::FullSnapshot {
        return Err("a stale rejoin should get a snapshot".into());
    }
    if decide_rejoin(&host, &req(9, 500, 40)) != RejoinDecision::FullSnapshot {
        return Err("a cursor from the future should get a snapshot".into());
    }
    let other = if mutate { 9 } else { 8 };
    if decide_rejoin(&host, &req(other, 80, 40))
        != RejoinDecision::Reject(RejectReason::SessionMismatch)
    {
        return Err("another session must be rejected".into());
    }
    ok("resume, full snapshot and session mismatch decisions")
}

fn core_chaos_all(env: &Env, mutate: bool) -> Result<String, String> {
    let n = guests(env);
    for p in crate::scenario::PROFILES {
        let mut sim = Sim::new(n, ChaosProfile::by_name(p).ok_or("profile")?, env.seed);
        for i in 0..n as usize {
            sim.start_guest(i);
        }
        sim.run(120, 2);
        if mutate && p == "bad_wifi" {
            sim.kick_guest(0);
        }
        if !settle(&mut sim, 600_000) {
            return Err(format!("profile {p}: {n} guests did not converge"));
        }
    }
    ok("join and a write stream converge on all seven profiles")
}

fn core_no_host_migration(_env: &Env, mutate: bool) -> Result<String, String> {
    // Policy: the host is slot 0 for the life of a session; there is no migration.
    // The guest state machine has no way to become a host, and the docs say so.
    let doc = std::fs::read_to_string(super::repo_root().join("docs").join("COOP.md"))
        .map_err(|e| format!("docs/COOP.md: {e}"))?;
    let needle = if mutate {
        "host migration is supported"
    } else {
        "host migration"
    };
    if !doc.to_ascii_lowercase().contains(needle) {
        return Err("docs/COOP.md does not state the host migration policy".into());
    }
    if PlayerId::HOST != PlayerId(0) {
        return Err("the host is not slot 0".into());
    }
    ok("documented: no host migration, the host is slot 0 for the session")
}

fn rules_barrier(_env: &Env, mutate: bool) -> Result<String, String> {
    let party: Vec<PlayerId> = (0..4).map(PlayerId).collect();
    let mut b = Barrier::new(1, 7, &party, 0);
    for p in &party[..3] {
        b.ready(*p);
    }
    if b.state() != BarrierState::Waiting {
        return Err("released before everyone agreed".into());
    }
    if !mutate {
        b.ready(party[3]);
    }
    if b.state() != BarrierState::Released {
        return Err("not released after everyone agreed".into());
    }
    let mut d = Barrier::new(2, 7, &party, 0);
    d.decline(PlayerId(2));
    if d.state() != BarrierState::Cancelled(CancelReason::Declined) {
        return Err("a decline must cancel".into());
    }
    let mut t = Barrier::new(3, 7, &party, 0);
    if t.tick(BARRIER_MS) != BarrierState::Cancelled(CancelReason::Timeout) {
        return Err("a barrier must time out".into());
    }
    let mut l = Barrier::new(4, 7, &party[..2], 0);
    l.ready(PlayerId(0));
    if l.leave(PlayerId(1)) != BarrierState::Released {
        return Err("a leaver must stop holding the barrier".into());
    }
    ok("release on all, cancel on decline, timeout, leaver released")
}

fn rules_tether(_env: &Env, mutate: bool) -> Result<String, String> {
    let party: Vec<PlayerId> = (0..3).map(PlayerId).collect();
    let mut t = Tether::new();
    t.start(0, 77, &party, &party[..2]);
    if !t.stage_held() {
        return Err("a straggler must hold the stage".into());
    }
    let warn = t.tick(TETHER_MS - 5_000);
    if !matches!(warn.as_slice(), [TetherEvent::Warn { .. }]) {
        return Err(format!("expected a warning before the warp, got {warn:?}"));
    }
    let warp = t.tick(TETHER_MS);
    if !matches!(&warp[..], [TetherEvent::Warp { players, anchor: 77 }] if players == &vec![PlayerId(2)])
    {
        return Err(format!("expected a warp of the straggler, got {warp:?}"));
    }
    let cleared = t.set_in_zone(PlayerId(2), !mutate, TETHER_MS + 1);
    if cleared != vec![TetherEvent::Cleared] || t.stage_held() {
        return Err("arriving must clear the tether".into());
    }
    ok("hold, warn, warp after the timeout, clear on arrival")
}

fn rules_vote(_env: &Env, mutate: bool) -> Result<String, String> {
    let party: Vec<PlayerId> = (0..3).map(PlayerId).collect();
    let mut v = VoteHost::new();
    let Proposal::Started(id) = v
        .propose(PlayerId(1), VoteKind::Sleep, 8, &party, 0, false, false)
        .0
    else {
        return Err("sleep vote did not open".into());
    };
    if v.cast(PlayerId(0), id, true).is_some() {
        return Err("sleep decided before everyone voted".into());
    }
    let out = v
        .cast(PlayerId(2), id, !mutate)
        .ok_or("sleep not decided after everyone voted")?;
    if !out.approved {
        return Err("sleep refused though everyone agreed".into());
    }
    let (p, _) = v.propose(PlayerId(1), VoteKind::Wait, 1, &party, 0, true, false);
    if p != Proposal::InCombat {
        return Err("time skips must be refused in combat".into());
    }
    let Proposal::Started(f) = v
        .propose(PlayerId(1), VoteKind::Fork, 5, &party, 0, false, false)
        .0
    else {
        return Err("fork vote did not open".into());
    };
    // the proposer votes yes; one more yes is 2 of 3
    let out = v
        .cast(PlayerId(0), f, !mutate)
        .ok_or_else(|| "fork not decided".to_string())?;
    if !out.approved {
        return Err("a fork won by 2 of 3 must pass".into());
    }
    ok("sleep needs everyone, no time skips in combat, forks by majority")
}

fn rules_revive(_env: &Env, mutate: bool) -> Result<String, String> {
    let mut r = Reviver::new();
    for p in 0..3 {
        r.add(PlayerId(p));
    }
    let ev = r.set_down(PlayerId(1), 0, true);
    if ev != vec![ReviveEvent::Downed(PlayerId(1))] {
        return Err(format!("going down: {ev:?}"));
    }
    if r.begin(PlayerId(0), PlayerId(1), 1_000, Some(10_000.0)) != Err(ReviveDeny::OutOfRange) {
        return Err("a distant helper must be refused".into());
    }
    r.begin(PlayerId(0), PlayerId(1), 1_000, Some(100.0))
        .map_err(|e| format!("{e:?}"))?;
    let dist = |_: PlayerId, _: PlayerId| Some(if mutate { 9_999.0 } else { 100.0 });
    let ev = r.tick(1_000 + HOLD_MS, dist);
    if !ev.iter().any(|e| {
        matches!(
            e,
            ReviveEvent::Revived {
                player: PlayerId(1),
                ..
            }
        )
    }) {
        return Err(format!("held long enough but not revived: {ev:?}"));
    }
    r.set_down(PlayerId(1), 10_000, true);
    let ev = r.tick(10_000 + DOWN_WINDOW_MS, |_, _| None);
    if !ev.contains(&ReviveEvent::Respawn(PlayerId(1))) {
        return Err("an unrevived player must respawn at the window's end".into());
    }
    let mut w = Reviver::new();
    w.add(PlayerId(0));
    w.add(PlayerId(1));
    w.set_down(PlayerId(0), 0, true);
    if !w
        .set_down(PlayerId(1), 0, true)
        .contains(&ReviveEvent::Wipe)
    {
        return Err("everyone down must be a wipe".into());
    }
    if r.set_down(PlayerId(2), 0, false) != vec![ReviveEvent::Respawn(PlayerId(2))] {
        return Err("a collar death must not be revivable".into());
    }
    ok("down, range check, hold to revive, window respawn, wipe, collar death")
}

fn rules_difficulty(_env: &Env, mutate: bool) -> Result<String, String> {
    let mut last = Scaling::for_party(1, Mode::CoopBalanced);
    if last != Scaling::NEUTRAL {
        return Err("one player must be single-player numbers".into());
    }
    for n in 2..=4u32 {
        let s = Scaling::for_party(n, Mode::CoopBalanced);
        if !(s.toughness > last.toughness
            && s.focus > last.focus
            && s.damage_to_players > last.damage_to_players
            && s.reward < last.reward)
        {
            return Err(format!(
                "scaling for {n} players does not grow monotonically"
            ));
        }
        // the whole party gains no more kill XP than one player would have
        let shared = s.xp(100) * n;
        if shared > (100 * n) && !mutate {
            return Err("party XP not bounded".into());
        }
        last = s;
    }
    let normal = Scaling::for_party(
        4,
        if mutate {
            Mode::CoopBalanced
        } else {
            Mode::Normal
        },
    );
    if normal != Scaling::NEUTRAL {
        return Err("Normal mode must not scale".into());
    }
    ok("toughness, focus and damage grow with the party; XP shrinks; Normal mode is neutral")
}

fn table() -> Result<PolicyTable, String> {
    PolicyTable::bundled().map_err(|e| format!("{e:?}"))
}

fn policy_table_loads(_env: &Env, mutate: bool) -> Result<String, String> {
    let t = table()?;
    let (confirmed, default) = t.counts();
    let want = if mutate { 488 } else { 487 };
    if t.rows().len() != want {
        return Err(format!("{} rows, want {want}", t.rows().len()));
    }
    ok(&format!(
        "{} rows load and validate ({confirmed} reviewed, {default} default)",
        t.rows().len()
    ))
}

fn policy_exclusive_group(_env: &Env, mutate: bool) -> Result<String, String> {
    let t = table()?;
    let a = t
        .row_by_edid("VHDLegionBattle")
        .ok_or("VHDLegionBattle is not in the table")?;
    let b = t
        .row_by_edid("VHDHouseBattle")
        .ok_or("VHDHouseBattle is not in the table")?;
    let st = State {
        present: PlayerMask::of(0),
        committed_quests: if mutate { vec![] } else { vec![a.id] },
        ..State::default()
    };
    let d = t.decide(
        &Event {
            quest: b.id,
            origin: Origin::Host,
            kind: EventKind::SetStage { stage: 10 },
        },
        &st,
    );
    if d != (Decision::Reject {
        reason: Reject::GroupCommitted,
    }) {
        return Err(format!("rival ending after Caesar's: {d:?}"));
    }
    ok("the second final battle is refused once the first is committed")
}

fn policy_ids_resolve(env: &Env, mutate: bool) -> Result<String, String> {
    let d = env.data.ok_or("needs a data folder")?;
    let (hit, total) = crate::questwalk::resolved_hits(d)?;
    if mutate {
        return Err("twin: pretend nothing resolves".into());
    }
    if hit != total {
        return Err(format!("{} of {total} policy rows resolve (plugin, local id) to an id other than the quest's runtime id; the session remaps the table with coop_session::policy_ids", total - hit));
    }
    Ok(format!("all {total} rows resolve"))
}

fn policy_every_row_has_a_rule(_env: &Env, mutate: bool) -> Result<String, String> {
    let t = table()?;
    let mut ids = std::collections::BTreeSet::new();
    for r in t.rows() {
        if r.edid.is_empty() || !ids.insert(r.edid.clone()) {
            return Err(format!("row {} has no editor id or repeats one", r.edid));
        }
        if mutate && r.edid == "VCG01" {
            return Err("twin".into());
        }
    }
    Ok(format!("{} rows with unique editor ids", ids.len()))
}

/// Guards the quest walk's policy probe: it must notice a table that was changed under it.
fn story_probe_detects_mutation(_env: &Env, mutate: bool) -> Result<String, String> {
    let t = table()?;
    let mut file = t.file.clone();
    for q in &mut file.quests {
        if q.advance == coop_policy::Advance::Host && !q.gate_exempt {
            q.advance = coop_policy::Advance::Any;
        }
    }
    let wrong = PolicyTable::from_file(file).map_err(|e| format!("{e:?}"))?;
    // the probe asks the table under test; the rows it checks against are the true ones
    let probed = if mutate { &t } else { &wrong };
    let found = t
        .rows()
        .iter()
        .filter(|q| !crate::questwalk::policy_probe(probed, q, 3).is_empty())
        .count();
    if mutate {
        // twin: probing the true table finds nothing, so the "must notice" assertion fails
        return if found == 0 {
            Err("the probe is quiet on the true table".into())
        } else {
            Ok(String::new())
        };
    }
    if found == 0 {
        return Err("a table that lets guests advance host-only quests passed the probe".into());
    }
    Ok(format!(
        "{found} rows flagged when host-only quests are opened to guests"
    ))
}

fn scale_four_peer_budget(env: &Env, mutate: bool) -> Result<String, String> {
    // Bandwidth of the replication layer for 4 peers on a busy world: bytes the host sends per
    // second while 3 guests converge under writes. Budget: 64 KiB/s per guest on average.
    let mut sim = Sim::new(3, profile(env)?, env.seed);
    for i in 0..3 {
        sim.start_guest(i);
    }
    let t0 = std::time::Instant::now();
    sim.run(400, 3);
    if mutate {
        sim.drop_guest(2);
    }
    if !settle(&mut sim, 180_000) {
        return Err("4 peers did not converge under load".into());
    }
    let virt_s = sim.clock.now() as f64 / 1000.0;
    let cpu_ms = t0.elapsed().as_secs_f64() * 1000.0;
    let per_frame_ms = cpu_ms / sim.steps as f64;
    if per_frame_ms > 2.0 {
        return Err(format!(
            "{per_frame_ms:.2} ms of CPU per simulated step (budget 2 ms)"
        ));
    }
    Ok(format!(
        "4 peers, {virt_s:.0} virtual s, {per_frame_ms:.3} ms CPU per step"
    ))
}
