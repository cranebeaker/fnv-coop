//! Decisions added to close the campaign gaps (perk ids, friendly fire).

use coop_policy::*;

fn table() -> PolicyTable {
    PolicyTable::bundled().unwrap()
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

#[test]
fn perk_rewards_match_plugin_form_strings() {
    let t = table();
    let row = t
        .rows()
        .iter()
        .find(|r| {
            r.perks.as_ref().is_some_and(|p| {
                p.initiator
                    .iter()
                    .chain(&p.recruiter)
                    .any(|x| x.contains(':'))
            })
        })
        .expect("a row with an initiator perk");
    let p = row.perks.as_ref().unwrap();
    let listed = p
        .initiator
        .iter()
        .chain(&p.recruiter)
        .find(|x| x.contains(':'))
        .unwrap();
    let hex = listed.rsplit(':').next().unwrap();
    let mut s = party(&[0, 1, 2]);
    s.initiator = Some(1);
    // the world sends `EDID|hex`; either alias matches a row entry
    let kind = RewardKind::Perk(format!("NoSuchEdid|00{hex:0>6}"));
    let e = Event {
        quest: row.id,
        origin: Origin::Host,
        kind: EventKind::Reward {
            reward_id: 9,
            what: kind,
        },
    };
    assert_eq!(
        t.decide(&e, &s),
        Decision::Replicate {
            recipients: mask(&[1]),
            mode: FanOut::Full
        }
    );
}

#[test]
fn friendly_fire_guard_trips_after_the_allowed_hits() {
    let t = table();
    let row = t
        .rows()
        .iter()
        .find(|r| r.friendly_fire.is_some())
        .expect("ff row");
    let ff = row.friendly_fire.as_ref().unwrap();
    // rows name actors by editor id or `plugin:id`; the world sends both aliases
    let victim = match ff.actors[0].rsplit_once(':') {
        Some((_, hex)) => format!("NoSuchEdid|00{hex:0>6}"),
        None => format!("{}|00000000", ff.actors[0].to_ascii_lowercase()),
    };
    let s = party(&[0, 1]);
    let h = |recent| Event {
        quest: row.id,
        origin: Origin::Player(1),
        kind: EventKind::Hit {
            victim: victim.clone(),
            recent,
        },
    };
    assert_eq!(t.decide(&h(0), &s), Decision::Apply);
    assert_eq!(
        t.decide(&h(ff.deliberate_hits), &s),
        Decision::Reject {
            reason: Reject::FriendlyFire
        }
    );
    let other = Event {
        quest: row.id,
        origin: Origin::Player(1),
        kind: EventKind::Hit {
            victim: "nobody|00777777".into(),
            recent: 99,
        },
    };
    assert_eq!(t.decide(&other, &s), Decision::Apply);
    assert!(!t.guards_for(&victim).is_empty());
}

#[test]
fn perk_rewards_match_by_editor_id_too() {
    let t = table();
    let row = t.row_by_edid("VDialogueNovac").unwrap();
    let mut s = party(&[0, 1]);
    s.initiator = Some(1);
    let e = Event {
        quest: row.id,
        origin: Origin::Host,
        kind: EventKind::Reward {
            reward_id: 3,
            what: RewardKind::Perk("rangertakedown|00123456".into()),
        },
    };
    assert_eq!(
        t.decide(&e, &s),
        Decision::Replicate {
            recipients: mask(&[1]),
            mode: FanOut::Full
        }
    );
}
