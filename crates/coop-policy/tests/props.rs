//! Property tests: decisions are deterministic and total, and respect basic invariants.

use coop_policy::*;
use proptest::prelude::*;

fn table() -> &'static PolicyTable {
    use std::sync::OnceLock;
    static T: OnceLock<PolicyTable> = OnceLock::new();
    T.get_or_init(|| PolicyTable::bundled().unwrap())
}

fn is_reject(d: &Decision) -> bool {
    matches!(d, Decision::Reject { .. })
}

fn arb_mask() -> impl Strategy<Value = PlayerMask> {
    any::<u8>().prop_map(PlayerMask)
}

fn arb_origin() -> impl Strategy<Value = Origin> {
    prop_oneof![Just(Origin::Host), (0u8..10).prop_map(Origin::Player)]
}

fn arb_class() -> impl Strategy<Value = ShadowClass> {
    prop_oneof![
        Just(ShadowClass::Skills),
        Just(ShadowClass::Karma),
        Just(ShadowClass::Followers),
        Just(ShadowClass::Collar),
        Just(ShadowClass::Organs),
        Just(ShadowClass::World),
    ]
}

fn arb_kind() -> impl Strategy<Value = EventKind> {
    prop_oneof![
        (0u32..300).prop_map(|stage| EventKind::SetStage { stage }),
        (
            0u32..4,
            prop_oneof![
                Just(RewardKind::Caps),
                Just(RewardKind::Xp),
                Just(RewardKind::Item),
                Just(RewardKind::QuestItem),
                Just(RewardKind::Karma),
                Just(RewardKind::Payment),
                "[A-Za-z0-9:]{0,8}".prop_map(RewardKind::Perk),
            ]
        )
            .prop_map(|(reward_id, what)| EventKind::Reward { reward_id, what }),
        (arb_class(), 0u8..10, proptest::option::of("[a-zA-Z]{0,10}"))
            .prop_map(|(class, slot, var)| EventKind::ShadowWrite { class, slot, var }),
        (
            prop_oneof![
                Just(CompanionOp::Recruit),
                Just(CompanionOp::Dismiss),
                Just(CompanionOp::Wait),
                Just(CompanionOp::Follow),
                Just(CompanionOp::Trade)
            ],
            0u8..10,
            any::<bool>()
        )
            .prop_map(|(op, owner, critter)| EventKind::Companion { op, owner, critter }),
        prop_oneof![
            Just(SceneKind::Ending),
            Just(SceneKind::TimeSkip),
            Just(SceneKind::Cinematic)
        ]
        .prop_map(|kind| EventKind::Scene { kind }),
        any::<u32>().prop_map(|choice| EventKind::Choice { choice }),
        any::<u32>().prop_map(|info| EventKind::ReadyRequest { info }),
        (0u8..10).prop_map(|slot| EventKind::PlayerDown { slot }),
        Just(EventKind::FastTravel),
        Just(EventKind::Save),
        any::<u32>().prop_map(|script| EventKind::GuestScript { script }),
    ]
}

fn arb_state() -> impl Strategy<Value = State> {
    let a = (
        arb_mask(),
        proptest::option::of(0u8..10),
        arb_mask(),
        0u32..400,
        prop_oneof![
            Just(TetherMode::On),
            Just(TetherMode::Warn),
            Just(TetherMode::Off)
        ],
        arb_mask(),
        0u32..400,
    );
    let b = (
        arb_mask(),
        arb_mask(),
        0u32..400,
        arb_mask(),
        arb_mask(),
        0u32..400,
        proptest::collection::vec(0u32..300, 0..4),
    );
    let c = (
        proptest::collection::vec((0u32..4, 0u8..8), 0..6),
        any::<bool>(),
        proptest::collection::vec((0u8..3, 0u8..3), 8),
    );
    (a, b, c).prop_map(|(a, b, c)| {
        let mut followers = [(0u8, 0u8); 8];
        for (i, f) in c.2.into_iter().enumerate() {
            followers[i] = f;
        }
        State {
            present: a.0,
            initiator: a.1,
            in_zone: a.2,
            tether_elapsed_s: a.3,
            tether_mode: a.4,
            barrier_passed: a.5,
            barrier_elapsed_s: a.6,
            ready_yes: b.0,
            ready_no: b.1,
            ready_elapsed_s: b.2,
            votes_yes: b.3,
            votes_no: b.4,
            vote_elapsed_s: b.5,
            committed_stages: b.6,
            committed_quests: vec![],
            granted: c.0,
            active_scenes: vec![],
            followers,
            event_done: c.1,
        }
    })
}

fn arb_quest() -> impl Strategy<Value = FormId> {
    let n = table().rows().len();
    prop_oneof![
        9 => (0..n).prop_map(|i| table().rows()[i].id),
        1 => any::<u32>(),
    ]
}

fn arb_event() -> impl Strategy<Value = Event> {
    (arb_quest(), arb_origin(), arb_kind()).prop_map(|(quest, origin, kind)| Event {
        quest,
        origin,
        kind,
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(4000))]

    /// Same inputs, same answer, and an answer for every input (no panic).
    #[test]
    fn decisions_are_deterministic_and_total(e in arb_event(), s in arb_state()) {
        let t = table();
        let a = t.decide(&e, &s);
        let b = t.decide(&e.clone(), &s.clone());
        prop_assert_eq!(&a, &b);
        // a freshly loaded table agrees
        let t2 = PolicyTable::bundled().unwrap();
        prop_assert_eq!(&a, &t2.decide(&e, &s));
    }

    /// Whatever is returned stays inside the people who are actually present.
    #[test]
    fn recipients_and_waiters_are_present_players(e in arb_event(), s in arb_state()) {
        match table().decide(&e, &s) {
            Decision::Replicate { recipients, .. } => {
                prop_assert!(!recipients.is_empty());
                if !matches!(e.kind, EventKind::Companion { .. }) {
                    prop_assert!(recipients.is_subset_of(s.present));
                }
            }
            Decision::DeferToBarrier { waiting, timeout_s, .. } => {
                prop_assert!(!waiting.is_empty());
                prop_assert!(waiting.is_subset_of(s.present));
                prop_assert!(timeout_s > 0);
            }
            Decision::GatherThenApply { stragglers } => {
                prop_assert!(!stragglers.is_empty());
                prop_assert!(stragglers.is_subset_of(s.present));
            }
            _ => {}
        }
    }

    /// A player who is not counted never gets anything applied.
    #[test]
    fn absent_players_are_always_rejected(q in arb_quest(), slot in 0u8..10, kind in arb_kind(), s in arb_state()) {
        let mut s = s;
        s.present = PlayerMask(s.present.0 & !(if slot < 8 { 1u8 << slot } else { 0 }));
        if slot < 8 {
            let e = Event { quest: q, origin: Origin::Player(slot), kind };
            prop_assert_eq!(table().decide(&e, &s), Decision::Reject { reason: Reject::NotPresent });
        }
    }

    /// Players never cause host-only scenes or hand themselves rewards.
    #[test]
    fn players_cannot_issue_host_events(q in arb_quest(), slot in 0u8..8, s in arb_state()) {
        let mut s = s;
        s.present = s.present.or(PlayerMask::of(slot));
        let scene = Event { quest: q, origin: Origin::Player(slot), kind: EventKind::Scene { kind: SceneKind::Ending } };
        prop_assert_eq!(table().decide(&scene, &s), Decision::Reject { reason: Reject::NotHost });
        let reward = Event { quest: q, origin: Origin::Player(slot), kind: EventKind::Reward { reward_id: 1, what: RewardKind::Caps } };
        prop_assert_eq!(table().decide(&reward, &s), Decision::Reject { reason: Reject::NotHost });
    }

    /// Only the owner can change a private shadow slot from a player intent.
    #[test]
    fn shadow_writes_are_owner_only(q in arb_quest(), who in 0u8..8, slot in 0u8..8, class in arb_class(), s in arb_state()) {
        let mut s = s;
        s.present = s.present.or(PlayerMask::of(who));
        let e = Event { quest: q, origin: Origin::Player(who), kind: EventKind::ShadowWrite { class, slot, var: None } };
        let d = table().decide(&e, &s);
        if who != slot {
            prop_assert_eq!(d, Decision::Reject { reason: Reject::NotOwner });
        } else {
            prop_assert!(!is_reject(&d));
        }
    }

    /// Fully granted rewards are never paid twice.
    #[test]
    fn rewards_are_never_double_paid(q in arb_quest(), s in arb_state()) {
        let mut s = s;
        let mut granted = vec![];
        for slot in 0..8u8 { granted.push((9u32, slot)); }
        s.granted = granted;
        let e = Event { quest: q, origin: Origin::Host, kind: EventKind::Reward { reward_id: 9, what: RewardKind::Caps } };
        prop_assert!(is_reject(&table().decide(&e, &s)));
    }
}
