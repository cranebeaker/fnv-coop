//! Per-quest co-op walk over the real game data.
//!
//! For every row of the bundled campaign policy table the quest's stage graph
//! is read from the plugins and walked on a host and N-1 guests: the world's
//! real `Runner` sets each stage (so the real result scripts run on the
//! host), the policy gate decides who may advance and when, held stages are
//! released the way a session would (barrier, tether zone, ready check), and
//! the host's replicated state is carried to the guests over coop-core's
//! seeded chaos transport. After the walk the run asserts the invariants of
//! the co-op design:
//!
//! * the shared shard hashes of host and guests are equal;
//! * a per-player quest changed only the advancing player's shadow;
//! * replaying every stage changes nothing (rewards and results run once);
//! * a guest never runs a quest script;
//! * no panic;
//! * the pure policy engine answers every event of the row as the row says
//!   ([`policy_probe`]).
//!
//! Script functions the single-player interpreter does not implement are
//! collected per quest as `SP-GAP`s: they are single-player gaps, not co-op
//! failures.

use std::collections::BTreeMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use coop_core::ids::PlayerId;
use coop_core::sim_test::Sim;
use coop_core::sync::GuestPhase;
use coop_core::transport::ChaosProfile;
use coop_policy::*;
use esm::FormId;
use world::coop::apply::{Ctx, Opts};
use world::coop::{Authority, GuestBridge, HostBridge, ShadowPlan, WorldBackend};
use world::dialogue::PLAYER_REF;
use world::scripting::{GameState, Runner};

use crate::data::Data;

/// Result of one quest.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QStatus {
    /// Every assertion held.
    Pass,
    /// A co-op assertion failed.
    Fail,
}

/// How deep the quest could be walked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Depth {
    /// Stages walked with the real runner.
    Full,
    /// The quest record is not in this load order: only the policy probe ran.
    PolicyOnly,
}

/// One quest's outcome over all the runs asked for.
#[derive(Clone, Debug)]
pub struct QuestResult {
    /// Editor id.
    pub edid: String,
    /// Plugin of the policy row.
    pub plugin: String,
    /// Questline label of the policy row.
    pub line: String,
    /// `Base game`, `Dead Money`...
    pub dlc: String,
    /// Pass or fail.
    pub status: QStatus,
    /// Full walk or policy only.
    pub depth: Depth,
    /// Stages in the quest record.
    pub stages: usize,
    /// Stage sets that ran.
    pub applied: usize,
    /// Stage sets held by the gate and released later.
    pub released: usize,
    /// Stage sets the policy refused (as the row says).
    pub refused: usize,
    /// Stage sets that went to one player's shadow.
    pub per_player: usize,
    /// Runs made (profile x peer count).
    pub runs: usize,
    /// What failed.
    pub fails: Vec<String>,
    /// Single-player gaps: function name and the first place it was hit.
    pub sp_gaps: BTreeMap<String, String>,
    /// Replay command.
    pub replay: String,
    /// Wall milliseconds.
    pub ms: u64,
}

/// What to run.
#[derive(Clone, Debug)]
pub struct WalkOpts {
    /// Chaos profiles.
    pub nets: Vec<String>,
    /// Peer counts (host included).
    pub peers: Vec<u8>,
    /// Seed.
    pub seed: u64,
    /// Worker threads.
    pub jobs: usize,
    /// Only quests whose editor id or plugin contains this (case-insensitive).
    pub filter: Option<String>,
}

impl Default for WalkOpts {
    fn default() -> Self {
        WalkOpts {
            nets: vec!["perfect".into(), "bad_wifi".into()],
            peers: vec![2],
            seed: 1,
            jobs: 4,
            filter: None,
        }
    }
}

/// The DLC a row belongs to.
pub fn dlc_of(row: &QuestRow) -> &'static str {
    let p = row.plugin.to_ascii_lowercase();
    if p.contains("deadmoney") {
        "Dead Money"
    } else if p.contains("honesthearts") {
        "Honest Hearts"
    } else if p.contains("oldworldblues") {
        "Old World Blues"
    } else if p.contains("lonesomeroad") {
        "Lonesome Road"
    } else if p.contains("gunrunners") {
        "Gun Runners' Arsenal"
    } else if row.edid.starts_with("NVDLC01") {
        "Dead Money"
    } else if row.edid.starts_with("NVDLC02") {
        "Honest Hearts"
    } else if row.edid.starts_with("NVDLC03") {
        "Old World Blues"
    } else if row.edid.starts_with("NVDLC04") {
        "Lonesome Road"
    } else {
        "Base game"
    }
}

/// Which invariant a failure text belongs to: `policy`, `converge`, `per_player`,
/// `replay`, `reward`, `guest_scripts`, `panic` or `run`.
pub fn fail_kind(f: &str) -> &'static str {
    if f.contains("[policy") {
        "policy"
    } else if f.contains("PANIC") {
        "panic"
    } else if f.contains("shard hashes differ") || f.contains("sees stage") {
        "converge"
    } else if f.contains("replay") {
        "replay"
    } else if f.contains("reward") {
        "reward"
    } else if f.contains("ran SetStage") || f.contains("refused script run") {
        "guest_scripts"
    } else if f.contains("quest stages") || f.contains("per-player") || f.contains("shadow") {
        "per_player"
    } else {
        "run"
    }
}

/// The bundled table with each quest's id replaced by its id in the load
/// order (the file stores a DLC quest under the load index it has when the
/// DLC runs alone). Returns the table and the editor ids not found.
pub fn remapped_table(d: &Data) -> Result<(PolicyTable, Vec<String>), String> {
    // the same call a live session makes at start
    let (t, rep) = coop_session::policy_ids::bundled_for(&d.order)?;
    Ok((t, rep.unmatched.into_iter().map(|(_, e)| e).collect()))
}

/// For every bundled row whose quest is loaded: does the session's resolution
/// (plugin file + local id) give the id the load order gives its editor id?
/// Returns (agreeing rows, rows checked).
pub fn resolved_hits(d: &Data) -> Result<(usize, usize), String> {
    let t = PolicyTable::bundled().map_err(|e| format!("{e:?}"))?;
    let (mut hit, mut total) = (0, 0);
    for q in t.rows().iter().filter(|q| !q.synthetic) {
        let Some(f) = d.order.form_by_editor_id(&q.edid) else {
            continue;
        };
        total += 1;
        if coop_session::policy_ids::resolve_row(&d.order, q) == Some(f.0) {
            hit += 1;
        }
    }
    Ok((hit, total))
}

fn bits(slots: impl IntoIterator<Item = u8>) -> PlayerMask {
    slots
        .into_iter()
        .fold(PlayerMask::NONE, |m, s| m.or(PlayerMask::of(s)))
}

fn want(fails: &mut Vec<String>, what: &str, d: &Decision, ok: impl Fn(&Decision) -> bool) {
    if !ok(d) {
        fails.push(format!("{what}: got {d:?}"));
    }
}

fn is_reject(d: &Decision, r: Reject) -> bool {
    matches!(d, Decision::Reject { reason } if *reason == r)
}

/// Asks the pure policy engine every question the row has an answer for and
/// returns the answers that disagree with the row. `n` is the party size.
pub fn policy_probe(t: &PolicyTable, row: &QuestRow, n: u8) -> Vec<String> {
    let mut f = Vec::new();
    let q = row.id;
    let n = n.clamp(2, 8);
    let present = bits(0..n);
    let base = State {
        present,
        ..State::default()
    };
    let ev = |origin, kind| Event {
        quest: q,
        origin,
        kind,
    };
    let set = |origin: Origin, st: u32| ev(origin, EventKind::SetStage { stage: st });
    let stage0 = row
        .tether
        .as_ref()
        .and_then(|t| t.stages.first().copied())
        .or_else(|| row.barrier.as_ref().and_then(|b| b.stages.first().copied()))
        .unwrap_or(1);

    // who may advance
    if !row.gate_exempt {
        match row.advance {
            Advance::Host => {
                let d = t.decide(&set(Origin::Player(1), 1), &base);
                want(&mut f, "host-only quest, guest intent", &d, |d| {
                    is_reject(d, Reject::NotHost)
                });
            }
            Advance::Initiator => {
                let st = State {
                    initiator: Some(0),
                    ..base.clone()
                };
                let d = t.decide(&set(Origin::Player(1), 1), &st);
                want(&mut f, "initiator quest, other player", &d, |d| {
                    is_reject(d, Reject::NotInitiator)
                });
                let st = State {
                    initiator: Some(1),
                    ..base.clone()
                };
                let d = t.decide(&set(Origin::Player(1), 1), &st);
                want(&mut f, "initiator quest, the initiator", &d, |d| {
                    !is_reject(d, Reject::NotInitiator) && !is_reject(d, Reject::NotHost)
                });
            }
            Advance::Any => {
                let d = t.decide(&set(Origin::Player(1), 1), &base);
                want(&mut f, "any-player quest, guest intent", &d, |d| {
                    !is_reject(d, Reject::NotHost) && !is_reject(d, Reject::NotInitiator)
                });
            }
        }
        let d = t.decide(&set(Origin::Player(n + 1), 1), &base);
        want(&mut f, "absent player", &d, |d| {
            is_reject(d, Reject::NotPresent)
        });
    }

    // per-player scope
    if row.scope == Scope::PerPlayer && !row.gate_exempt && row.exclusive.is_none() {
        let d = t.decide(&set(Origin::Player(1), 1), &base);
        if !is_reject(&d, Reject::NotHost)
            && !is_reject(&d, Reject::NotInitiator)
            && !is_reject(&d, Reject::GroupCommitted)
        {
            want(&mut f, "per-player stage by a player", &d, |d| {
                matches!(
                    d,
                    Decision::PerPlayerShadow {
                        slot: 1,
                        class: ShadowClass::QuestStage
                    }
                )
            });
        }
    } else if !row.gate_exempt && row.scope == Scope::Shared {
        // exclusive stages: the first committed wins
        if let Some(x) = &row.exclusive {
            if let [a, b, ..] = x.stages[..] {
                let st = State {
                    committed_stages: vec![a],
                    ..base.clone()
                };
                let d = t.decide(&set(Origin::Host, b), &st);
                want(
                    &mut f,
                    "exclusive stage after the other was committed",
                    &d,
                    |d| is_reject(d, Reject::StageExcluded),
                );
            }
        }
        // tether
        if let Some(th) = &row.tether {
            let hold = th.stages.is_empty() || th.stages.contains(&stage0);
            if hold && row.exclusive.is_none() {
                let st = State {
                    in_zone: PlayerMask::of(0),
                    ..base.clone()
                };
                let d = t.decide(&set(Origin::Host, stage0), &st);
                want(&mut f, "tether, stragglers outside", &d, |d| {
                    matches!(
                        d,
                        Decision::DeferToBarrier {
                            barrier: BarrierKind::Tether,
                            ..
                        }
                    )
                });
                let limit = th.timeout_s.unwrap_or(t.policy().tether_timeout_s);
                let st = State {
                    in_zone: PlayerMask::of(0),
                    tether_elapsed_s: limit,
                    ..base.clone()
                };
                let d = t.decide(&set(Origin::Host, stage0), &st);
                if th.gather == Gather::AutoWarp {
                    want(&mut f, "tether timeout, auto-warp", &d, |d| {
                        matches!(d, Decision::GatherThenApply { .. })
                    });
                } else {
                    want(&mut f, "tether timeout, wait", &d, |d| {
                        matches!(
                            d,
                            Decision::DeferToBarrier {
                                barrier: BarrierKind::Tether,
                                ..
                            }
                        )
                    });
                }
                let st = State {
                    in_zone: present,
                    ..base.clone()
                };
                let d = t.decide(&set(Origin::Host, stage0), &st);
                want(&mut f, "tether, all inside", &d, |d| {
                    !matches!(
                        d,
                        Decision::DeferToBarrier {
                            barrier: BarrierKind::Tether,
                            ..
                        }
                    ) && !matches!(d, Decision::GatherThenApply { .. })
                });
                let st = State {
                    in_zone: PlayerMask::of(0),
                    tether_mode: TetherMode::Off,
                    ..base.clone()
                };
                let d = t.decide(&set(Origin::Host, stage0), &st);
                want(&mut f, "tether mode off", &d, |d| {
                    !matches!(
                        d,
                        Decision::DeferToBarrier {
                            barrier: BarrierKind::Tether,
                            ..
                        }
                    )
                });
            }
        }
        // barrier on stages
        if let Some(b) = &row.barrier {
            if let Some(&bs) = b.stages.first() {
                let tether_in_way = row
                    .tether
                    .as_ref()
                    .is_some_and(|t| t.stages.is_empty() || t.stages.contains(&bs));
                if !tether_in_way && row.exclusive.is_none() {
                    let st = State {
                        barrier_passed: PlayerMask::of(0),
                        ..base.clone()
                    };
                    let d = t.decide(&set(Origin::Host, bs), &st);
                    want(&mut f, "barrier, one passed", &d, |d| {
                        matches!(
                            d,
                            Decision::DeferToBarrier {
                                barrier: BarrierKind::AllPlayersPass,
                                ..
                            }
                        )
                    });
                    let st = State {
                        barrier_passed: PlayerMask::of(0),
                        barrier_elapsed_s: t.policy().barrier_timeout_s,
                        ..base.clone()
                    };
                    let d = t.decide(&set(Origin::Host, bs), &st);
                    want(&mut f, "barrier timeout", &d, |d| {
                        is_reject(d, Reject::TimedOut)
                    });
                    let st = State {
                        barrier_passed: present,
                        ..base.clone()
                    };
                    let d = t.decide(&set(Origin::Host, bs), &st);
                    want(&mut f, "barrier, all passed", &d, |d| {
                        matches!(d, Decision::Apply)
                    });
                }
            }
        }
    }

    // ready check
    if let Some(r) = &row.ready {
        let k = EventKind::ReadyRequest { info: 0 };
        want(
            &mut f,
            "ready, nobody answered",
            &t.decide(&ev(Origin::Host, k.clone()), &base),
            |d| {
                matches!(
                    d,
                    Decision::DeferToBarrier {
                        barrier: BarrierKind::ReadyCheck,
                        ..
                    }
                )
            },
        );
        want(
            &mut f,
            "ready, all yes",
            &t.decide(
                &ev(Origin::Host, k.clone()),
                &State {
                    ready_yes: present,
                    ..base.clone()
                },
            ),
            |d| matches!(d, Decision::Apply),
        );
        want(
            &mut f,
            "ready, one no",
            &t.decide(
                &ev(Origin::Host, k.clone()),
                &State {
                    ready_yes: PlayerMask::of(0),
                    ready_no: PlayerMask::of(1),
                    ..base.clone()
                },
            ),
            |d| is_reject(d, Reject::Declined),
        );
        want(
            &mut f,
            "ready timeout",
            &t.decide(
                &ev(Origin::Host, k),
                &State {
                    ready_elapsed_s: r.timeout_s,
                    ..base.clone()
                },
            ),
            |d| is_reject(d, Reject::TimedOut),
        );
    }

    // consent
    if let Some(c) = &row.consent {
        let ch = |o: Origin, st: &State| t.decide(&ev(o, EventKind::Choice { choice: 0 }), st);
        match c.mode {
            ConsentMode::Unanimous => {
                want(
                    &mut f,
                    "unanimous, nobody voted",
                    &ch(Origin::Host, &base),
                    |d| {
                        matches!(
                            d,
                            Decision::DeferToBarrier {
                                barrier: BarrierKind::Consent,
                                ..
                            }
                        )
                    },
                );
                want(
                    &mut f,
                    "unanimous, all yes",
                    &ch(
                        Origin::Host,
                        &State {
                            votes_yes: present,
                            ..base.clone()
                        },
                    ),
                    |d| matches!(d, Decision::Apply),
                );
                want(
                    &mut f,
                    "unanimous, one no",
                    &ch(
                        Origin::Host,
                        &State {
                            votes_yes: PlayerMask::of(0),
                            votes_no: PlayerMask::of(1),
                            ..base.clone()
                        },
                    ),
                    |d| is_reject(d, Reject::Declined),
                );
            }
            ConsentMode::Vote => {
                want(
                    &mut f,
                    "vote, all yes",
                    &ch(
                        Origin::Host,
                        &State {
                            votes_yes: present,
                            ..base.clone()
                        },
                    ),
                    |d| matches!(d, Decision::Apply),
                );
                want(
                    &mut f,
                    "vote, all no",
                    &ch(
                        Origin::Host,
                        &State {
                            votes_no: present,
                            ..base.clone()
                        },
                    ),
                    |d| is_reject(d, Reject::Declined),
                );
                want(&mut f, "vote open", &ch(Origin::Host, &base), |d| {
                    matches!(
                        d,
                        Decision::DeferToBarrier {
                            barrier: BarrierKind::Consent,
                            ..
                        }
                    )
                });
            }
            ConsentMode::Host => {
                want(
                    &mut f,
                    "consent host, host",
                    &ch(Origin::Host, &base),
                    |d| matches!(d, Decision::Apply),
                );
                want(
                    &mut f,
                    "consent host, guest",
                    &ch(Origin::Player(1), &base),
                    |d| is_reject(d, Reject::NotHost),
                );
            }
            ConsentMode::Initiator => {
                want(
                    &mut f,
                    "consent initiator, initiator",
                    &ch(
                        Origin::Player(1),
                        &State {
                            initiator: Some(1),
                            ..base.clone()
                        },
                    ),
                    |d| matches!(d, Decision::Apply),
                );
                want(
                    &mut f,
                    "consent initiator, other",
                    &ch(
                        Origin::Player(1),
                        &State {
                            initiator: Some(0),
                            ..base.clone()
                        },
                    ),
                    |d| is_reject(d, Reject::NotInitiator),
                );
            }
        }
    }

    // rewards
    if !row.gate_exempt {
        let rw = |o: Origin, what: RewardKind, st: &State| {
            t.decide(&ev(o, EventKind::Reward { reward_id: 7, what }), st)
        };
        let st = State {
            initiator: Some(1),
            ..base.clone()
        };
        let d = rw(Origin::Host, RewardKind::Xp, &st);
        match row.reward {
            Reward::Everyone => want(
                &mut f,
                "reward everyone",
                &d,
                |d| matches!(d, Decision::Replicate { recipients, mode: FanOut::Full } if *recipients == present),
            ),
            Reward::Initiator => want(
                &mut f,
                "reward initiator",
                &d,
                |d| matches!(d, Decision::Replicate { recipients, mode: FanOut::Full } if *recipients == PlayerMask::of(1)),
            ),
            Reward::Split => want(
                &mut f,
                "reward split",
                &d,
                |d| matches!(d, Decision::Replicate { recipients, mode: FanOut::Split } if *recipients == present),
            ),
        }
        let st2 = State {
            initiator: Some(1),
            granted: (0..n).map(|s| (7u32, s)).collect(),
            ..base.clone()
        };
        want(
            &mut f,
            "reward again",
            &rw(Origin::Host, RewardKind::Xp, &st2),
            |d| is_reject(d, Reject::AlreadyGranted),
        );
        want(
            &mut f,
            "reward by a player",
            &rw(Origin::Player(1), RewardKind::Xp, &st),
            |d| is_reject(d, Reject::NotHost),
        );
        want(
            &mut f,
            "karma is shared",
            &rw(Origin::Host, RewardKind::Karma, &st),
            |d| matches!(d, Decision::Apply),
        );
        let d = rw(Origin::Host, RewardKind::QuestItem, &st);
        match row.quest_items {
            ItemRoute::All => want(
                &mut f,
                "quest items to all",
                &d,
                |d| matches!(d, Decision::Replicate { recipients, .. } if *recipients == present),
            ),
            ItemRoute::Initiator => want(
                &mut f,
                "quest items to initiator",
                &d,
                |d| matches!(d, Decision::Replicate { recipients, .. } if *recipients == PlayerMask::of(1)),
            ),
        }
        if let Some(p) = &row.perks {
            for (list, to_initiator) in
                [(&p.initiator, true), (&p.recruiter, true), (&p.all, false)]
            {
                if let Some(name) = list.first() {
                    let d = rw(Origin::Host, RewardKind::Perk(name.clone()), &st);
                    if to_initiator {
                        want(
                            &mut f,
                            "perk to initiator",
                            &d,
                            |d| matches!(d, Decision::Replicate { recipients, .. } if *recipients == PlayerMask::of(1)),
                        );
                    } else {
                        want(
                            &mut f,
                            "perk to all",
                            &d,
                            |d| matches!(d, Decision::Replicate { recipients, .. } if *recipients == present),
                        );
                    }
                }
            }
        }
    }

    // companions
    if let Some(c) = &row.companion {
        let co = |o, owner, critter, st: &State| {
            t.decide(
                &ev(
                    o,
                    EventKind::Companion {
                        op: CompanionOp::Recruit,
                        owner,
                        critter,
                    },
                ),
                st,
            )
        };
        match c.owner {
            CompanionOwner::Party => want(
                &mut f,
                "party companion",
                &co(Origin::Player(1), 0, false, &base),
                |d| matches!(d, Decision::Apply),
            ),
            CompanionOwner::Recruiter => {
                want(
                    &mut f,
                    "recruiter owns companion",
                    &co(Origin::Player(1), 1, false, &base),
                    |d| {
                        matches!(
                            d,
                            Decision::PerPlayerShadow {
                                slot: 1,
                                class: ShadowClass::Followers
                            }
                        )
                    },
                );
                want(
                    &mut f,
                    "recruit for someone else",
                    &co(Origin::Player(1), 0, false, &base),
                    |d| is_reject(d, Reject::NotOwner),
                );
                let mut full = base.clone();
                full.followers[1] = (t.policy().follower_cap.humanoid, 0);
                want(
                    &mut f,
                    "follower cap",
                    &co(Origin::Player(1), 1, false, &full),
                    |d| is_reject(d, Reject::FollowerCap),
                );
            }
        }
    }

    // shadow variables
    if let Some(sh) = &row.shadow {
        if let Some(v) = sh.vars.first().or(sh.objvars.first()) {
            let w = |o, slot| {
                t.decide(
                    &ev(
                        o,
                        EventKind::ShadowWrite {
                            class: ShadowClass::QuestVar,
                            slot,
                            var: Some(v.clone()),
                        },
                    ),
                    &base,
                )
            };
            want(&mut f, "shadow variable write", &w(Origin::Host, 1), |d| {
                matches!(
                    d,
                    Decision::PerPlayerShadow {
                        slot: 1,
                        class: ShadowClass::QuestVar
                    }
                )
            });
            want(
                &mut f,
                "shadow variable of another",
                &w(Origin::Player(2.min(n - 1)), 1),
                |d| {
                    if n > 2 {
                        is_reject(d, Reject::NotOwner)
                    } else {
                        true
                    }
                },
            );
        }
        if sh.down.is_some() {
            let d = t.decide(&ev(Origin::Host, EventKind::PlayerDown { slot: 1 }), &base);
            want(&mut f, "down until checkpoint", &d, |d| {
                matches!(d, Decision::PerPlayerShadow { slot: 1, .. })
            });
        }
    }

    // host-run scenes
    if let Some(h) = row.host_decision {
        if !row.gate_exempt
            && row.tether.is_none()
            && row.barrier.is_none()
            && row.exclusive.is_none()
        {
            let kind = match h {
                HostDecision::Ending => SceneKind::Ending,
                HostDecision::Time => SceneKind::TimeSkip,
                HostDecision::Cinematic => SceneKind::Cinematic,
            };
            let sc = |o| t.decide(&ev(o, EventKind::Scene { kind }), &base);
            want(
                &mut f,
                "scene by the host",
                &sc(Origin::Host),
                |d| matches!(d, Decision::Replicate { recipients, mode: FanOut::Full } if *recipients == present),
            );
            want(&mut f, "scene by a player", &sc(Origin::Player(1)), |d| {
                is_reject(d, Reject::NotHost)
            });
            if kind == SceneKind::TimeSkip && row.once.is_some() {
                let d = t.decide(
                    &ev(Origin::Host, EventKind::Scene { kind }),
                    &State {
                        event_done: true,
                        ..base.clone()
                    },
                );
                want(&mut f, "once-only time skip again", &d, |d| {
                    is_reject(d, Reject::AlreadyDone)
                });
            }
        }
    }

    // guest-local scripts
    let locals: Vec<u32> = row
        .guest_local
        .iter()
        .filter_map(|s| u32::from_str_radix(s.rsplit(':').next()?, 16).ok())
        .map(|v| v & 0x00FF_FFFF)
        .collect();
    if let Some(&l) = locals.first() {
        let gl = |script: u32| {
            t.decide(
                &ev(Origin::Player(1), EventKind::GuestScript { script }),
                &base,
            )
        };
        want(&mut f, "listed guest script", &gl(l | 0x0100_0000), |d| {
            matches!(d, Decision::Apply)
        });
        let other = (0..=0x00FF_FFFEu32)
            .find(|v| !locals.contains(v))
            .unwrap_or(0xABCDE);
        want(&mut f, "unlisted guest script", &gl(other), |d| {
            is_reject(d, Reject::GuestScriptSuppressed)
        });
    }

    // locks during scenes
    if row.fast_travel_lock {
        let st = State {
            active_scenes: vec![q],
            ..base.clone()
        };
        want(
            &mut f,
            "fast travel during scene",
            &t.decide(&ev(Origin::Host, EventKind::FastTravel), &st),
            |d| is_reject(d, Reject::FastTravelLocked),
        );
        want(
            &mut f,
            "fast travel after scene",
            &t.decide(&ev(Origin::Host, EventKind::FastTravel), &base),
            |d| matches!(d, Decision::Apply),
        );
    }
    if row.save_suppress {
        let st = State {
            active_scenes: vec![q],
            ..base.clone()
        };
        want(
            &mut f,
            "save during scene",
            &t.decide(&ev(Origin::Host, EventKind::Save), &st),
            |d| is_reject(d, Reject::SaveSuppressed),
        );
        want(
            &mut f,
            "save after scene",
            &t.decide(&ev(Origin::Host, EventKind::Save), &base),
            |d| matches!(d, Decision::Apply),
        );
    }
    f
}

// ---------------------------------------------------------------------------
// the rig: a host and guests over the chaos transport

struct Rig<'a> {
    d: &'a Data,
    sim: Sim,
    host: GameState,
    guests: Vec<GameState>,
    hb: HostBridge,
    gbs: Vec<GuestBridge>,
}

impl<'a> Rig<'a> {
    fn new(
        d: &'a Data,
        template: &GameState,
        table: &Arc<PolicyTable>,
        peers: u8,
        net: &str,
        seed: u64,
    ) -> Result<Rig<'a>, String> {
        let n = peers.saturating_sub(1).max(1);
        let profile = ChaosProfile::by_name(net).ok_or_else(|| format!("unknown profile {net}"))?;
        let mut host = template.clone();
        host.set_authority(Authority::Host);
        host.enable_coop_policy(table.clone(), 0);
        for s in 1..=n {
            host.register_player(s);
            host.with_player(s, |g| g.stock(&d.order, PLAYER_REF));
        }
        host.coop
            .gate
            .as_mut()
            .expect("gate")
            .set_present((0..=n).map(PlayerId));
        let plan = ShadowPlan::from_policy(table);
        let guests = (1..=n)
            .map(|s| {
                let mut g = template.clone();
                g.set_authority(Authority::Guest);
                g.enable_shadows(plan.clone(), s);
                g
            })
            .collect();
        let mut hb = HostBridge::default();
        hb.backstop_every = 0;
        let mut rig = Rig {
            d,
            sim: Sim::new(n, profile, seed),
            host,
            guests,
            hb,
            gbs: (0..n).map(|_| GuestBridge::default()).collect(),
        };
        for i in 0..n as usize {
            rig.sim.start_guest(i);
        }
        // collect the whole starting state once
        rig.host.coop.overflow = true;
        let ok = rig.settle(120_000);
        if !ok {
            let h: std::collections::BTreeSet<String> = rig
                .host
                .export_fields()
                .into_iter()
                .map(|(k, v)| format!("{k:?} {v:?}"))
                .collect();
            let g: std::collections::BTreeSet<String> = rig.guests[0]
                .export_fields()
                .into_iter()
                .map(|(k, v)| format!("{k:?} {v:?}"))
                .collect();
            let diff: Vec<String> = h
                .difference(&g)
                .map(|x| format!("host-only {x}"))
                .chain(g.difference(&h).map(|x| format!("guest-only {x}")))
                .take(6)
                .collect();
            let (hs, gs) = (
                format!("{:?}", rig.host.shadow_of(1)),
                format!("{:?}", rig.guests[0].shadow_of(1)),
            );
            let at = hs
                .bytes()
                .zip(gs.bytes())
                .position(|(a, b)| a != b)
                .unwrap_or(hs.len().min(gs.len()));
            let ctx = |s: &str| {
                s.get(at.saturating_sub(120)..(at + 200).min(s.len()))
                    .unwrap_or("")
                    .to_string()
            };
            let mut diff = diff;
            diff.push(format!(
                "shadow(1) lens {} vs {} differ at {at}: host `{}` guest `{}`",
                hs.len(),
                gs.len(),
                ctx(&hs),
                ctx(&gs)
            ));
            return Err(format!("guests did not join: {}", diff.join(" | ")));
        }
        Ok(rig)
    }

    fn tick(&mut self) {
        self.hb.collect(&mut self.host, &mut self.sim.host);
        self.sim.step();
        for i in 0..self.guests.len() {
            if self.sim.guests[i].sync.phase() == GuestPhase::Active {
                let opts = Opts {
                    present: true,
                    ctx: Some(Ctx {
                        order: &self.d.order,
                        scripts: &self.d.scripts,
                    }),
                };
                let store = &self.sim.guests[i].sync.store;
                self.gbs[i].sync(&mut self.guests[i], store, false, opts);
            }
        }
    }

    /// Steps until the stores agree and the shared shard hashes of every guest equal the host's.
    fn settle(&mut self, max_ms: u64) -> bool {
        let deadline = self.sim.clock.now() + max_ms;
        let mut quiet = 0;
        while self.sim.clock.now() < deadline {
            self.tick();
            if self.host.coop.out.is_empty() && self.sim.converged() {
                quiet += 1;
                if quiet >= 3 {
                    let h = self.host.shard_hashes();
                    if self.guests.iter().all(|g| g.shard_hashes() == h) {
                        return true;
                    }
                }
            } else {
                quiet = 0;
            }
        }
        false
    }
}

/// What one pass over the stages did.
#[derive(Default)]
struct Pass {
    applied: usize,
    released: usize,
    refused: usize,
    per_player: usize,
}

fn run_stage(
    rig: &mut Rig,
    row: &QuestRow,
    gid: FormId,
    stage: u16,
    slot: u8,
    acc: &mut Pass,
    fails: &mut Vec<String>,
) {
    let d = rig.d;
    // the intent check the host makes for a guest's request
    if slot != 0 {
        let dec = rig
            .host
            .coop
            .gate
            .as_ref()
            .expect("gate")
            .intent_set_stage(&rig.host, slot, gid, stage);
        if let Decision::Reject { reason } = dec {
            let ok = match reason {
                Reject::NotHost => row.advance == Advance::Host,
                Reject::NotInitiator => row.advance == Advance::Initiator,
                Reject::StageExcluded | Reject::GroupCommitted => true,
                _ => false,
            };
            if !ok {
                fails.push(format!(
                    "stage {stage}: guest intent refused unexpectedly: {reason:?}"
                ));
            }
            acc.refused += 1;
            // the host scripts still run it where the row is host-advanced
            if reason != Reject::NotHost {
                return;
            }
        }
    }
    let shared_before = rig.host.stages.get(&gid).copied();
    let ran = if slot == 0 || row.advance == Advance::Host {
        Runner::new(&d.order, &d.scripts, &mut rig.host).set_stage(gid, stage)
    } else {
        let mut g = rig.host.as_player(slot);
        Runner::new(&d.order, &d.scripts, &mut g).set_stage(gid, stage)
    };
    if ran {
        acc.applied += 1;
        if row.scope == Scope::PerPlayer && slot != 0 && row.advance != Advance::Host {
            acc.per_player += 1;
        }
        return;
    }
    // held? let the party do what the gate waits for, then release
    let held = rig
        .host
        .coop
        .gate
        .as_ref()
        .expect("gate")
        .pending()
        .iter()
        .any(|p| p.quest == gid && p.stage == stage);
    if held {
        let n = rig.guests.len() as u8;
        {
            let g = rig.host.coop.gate.as_mut().expect("gate");
            for s in 0..=n {
                g.barrier_pass(gid, s);
                g.set_in_zone(gid, s, true);
                g.ready_answer(gid, s, true);
            }
        }
        let mut gate = rig.host.coop.gate.take().expect("gate");
        let released = gate.poll(&rig.host);
        rig.host.coop.gate = Some(gate);
        for r in released {
            let owner = match r.decision {
                Decision::PerPlayerShadow { slot, .. } => slot,
                _ => 0,
            };
            let ok = if owner == 0 {
                Runner::new(&d.order, &d.scripts, &mut rig.host)
                    .set_stage(r.stage.quest, r.stage.stage)
            } else {
                let mut g = rig.host.as_player(owner);
                Runner::new(&d.order, &d.scripts, &mut g).set_stage(r.stage.quest, r.stage.stage)
            };
            if ok {
                acc.released += 1;
            } else {
                fails.push(format!(
                    "stage {} released by the gate but the runner refused it",
                    r.stage.stage
                ));
            }
        }
        if !rig
            .host
            .coop
            .gate
            .as_ref()
            .expect("gate")
            .pending()
            .iter()
            .any(|p| p.quest == gid)
        {
            return;
        }
        fails.push(format!(
            "stage {stage}: still held after the party arrived, passed and answered"
        ));
        return;
    }
    // refused or redirected: drain redirects (a per-player stage asked from the wrong rows)
    let reds = rig.host.coop.gate.as_mut().expect("gate").take_redirects();
    if !reds.is_empty() {
        for r in reds {
            let ok = {
                let mut g = rig.host.as_player(r.slot);
                Runner::new(&d.order, &d.scripts, &mut g).set_stage(r.quest, r.stage)
            };
            if ok {
                acc.applied += 1;
                acc.per_player += 1;
            }
        }
        return;
    }
    // refused by the gate (excluded stage, rival quest) or already done or not in the record
    let _ = shared_before;
    acc.refused += 1;
}

fn walk_run(
    d: &Data,
    template: &GameState,
    table: &Arc<PolicyTable>,
    row: &QuestRow,
    gid: FormId,
    net: &str,
    peers: u8,
    seed: u64,
    out: &mut QuestResult,
) {
    let tag = format!("[{net}, {peers} peers]");
    let mut rig = match Rig::new(d, template, table, peers, net, seed) {
        Ok(r) => r,
        Err(e) => {
            out.fails.push(format!("{tag} {e}"));
            return;
        }
    };
    let quest = d.scripts.quest(&d.order, gid);
    let mut stages: Vec<u16> = quest
        .as_ref()
        .map(|q| q.stages.iter().map(|s| s.index.max(0) as u16).collect())
        .unwrap_or_default();
    stages.sort_unstable();
    stages.dedup();
    out.stages = stages.len();
    let guests = rig.guests.len() as u8;
    let mut fails = Vec::new();
    let mut acc = Pass::default();

    // walk: guests advance the quests players may advance, in turn
    for (i, &st) in stages.iter().enumerate() {
        let slot = if row.advance == Advance::Host || guests == 0 {
            0
        } else {
            1 + (i as u8 % guests)
        };
        run_stage(&mut rig, row, gid, st, slot, &mut acc, &mut fails);
        if i % 8 == 7 {
            rig.tick();
        }
    }
    let sp_gaps: Vec<(String, String)> = rig
        .host
        .unhandled
        .iter()
        .map(|(k, _)| {
            (
                k.to_string(),
                rig.host.unhandled_first.get(k).cloned().unwrap_or_default(),
            )
        })
        .collect();
    for (k, v) in sp_gaps {
        out.sp_gaps.entry(k).or_insert(v);
    }

    // converge
    if !rig.settle(180_000) {
        let h = rig.host.shard_hashes();
        let bad: Vec<usize> = (0..rig.guests.len())
            .filter(|&i| rig.guests[i].shard_hashes() != h)
            .collect();
        let hf = rig.host.export_fields();
        let mut diff = Vec::new();
        if let Some(&g) = bad.first() {
            let gf = rig.guests[g].export_fields();
            for (k, f) in &hf {
                if !gf.iter().any(|(k2, f2)| k2 == k && f2 == f) {
                    diff.push(format!("{k:?}={f:?}"));
                }
            }
            for (k, _) in &gf {
                if !hf.iter().any(|(k2, _)| k2 == k) {
                    diff.push(format!("guest-only {k:?}"));
                }
            }
        }
        if let Some(&g) = bad.first() {
            for s in 0..=guests {
                let (a, b) = (
                    rig.host.shadow_of(s).map(|x| format!("{:#?}", x)),
                    rig.guests[g].shadow_of(s).map(|x| format!("{:#?}", x)),
                );
                if a != b {
                    let (a, b) = (a.unwrap_or_default(), b.unwrap_or_default());
                    let l: Vec<String> = a
                        .lines()
                        .zip(b.lines())
                        .filter(|(x, y)| x != y)
                        .take(2)
                        .map(|(x, y)| format!("{} vs {}", x.trim(), y.trim()))
                        .collect();
                    diff = vec![format!("player {s}'s rows differ: {}", l.join(" / "))];
                    break;
                }
            }
        }
        diff.truncate(3);
        fails.push(format!(
            "shared shard hashes differ after the walk (guests {bad:?}, stores equal: {}): {}",
            rig.sim.converged(),
            diff.join("; ")
        ));
    }

    // shared quests: every guest sees the host's stage; per-player quests: only the advancing player's shadow moved
    let max_stage = stages.last().copied();
    if row.scope == Scope::Shared {
        let hs = rig.host.stages.get(&gid).copied();
        for (i, g) in rig.guests.iter().enumerate() {
            if g.stages.get(&gid).copied() != hs {
                fails.push(format!(
                    "guest {} sees stage {:?}, host {:?}",
                    i + 1,
                    g.stages.get(&gid),
                    hs
                ));
            }
        }
    } else if let Some(m) = max_stage {
        let mut moved: Vec<u8> = Vec::new();
        for s in 0..=guests {
            if rig
                .host
                .shadow_of(s)
                .is_some_and(|sh| sh.quest_stage.contains_key(&gid))
            {
                moved.push(s);
            }
        }
        // guest slots that took a turn
        let turns: Vec<u8> = (0..stages.len())
            .map(|i| 1 + (i as u8 % guests.max(1)))
            .collect();
        for &s in &moved {
            if s != 0 && !turns.contains(&s) {
                fails.push(format!(
                    "player {s} has quest stage {:?} but never advanced it",
                    rig.host
                        .shadow_of(s)
                        .and_then(|x| x.quest_stage.get(&gid).copied())
                ));
            }
        }
        if acc.per_player > 0 && moved.is_empty() && rig.host.stages.get(&gid).is_none() {
            fails.push(format!(
                "per-player stages ran ({}) but no shadow holds the quest (max stage {m})",
                acc.per_player
            ));
        }
        // every guest holds the same copy of every player's quest rows
        for s in 0..=guests {
            let h = rig.host.shadow_of(s).map(|x| x.quest_stage);
            for (i, g) in rig.guests.iter().enumerate() {
                if g.shadow_of(s).map(|x| x.quest_stage) != h && s != (i as u8 + 1) {
                    fails.push(format!(
                        "guest {} holds a different copy of player {s}'s quest stages",
                        i + 1
                    ));
                }
            }
        }
    }

    // replay: nothing changes (rewards and results ran once)
    let repeats = quest
        .as_ref()
        .is_some_and(|q| q.flags & world::quest::ALLOW_REPEATED_STAGES != 0);
    if !repeats {
        let before = (
            rig.host.shard_hashes(),
            (0..=guests)
                .map(|s| {
                    rig.host
                        .shadow_of(s)
                        .map(|x| (x.items.clone(), x.level, x.quest_stage.clone()))
                })
                .collect::<Vec<_>>(),
        );
        let mut scratch = Pass::default();
        let mut rf = Vec::new();
        for (i, &st) in stages.iter().enumerate() {
            let slot = if row.advance == Advance::Host || guests == 0 {
                0
            } else {
                1 + (i as u8 % guests)
            };
            run_stage(&mut rig, row, gid, st, slot, &mut scratch, &mut rf);
        }
        let after = (
            rig.host.shard_hashes(),
            (0..=guests)
                .map(|s| {
                    rig.host
                        .shadow_of(s)
                        .map(|x| (x.items.clone(), x.level, x.quest_stage.clone()))
                })
                .collect::<Vec<_>>(),
        );
        if before != after {
            fails.push(
                "replaying every stage changed the state (a stage or its reward ran twice)".into(),
            );
        }
        if scratch.applied > 0 && scratch.applied > scratch.released {
            fails.push(format!(
                "replay applied {} stage sets again",
                scratch.applied
            ));
        }
    }

    // routing a reward through the gate gives each recipient exactly one
    if !row.gate_exempt {
        let before: Vec<Option<i32>> = (0..=guests)
            .map(|s| {
                rig.host
                    .shadow_of(s)
                    .map(|x| x.items.get(&world::items::CAPS).copied().unwrap_or(0))
            })
            .collect();
        let init = if guests == 0 { 0 } else { 1 };
        rig.host
            .coop
            .gate
            .as_mut()
            .expect("gate")
            .set_initiator(Some(init));
        let rid = world::coop::policy_gate::reward_id(gid, 0xFFFF, 0);
        let o1 = rig.host.coop_reward(
            &d.order,
            gid,
            rid,
            world::coop::policy_gate::Grant::Caps(100),
        );
        let o2 = rig.host.coop_reward(
            &d.order,
            gid,
            rid,
            world::coop::policy_gate::Grant::Caps(100),
        );
        rig.host
            .coop
            .gate
            .as_mut()
            .expect("gate")
            .set_initiator(None);
        let after: Vec<Option<i32>> = (0..=guests)
            .map(|s| {
                rig.host
                    .shadow_of(s)
                    .map(|x| x.items.get(&world::items::CAPS).copied().unwrap_or(0))
            })
            .collect();
        let got: Vec<i32> = before
            .iter()
            .zip(&after)
            .map(|(b, a)| a.unwrap_or(0) - b.unwrap_or(0))
            .collect();
        let total: i32 = got.iter().sum();
        match row.reward {
            Reward::Everyone => {
                if got.iter().any(|g| *g != 100) {
                    fails.push(format!("reward everyone: per-player caps change {got:?}"));
                }
            }
            Reward::Initiator => {
                let want: Vec<i32> = (0..=guests)
                    .map(|s| if s == init { 100 } else { 0 })
                    .collect();
                if got != want {
                    fails.push(format!("reward initiator: caps change {got:?}"));
                }
            }
            Reward::Split => {
                if total != 100 {
                    fails.push(format!("reward split: total {total} (not 100), {got:?}"));
                }
            }
        }
        if o2.is_some_and(|o| !o.applied.is_empty()) {
            fails.push("the same reward was granted twice".into());
        }
        let _ = o1;
    }

    // a guest never runs quest scripts
    if let Some(&st) = stages.first() {
        for (i, g) in rig.guests.iter().enumerate() {
            let mut c = g.clone();
            let before = (c.stages.clone(), c.coop.stats.suppressed_runs);
            let ran = Runner::new(&d.order, &d.scripts, &mut c).set_stage(gid, st);
            if ran || c.stages != before.0 {
                fails.push(format!("guest {} ran SetStage {st} itself", i + 1));
            } else if c.coop.stats.suppressed_runs <= before.1 {
                fails.push(format!(
                    "guest {}: the refused script run was not counted",
                    i + 1
                ));
            }
        }
    }

    out.applied = out.applied.max(acc.applied);
    out.released = out.released.max(acc.released);
    out.refused = out.refused.max(acc.refused);
    out.per_player = out.per_player.max(acc.per_player);
    out.fails
        .extend(fails.into_iter().map(|f| format!("{tag} {f}")));
}

/// Walks one quest row over every (profile, peer count) of the options.
pub fn walk_quest(
    d: &Data,
    template: &GameState,
    table: &Arc<PolicyTable>,
    row: &QuestRow,
    opts: &WalkOpts,
) -> QuestResult {
    let t0 = std::time::Instant::now();
    let mut out = QuestResult {
        edid: row.edid.clone(),
        plugin: row.plugin.clone(),
        line: row.line.clone(),
        dlc: dlc_of(row).to_string(),
        status: QStatus::Pass,
        depth: Depth::Full,
        stages: 0,
        applied: 0,
        released: 0,
        refused: 0,
        per_player: 0,
        runs: 0,
        fails: Vec::new(),
        sp_gaps: BTreeMap::new(),
        replay: format!(
            "coop-sim questwalk --only {} --net {} --peers {}",
            row.edid,
            opts.nets.join(","),
            opts.peers
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(",")
        ),
        ms: 0,
    };
    for p in &opts.peers {
        for f in policy_probe(table, row, *p) {
            out.fails.push(format!("[policy, {p} peers] {f}"));
        }
    }
    let gid = FormId(row.id);
    if row.synthetic || d.scripts.quest(&d.order, gid).is_none() {
        out.depth = Depth::PolicyOnly;
    } else {
        for net in &opts.nets {
            for &peers in &opts.peers {
                let seed = opts.seed ^ (u64::from(row.id) << 8) ^ u64::from(peers);
                let r = catch_unwind(AssertUnwindSafe(|| {
                    walk_run(d, template, table, row, gid, net, peers, seed, &mut out)
                }));
                out.runs += 1;
                if let Err(e) = r {
                    let msg = e
                        .downcast_ref::<String>()
                        .cloned()
                        .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
                        .unwrap_or_else(|| "panic".into());
                    out.fails
                        .push(format!("[{net}, {peers} peers] PANIC {msg}"));
                }
            }
        }
    }
    if !out.fails.is_empty() {
        out.status = QStatus::Fail;
    }
    out.ms = t0.elapsed().as_millis() as u64;
    out
}

/// Walks every row that matches the options, on `opts.jobs` threads.
pub fn walk_all(d: &Arc<Data>, opts: &WalkOpts) -> Result<(Vec<QuestResult>, Vec<String>), String> {
    let (table, missing) = remapped_table(d)?;
    let table = Arc::new(table);
    let template = GameState::new(&d.order);
    let rows: Vec<&QuestRow> = table
        .rows()
        .iter()
        .filter(|r| match &opts.filter {
            None => true,
            Some(f) => {
                let f = f.to_ascii_lowercase();
                r.edid.to_ascii_lowercase().contains(&f)
                    || r.plugin.to_ascii_lowercase().contains(&f)
                    || dlc_of(r).to_ascii_lowercase().contains(&f)
            }
        })
        .collect();
    let next = AtomicUsize::new(0);
    let results: Mutex<Vec<(usize, QuestResult)>> = Mutex::new(Vec::new());
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    std::thread::scope(|sc| {
        for _ in 0..opts.jobs.max(1).min(rows.len().max(1)) {
            sc.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                let Some(row) = rows.get(i) else { break };
                let r = walk_quest(d, &template, &table, row, opts);
                results.lock().expect("results").push((i, r));
            });
        }
    });
    std::panic::set_hook(prev);
    let mut v = results.into_inner().expect("results");
    v.sort_by_key(|x| x.0);
    Ok((v.into_iter().map(|x| x.1).collect(), missing))
}
