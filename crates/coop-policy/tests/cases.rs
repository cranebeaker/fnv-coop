//! Decisions for representative cases taken from the real table.

use coop_policy::*;

fn table() -> PolicyTable {
    PolicyTable::bundled().unwrap()
}

fn id(t: &PolicyTable, edid: &str) -> FormId {
    t.row_by_edid(edid)
        .unwrap_or_else(|| panic!("no row {edid}"))
        .id
}

fn mask(slots: &[u8]) -> PlayerMask {
    slots
        .iter()
        .fold(PlayerMask::NONE, |m, s| m.or(PlayerMask::of(*s)))
}

fn party(slots: &[u8]) -> State {
    State {
        present: mask(slots),
        in_zone: mask(slots),
        ..State::default()
    }
}

fn ev(quest: FormId, origin: Origin, kind: EventKind) -> Event {
    Event {
        quest,
        origin,
        kind,
    }
}

fn stage(quest: FormId, origin: Origin, stage: u32) -> Event {
    ev(quest, origin, EventKind::SetStage { stage })
}

fn rej(r: Reject) -> Decision {
    Decision::Reject { reason: r }
}

// ---- main quests -------------------------------------------------------------------------

#[test]
fn main_quest_only_the_initiator_advances() {
    let t = table();
    let q = id(&t, "VMQ01");
    let mut s = party(&[0, 1, 2]);
    s.initiator = Some(1);
    assert_eq!(
        t.decide(&stage(q, Origin::Player(2), 40), &s),
        rej(Reject::NotInitiator)
    );
    assert_eq!(
        t.decide(&stage(q, Origin::Player(1), 40), &s),
        Decision::Apply
    );
    // host-run scripts always pass
    assert_eq!(t.decide(&stage(q, Origin::Host, 40), &s), Decision::Apply);
}

#[test]
fn main_quest_absent_player_cannot_act() {
    let t = table();
    let q = id(&t, "VMQ01");
    let s = party(&[0, 1]);
    assert_eq!(
        t.decide(&stage(q, Origin::Player(3), 10), &s),
        rej(Reject::NotPresent)
    );
}

#[test]
fn main_quest_ncr_barrier_on_one_way_stage() {
    let t = table();
    let q = id(&t, "VMQ05");
    let mut s = party(&[0, 1]);
    s.barrier_passed = mask(&[0]);
    assert_eq!(
        t.decide(&stage(q, Origin::Host, 100), &s),
        Decision::DeferToBarrier {
            barrier: BarrierKind::AllPlayersPass,
            waiting: mask(&[1]),
            timeout_s: 120
        }
    );
    s.barrier_passed = mask(&[0, 1]);
    assert_eq!(t.decide(&stage(q, Origin::Host, 100), &s), Decision::Apply);
    s.barrier_passed = mask(&[0]);
    s.barrier_elapsed_s = 120;
    assert_eq!(
        t.decide(&stage(q, Origin::Host, 100), &s),
        rej(Reject::TimedOut)
    );
}

#[test]
fn main_quest_tether_holds_then_warps_stragglers() {
    let t = table();
    let q = id(&t, "VMQ05"); // tether HooverDam, stages 60 and 100, auto-warp
    let mut s = party(&[0, 1]);
    s.in_zone = mask(&[0]);
    s.barrier_passed = mask(&[0, 1]);
    assert_eq!(
        t.decide(&stage(q, Origin::Host, 60), &s),
        Decision::DeferToBarrier {
            barrier: BarrierKind::Tether,
            waiting: mask(&[1]),
            timeout_s: 120
        }
    );
    s.tether_elapsed_s = 120;
    assert_eq!(
        t.decide(&stage(q, Origin::Host, 60), &s),
        Decision::GatherThenApply {
            stragglers: mask(&[1])
        }
    );
    // a stage the tether does not cover is free
    assert_eq!(t.decide(&stage(q, Origin::Host, 80), &s), Decision::Apply);
    // warn mode never holds anyone
    s.tether_mode = TetherMode::Warn;
    assert_eq!(t.decide(&stage(q, Origin::Host, 60), &s), Decision::Apply);
}

#[test]
fn main_quest_ready_check_needs_everyone() {
    let t = table();
    let q = id(&t, "VMQTops");
    let ask = ev(
        q,
        Origin::Player(0),
        EventKind::ReadyRequest { info: 0x160723 },
    );
    let mut s = party(&[0, 1]);
    s.ready_yes = mask(&[0]);
    assert_eq!(
        t.decide(&ask, &s),
        Decision::DeferToBarrier {
            barrier: BarrierKind::ReadyCheck,
            waiting: mask(&[1]),
            timeout_s: 120
        }
    );
    s.ready_no = mask(&[1]);
    assert_eq!(t.decide(&ask, &s), rej(Reject::Declined));
    s.ready_no = PlayerMask::NONE;
    s.ready_elapsed_s = 120;
    assert_eq!(t.decide(&ask, &s), rej(Reject::TimedOut));
    s.ready_yes = mask(&[0, 1]);
    assert_eq!(t.decide(&ask, &s), Decision::Apply);
}

#[test]
fn debug_quest_is_exempt_from_gating() {
    let t = table();
    let q = id(&t, "VMQ06");
    assert_eq!(
        t.decide(&stage(q, Origin::Player(2), 5), &party(&[0, 2])),
        Decision::Apply
    );
}

#[test]
fn unknown_quest_is_host_only() {
    let t = table();
    let s = party(&[0, 1]);
    assert_eq!(
        t.decide(&stage(0x00DE_AD01, Origin::Host, 5), &s),
        Decision::Apply
    );
    assert_eq!(
        t.decide(&stage(0x00DE_AD01, Origin::Player(1), 5), &s),
        rej(Reject::NotHost)
    );
}

// ---- Dead Money collars -----------------------------------------------------------------

#[test]
fn collar_variables_live_in_each_players_shadow() {
    let t = table();
    let q = id(&t, "NVDLC01BombCollarQuest");
    let s = party(&[0, 1]);
    let w = |origin, slot| {
        ev(
            q,
            origin,
            EventKind::ShadowWrite {
                class: ShadowClass::Collar,
                slot,
                var: Some("fCollarTimer".into()),
            },
        )
    };
    assert_eq!(
        t.decide(&w(Origin::Player(1), 1), &s),
        Decision::PerPlayerShadow {
            slot: 1,
            class: ShadowClass::QuestVar
        }
    );
    // nobody writes another player's collar
    assert_eq!(
        t.decide(&w(Origin::Player(0), 1), &s),
        rej(Reject::NotOwner)
    );
    // the host's own scripts may
    assert_eq!(
        t.decide(&w(Origin::Host, 1), &s),
        Decision::PerPlayerShadow {
            slot: 1,
            class: ShadowClass::QuestVar
        }
    );
}

#[test]
fn collar_explosion_downs_only_its_wearer() {
    let t = table();
    let q = id(&t, "NVDLC01BombCollarQuest");
    let s = party(&[0, 1, 2]);
    let d = t.decide(&ev(q, Origin::Host, EventKind::PlayerDown { slot: 2 }), &s);
    assert_eq!(
        d,
        Decision::PerPlayerShadow {
            slot: 2,
            class: ShadowClass::Collar
        }
    );
    // elsewhere a downed player is just the global death policy
    let other = id(&t, "VMQ01");
    assert_eq!(
        t.decide(
            &ev(other, Origin::Host, EventKind::PlayerDown { slot: 2 }),
            &s
        ),
        Decision::Apply
    );
}

#[test]
fn collar_quest_stage_is_per_player() {
    let t = table();
    let q = id(&t, "NVDLC01BombCollarQuest");
    let mut s = party(&[0, 1]);
    s.initiator = Some(1);
    assert_eq!(
        t.decide(&stage(q, Origin::Host, 10), &s),
        Decision::PerPlayerShadow {
            slot: 1,
            class: ShadowClass::QuestStage
        }
    );
    s.initiator = None;
    assert_eq!(
        t.decide(&stage(q, Origin::Host, 10), &s),
        Decision::Replicate {
            recipients: mask(&[0, 1]),
            mode: FanOut::Full
        }
    );
}

#[test]
fn collar_checkpoints_are_named() {
    let t = table();
    let r = t.row_by_edid("NVDLC01BombCollarQuest").unwrap();
    let sh = r.shadow.as_ref().unwrap();
    assert_eq!(sh.down, Some(DownMode::UntilCheckpoint));
    assert!(sh.checkpoint.iter().any(|c| c == "NVDLC01Suites"));
}

// ---- Caesar versus House ending ---------------------------------------------------------

#[test]
fn endings_first_committed_wins() {
    let t = table();
    let legion = id(&t, "VHDLegionBattle");
    let house = id(&t, "VHDHouseBattle");
    let mut s = party(&[0, 1]);
    assert!(matches!(
        t.decide(&stage(house, Origin::Host, 10), &s),
        Decision::Apply
    ));
    s.committed_quests = vec![house];
    assert_eq!(
        t.decide(&stage(legion, Origin::Host, 10), &s),
        rej(Reject::GroupCommitted)
    );
    let scene = ev(
        legion,
        Origin::Host,
        EventKind::Scene {
            kind: SceneKind::Ending,
        },
    );
    assert_eq!(t.decide(&scene, &s), rej(Reject::GroupCommitted));
}

#[test]
fn ending_is_a_host_decision_shown_to_everyone() {
    let t = table();
    let legion = id(&t, "VHDLegionBattle");
    let scene = |o| {
        ev(
            legion,
            o,
            EventKind::Scene {
                kind: SceneKind::Ending,
            },
        )
    };
    let mut s = party(&[0, 1, 2]);
    assert_eq!(
        t.decide(&scene(Origin::Player(1)), &s),
        rej(Reject::NotHost)
    );
    assert_eq!(
        t.decide(&scene(Origin::Host), &s),
        Decision::Replicate {
            recipients: mask(&[0, 1, 2]),
            mode: FanOut::Full
        }
    );
    // the whole party must be in the ending area
    s.in_zone = mask(&[0, 1]);
    assert_eq!(
        t.decide(&scene(Origin::Host), &s),
        Decision::DeferToBarrier {
            barrier: BarrierKind::Tether,
            waiting: mask(&[2]),
            timeout_s: 120
        }
    );
}

#[test]
fn ending_slideshow_waits_for_every_player_to_pass() {
    let t = table();
    let q = id(&t, "VEnding");
    let scene = ev(
        q,
        Origin::Host,
        EventKind::Scene {
            kind: SceneKind::Ending,
        },
    );
    let mut s = party(&[0, 1]);
    s.barrier_passed = mask(&[1]);
    assert_eq!(
        t.decide(&scene, &s),
        Decision::DeferToBarrier {
            barrier: BarrierKind::AllPlayersPass,
            waiting: mask(&[0]),
            timeout_s: 120
        }
    );
    s.barrier_passed = mask(&[0, 1]);
    assert_eq!(
        t.decide(&scene, &s),
        Decision::Replicate {
            recipients: mask(&[0, 1]),
            mode: FanOut::Full
        }
    );
}

#[test]
fn time_skip_runs_once_per_event() {
    let t = table();
    let q = id(&t, "NVDLC02MQ00");
    let skip = ev(
        q,
        Origin::Host,
        EventKind::Scene {
            kind: SceneKind::TimeSkip,
        },
    );
    let mut s = party(&[0, 1]);
    assert!(!matches!(t.decide(&skip, &s), Decision::Reject { .. }));
    s.event_done = true;
    assert_eq!(t.decide(&skip, &s), rej(Reject::AlreadyDone));
}

#[test]
fn irreversible_fork_goes_to_a_vote_with_host_tiebreak() {
    let t = table();
    let q = id(&t, "NVDLC04MQ03b");
    let pick = ev(q, Origin::Host, EventKind::Choice { choice: 1 });
    let mut s = party(&[0, 1, 2]);
    s.votes_yes = mask(&[1]);
    assert_eq!(
        t.decide(&pick, &s),
        Decision::DeferToBarrier {
            barrier: BarrierKind::Consent,
            waiting: mask(&[0, 2]),
            timeout_s: 90
        }
    );
    s.votes_yes = mask(&[0, 1]);
    s.votes_no = mask(&[2]);
    assert_eq!(t.decide(&pick, &s), Decision::Apply);
    let mut tie = party(&[0, 1]);
    tie.votes_yes = mask(&[0]);
    tie.votes_no = mask(&[1]);
    assert_eq!(t.decide(&pick, &tie), Decision::Apply);
    tie.votes_yes = mask(&[1]);
    tie.votes_no = mask(&[0]);
    assert_eq!(t.decide(&pick, &tie), rej(Reject::Declined));
}

#[test]
fn mutually_exclusive_stages_first_committed_wins() {
    let t = table();
    let q = id(&t, "nVPrimmDeputyConv");
    let mut s = party(&[0, 1]);
    s.committed_stages = vec![110];
    assert_eq!(
        t.decide(&stage(q, Origin::Host, 120), &s),
        rej(Reject::StageExcluded)
    );
    assert_eq!(t.decide(&stage(q, Origin::Host, 110), &s), Decision::Apply);
}

// ---- companions -------------------------------------------------------------------------

#[test]
fn companion_commands_are_owner_only() {
    let t = table();
    let q = id(&t, "vDialogueEDE");
    let cmd = |o, owner| {
        ev(
            q,
            o,
            EventKind::Companion {
                op: CompanionOp::Wait,
                owner,
                critter: false,
            },
        )
    };
    let s = party(&[0, 1]);
    assert_eq!(
        t.decide(&cmd(Origin::Player(1), 0), &s),
        rej(Reject::NotOwner)
    );
    assert_eq!(
        t.decide(&cmd(Origin::Player(0), 0), &s),
        Decision::PerPlayerShadow {
            slot: 0,
            class: ShadowClass::Followers
        }
    );
}

#[test]
fn companion_recruit_respects_caps() {
    let t = table();
    let q = id(&t, "vDialogueEDE");
    let hire = |critter| {
        ev(
            q,
            Origin::Player(1),
            EventKind::Companion {
                op: CompanionOp::Recruit,
                owner: 1,
                critter,
            },
        )
    };
    let mut s = party(&[0, 1]);
    assert_eq!(
        t.decide(&hire(false), &s),
        Decision::PerPlayerShadow {
            slot: 1,
            class: ShadowClass::Followers
        }
    );
    s.followers[1] = (1, 0);
    assert_eq!(t.decide(&hire(false), &s), rej(Reject::FollowerCap));
    // the critter slot is separate
    assert_eq!(
        t.decide(&hire(true), &s),
        Decision::PerPlayerShadow {
            slot: 1,
            class: ShadowClass::Followers
        }
    );
    // party total cap of 4
    s.followers = [
        (1, 1),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];
    s.followers[0] = (1, 1);
    s.followers[1] = (0, 0);
    let mut full = s.clone();
    full.followers[2] = (1, 1);
    full.present = mask(&[0, 1, 2]);
    full.followers[0] = (1, 1);
    assert_eq!(t.decide(&hire(false), &full), rej(Reject::PartyCap));
}

#[test]
fn plot_companion_belongs_to_the_party() {
    let t = table();
    let q = id(&t, "NVDLC04DialogueEDE");
    let s = party(&[0, 1, 2]);
    let cmd = ev(
        q,
        Origin::Player(2),
        EventKind::Companion {
            op: CompanionOp::Follow,
            owner: 0,
            critter: false,
        },
    );
    assert_eq!(t.decide(&cmd, &s), Decision::Apply);
}

// ---- rewards, perks, shadows ------------------------------------------------------------

#[test]
fn rewards_go_to_everyone_once() {
    let t = table();
    let q = id(&t, "VMQ01");
    let r = |what| ev(q, Origin::Host, EventKind::Reward { reward_id: 7, what });
    let mut s = party(&[0, 1, 2]);
    assert_eq!(
        t.decide(&r(RewardKind::Caps), &s),
        Decision::Replicate {
            recipients: mask(&[0, 1, 2]),
            mode: FanOut::Full
        }
    );
    s.granted = vec![(7, 0), (7, 1)];
    assert_eq!(
        t.decide(&r(RewardKind::Caps), &s),
        Decision::Replicate {
            recipients: mask(&[2]),
            mode: FanOut::Full
        }
    );
    s.granted.push((7, 2));
    assert_eq!(
        t.decide(&r(RewardKind::Xp), &s),
        rej(Reject::AlreadyGranted)
    );
    // a guest cannot hand itself a reward
    let own = ev(
        q,
        Origin::Player(1),
        EventKind::Reward {
            reward_id: 8,
            what: RewardKind::Caps,
        },
    );
    assert_eq!(t.decide(&own, &s), rej(Reject::NotHost));
}

#[test]
fn karma_is_one_value_and_payment_is_the_speakers() {
    let t = table();
    let q = id(&t, "VMQ01");
    let mut s = party(&[0, 1, 2]);
    s.initiator = Some(2);
    let r = |what| ev(q, Origin::Host, EventKind::Reward { reward_id: 1, what });
    assert_eq!(t.decide(&r(RewardKind::Karma), &s), Decision::Apply);
    assert_eq!(
        t.decide(&r(RewardKind::Payment), &s),
        Decision::Replicate {
            recipients: mask(&[2]),
            mode: FanOut::Full
        }
    );
    s.initiator = None;
    assert_eq!(
        t.decide(&r(RewardKind::Payment), &s),
        rej(Reject::NotInitiator)
    );
}

#[test]
fn recruit_perk_goes_to_the_recruiter_only() {
    let t = table();
    let q = id(&t, "vDialogueEDE");
    let mut s = party(&[0, 1, 2]);
    s.initiator = Some(1);
    let perk = |p: &str| {
        ev(
            q,
            Origin::Host,
            EventKind::Reward {
                reward_id: 3,
                what: RewardKind::Perk(p.into()),
            },
        )
    };
    assert_eq!(
        t.decide(&perk("FalloutNV.esm:15C571"), &s),
        Decision::Replicate {
            recipients: mask(&[1]),
            mode: FanOut::Full
        }
    );
    assert_eq!(
        t.decide(&perk("SomeQuestPerk"), &s),
        Decision::Replicate {
            recipients: mask(&[0, 1, 2]),
            mode: FanOut::Full
        }
    );
}

#[test]
fn shadow_classes_decide_shared_versus_private() {
    let t = table();
    let q = id(&t, "VMQ01");
    let s = party(&[0, 1]);
    let w = |class| {
        ev(
            q,
            Origin::Host,
            EventKind::ShadowWrite {
                class,
                slot: 1,
                var: None,
            },
        )
    };
    assert_eq!(t.decide(&w(ShadowClass::Karma), &s), Decision::Apply);
    assert_eq!(
        t.decide(&w(ShadowClass::Skills), &s),
        Decision::PerPlayerShadow {
            slot: 1,
            class: ShadowClass::Skills
        }
    );
    assert_eq!(
        t.decide(&w(ShadowClass::Organs), &s),
        Decision::PerPlayerShadow {
            slot: 1,
            class: ShadowClass::Organs
        }
    );
}

#[test]
fn per_player_quest_stages_stay_with_the_player() {
    let t = table();
    let q = id(&t, "AchievementQuest");
    let s = party(&[0, 1, 2]);
    assert_eq!(
        t.decide(&stage(q, Origin::Player(2), 10), &s),
        Decision::PerPlayerShadow {
            slot: 2,
            class: ShadowClass::QuestStage
        }
    );
}

// ---- locks and guests -------------------------------------------------------------------

#[test]
fn scenes_lock_fast_travel_and_saves() {
    let t = table();
    let legion = id(&t, "VHDLegionBattle");
    let other = id(&t, "VMQ01");
    let mut s = party(&[0, 1]);
    assert_eq!(
        t.decide(&ev(other, Origin::Player(1), EventKind::FastTravel), &s),
        Decision::Apply
    );
    s.active_scenes = vec![legion];
    assert_eq!(
        t.decide(&ev(other, Origin::Player(1), EventKind::FastTravel), &s),
        rej(Reject::FastTravelLocked)
    );
    assert_eq!(
        t.decide(&ev(other, Origin::Host, EventKind::Save), &s),
        rej(Reject::SaveSuppressed)
    );
}

#[test]
fn guests_run_only_allow_listed_scripts() {
    let t = table();
    let q = id(&t, "AchievementQuest");
    let s = party(&[0, 1]);
    let run = |script| ev(q, Origin::Player(1), EventKind::GuestScript { script });
    assert_eq!(t.decide(&run(0x0007_691E), &s), Decision::Apply);
    assert_eq!(
        t.decide(&run(0x0007_691F), &s),
        rej(Reject::GuestScriptSuppressed)
    );
}

#[test]
fn friendly_fire_guard_data_is_present() {
    let t = table();
    let r = t.row_by_edid("VMQ05").unwrap();
    let ff = r.friendly_fire.as_ref().unwrap();
    assert_eq!(ff.deliberate_hits, 3);
    assert!(ff.actors.iter().any(|a| a == "VHDColonelMooreREF"));
}
