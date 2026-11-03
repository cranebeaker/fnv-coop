//! Live-layer scenarios, part two: loot races, revive timing, avatar stream,
//! and the wire format.

use super::live::{all_profiles, at, leased, loot_pending, Rig, G1, G2, H};
use super::*;
use crate::avatar::{quant_vel, AvatarMsg};
use crate::ids::{CellId, FormId, Seq};
use crate::loot::LootMsg;
use crate::rules::revive::{DOWN_WINDOW_MS, HOLD_MS};
use crate::rules::Notice;
use crate::rules::{ReviveEvent, RulesEvent, RulesMsg};
use crate::social::{ChatChannel, SocialMsg, VoicePacket};

const BOX: FormId = FormId(0xB0);
const CAPS: FormId = FormId(0xCA);

fn loot_rig(profile: ChaosProfile, seed: u64, caps: u32) -> Rig {
    let mut r = Rig::new(profile, seed);
    r.sim
        .host
        .live
        .loot
        .register(BOX, CellId(1), || vec![(CAPS, caps)]);
    r.g(0).live.local = at(1, 0.0);
    r.g(1).live.local = at(1, 10.0);
    r.run_ms(500);
    r
}

#[test]
fn loot_race_resolves_exactly_once_under_every_profile() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let name = profile.name;
        let mut r = loot_rig(profile, 500 + n as u64, 10);
        let open = r.g(0).live.loot.open(BOX);
        r.guest_send(0, Msg::Loot(open.clone()));
        r.guest_send(1, Msg::Loot(open));
        let settled = |r: &mut Rig| {
            (0..2).all(|i| leased(r, i, BOX) || r.g(i).live.loot.busy.contains_key(&BOX))
        };
        assert!(r.run_until(60_000, settled), "{name}");
        let winners: Vec<usize> = (0..2).filter(|i| leased(&mut r, *i, BOX)).collect();
        assert_eq!(winners.len(), 1, "{name}: one lease holder");
        let (w, l) = (winners[0], 1 - winners[0]);
        assert_eq!(
            r.sim.host.live.loot.holder(BOX),
            Some(PlayerId(w as u8 + 1)),
            "{name}"
        );

        // the winner takes 4; the same message is resent (reconnect style), still applied once
        let m = r.g(w).live.loot.take(BOX, CAPS, 4).unwrap();
        r.guest_send(w, Msg::Loot(m.clone()));
        r.guest_send(w, Msg::Loot(m.clone()));
        r.run_ms(150);
        r.guest_send(w, Msg::Loot(m));
        assert!(
            r.run_until(60_000, |r| loot_pending(r, w, BOX) == 0),
            "{name}"
        );
        r.run_ms(500);
        assert_eq!(
            r.sim.host.live.loot.items(BOX).unwrap()[&CAPS],
            6,
            "{name}: applied once"
        );
        let m = r.g(w).live.loot.close(BOX).unwrap();
        r.guest_send(w, Msg::Loot(m));

        // the loser retries until the lease frees up, then takes the rest
        let mut tries = 0;
        let got = r.run_until(120_000, |r| {
            if leased(r, l, BOX) {
                return true;
            }
            tries += 1;
            if tries % 40 == 1 {
                let m = r.g(l).live.loot.open(BOX);
                r.guest_send(l, Msg::Loot(m));
            }
            false
        });
        assert!(got, "{name}: loser never got the lease");
        assert_eq!(
            r.g(l).live.loot.view(BOX).unwrap().count(CAPS),
            6,
            "{name}: loser sees the winner's take"
        );
        let m = r.g(l).live.loot.take(BOX, CAPS, 6).unwrap();
        r.guest_send(l, Msg::Loot(m));
        assert!(
            r.run_until(60_000, |r| loot_pending(r, l, BOX) == 0),
            "{name}"
        );
        let m = r.g(l).live.loot.close(BOX).unwrap();
        r.guest_send(l, Msg::Loot(m));
        r.run_ms(1_000);

        assert_eq!(r.sim.host.live.loot.items(BOX).unwrap()[&CAPS], 0, "{name}");
        let (dw, dl) = (
            r.g(w).live.loot.confirmed_inventory_delta(CAPS),
            r.g(l).live.loot.confirmed_inventory_delta(CAPS),
        );
        assert_eq!((dw, dl), (4, 6), "{name}: every item moved exactly once");
        assert_eq!(
            r.g(w).live.loot.rollbacks + r.g(l).live.loot.rollbacks,
            0,
            "{name}"
        );
    }
}

#[test]
fn drop_claims_have_one_winner() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let name = profile.name;
        let mut r = loot_rig(profile, 600 + n as u64, 1);
        let d = FormId(0xD0);
        r.sim.host.live.loot.register_drop(d, CellId(1));
        r.guest_send(0, Msg::Loot(LootMsg::ClaimDrop { drop: d }));
        r.guest_send(1, Msg::Loot(LootMsg::ClaimDrop { drop: d }));
        let done = |r: &mut Rig| {
            (0..2)
                .all(|i| r.g(i).live.loot.drops_won.len() + r.g(i).live.loot.drops_lost.len() == 1)
        };
        assert!(r.run_until(60_000, done), "{name}");
        let won: usize = (0..2).map(|i| r.g(i).live.loot.drops_won.len()).sum();
        assert_eq!(won, 1, "{name}");
    }
}

#[test]
fn optimistic_take_rolls_back_when_the_host_changes_the_box() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let name = profile.name;
        let mut r = loot_rig(profile, 700 + n as u64, 10);
        let open = r.g(0).live.loot.open(BOX);
        r.guest_send(0, Msg::Loot(open));
        assert!(r.run_until(60_000, |r| leased(r, 0, BOX)), "{name}");
        let m = r.g(0).live.loot.take(BOX, CAPS, 8).unwrap();
        assert_eq!(
            r.g(0).live.loot.inventory_delta(CAPS),
            8,
            "{name}: shown at once"
        );
        // a script empties the box before the request reaches the host
        let outs = r.sim.host.live.loot.host_set(BOX, CAPS, 2);
        drop(outs); // the Changed broadcast is not needed for the verdict
        r.guest_send(0, Msg::Loot(m));
        assert!(
            r.run_until(60_000, |r| loot_pending(r, 0, BOX) == 0),
            "{name}"
        );
        assert_eq!(r.g(0).live.loot.rollbacks, 1, "{name}");
        assert_eq!(
            r.g(0).live.loot.inventory_delta(CAPS),
            0,
            "{name}: nothing leaked into the inventory"
        );
        assert_eq!(
            r.g(0).live.loot.view(BOX).unwrap().count(CAPS),
            2,
            "{name}: view adopts the host truth"
        );
    }
}

fn revive_rig(profile: ChaosProfile, seed: u64, helper_x: f32) -> Rig {
    let mut r = Rig::new(profile, seed);
    r.g(0).live.local = at(1, 0.0);
    r.g(1).live.local = at(1, helper_x);
    r.host_state = at(1, 50.0);
    assert!(r.run_until(60_000, |r| {
        r.sim.host.live.avatars.state(G1).is_some() && r.sim.host.live.avatars.state(G2).is_some()
    }));
    r
}

fn first_event(r: &Rig, f: impl Fn(&RulesEvent) -> bool) -> Option<u64> {
    r.rules_events.iter().find(|(_, e)| f(e)).map(|(t, _)| *t)
}

#[test]
fn revive_needs_a_three_second_hold_and_restores_thirty_percent() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let name = profile.name;
        let mut r = revive_rig(profile, 800 + n as u64, 100.0);
        let m = r.g(0).live.rules.report_down(true);
        r.guest_send(0, Msg::Rules(m));
        assert!(
            r.run_until(60_000, |r| r.g(1).live.rules.downed.contains(&G1)),
            "{name}"
        );
        let t0 = r.sim.clock.now();
        r.guest_send(1, Msg::Rules(RulesMsg::ReviveBegin { target: G1 }));
        let revived = |e: &RulesEvent| matches!(e, RulesEvent::Revive(ReviveEvent::Revived { .. }));
        assert!(
            r.run_until(60_000, |r| first_event(r, revived).is_some()),
            "{name}: never revived"
        );
        let t1 = first_event(&r, revived).unwrap();
        assert!(
            t1 >= t0 + HOLD_MS,
            "{name}: revived after {} ms, before the 3 s hold",
            t1 - t0
        );
        assert!(t1 <= t0 + HOLD_MS + 8_000, "{name}: took {} ms", t1 - t0);
        assert!(
            r.run_until(60_000, |r| r
                .g(0)
                .live
                .rules
                .notices
                .contains(&Notice::Revived(30))),
            "{name}"
        );
        assert!(
            r.run_until(60_000, |r| r.g(1).live.rules.downed.is_empty()),
            "{name}"
        );
    }
}

#[test]
fn revive_window_expires_into_a_respawn_and_far_helpers_are_refused() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let name = profile.name;
        let mut r = revive_rig(profile, 900 + n as u64, 900.0); // helper is out of the host's range
        let t_down = r.sim.clock.now();
        let m = r.g(0).live.rules.report_down(true);
        r.guest_send(0, Msg::Rules(m));
        assert!(
            r.run_until(60_000, |r| r.g(1).live.rules.downed.contains(&G1)),
            "{name}"
        );
        r.guest_send(1, Msg::Rules(RulesMsg::ReviveBegin { target: G1 }));
        r.run_ms(5_000);
        assert!(
            r.sim.host.live.rules.revive.helper_of(G1).is_none(),
            "{name}: out-of-range helper accepted"
        );
        let respawn =
            |e: &RulesEvent| matches!(e, RulesEvent::Revive(ReviveEvent::Respawn(p)) if *p == G1);
        assert!(
            r.run_until(120_000, |r| first_event(r, respawn).is_some()),
            "{name}"
        );
        let t = first_event(&r, respawn).unwrap();
        assert!(
            t >= t_down + DOWN_WINDOW_MS,
            "{name}: respawned early after {} ms",
            t - t_down
        );
        assert!(t <= t_down + DOWN_WINDOW_MS + 8_000, "{name}");
        assert!(
            r.run_until(60_000, |r| r
                .g(0)
                .live
                .rules
                .notices
                .contains(&Notice::Respawn)),
            "{name}"
        );
        assert!(
            first_event(&r, |e| matches!(
                e,
                RulesEvent::Revive(ReviveEvent::Revived { .. })
            ))
            .is_none(),
            "{name}"
        );
    }
}

#[test]
fn everyone_down_is_one_wipe() {
    let mut r = revive_rig(ChaosProfile::BAD_WIFI, 1000, 100.0);
    for i in 0..2 {
        let m = r.g(i).live.rules.report_down(true);
        r.guest_send(i, Msg::Rules(m));
    }
    r.sim
        .host_local(Msg::Rules(RulesMsg::ReportDown { revivable: true }));
    assert!(r.run_until(60_000, |r| (0..2).all(|i| r
        .g(i)
        .live
        .rules
        .notices
        .contains(&Notice::Wipe))));
    r.run_ms(2_000);
    let wipes = r
        .rules_events
        .iter()
        .filter(|(_, e)| matches!(e, RulesEvent::Revive(ReviveEvent::Wipe)))
        .count();
    assert_eq!(wipes, 1);
}

#[test]
fn avatar_stream_is_smooth_and_teleports_are_flagged() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let name = profile.name;
        let mut r = Rig::new(profile, 1100 + n as u64);
        r.sim.host.live.interest.link(CellId(1), CellId(2));
        r.g(1).live.local = at(1, 0.0);
        let speed = 100.0f32;
        let mut errs = Vec::new();
        let mut last = f32::MIN;
        let t_start = r.sim.clock.now();
        for _ in 0..400 {
            let t = (r.sim.clock.now() - t_start) as f32 / 1000.0;
            let mut s = at(1, speed * t);
            s.vel = [quant_vel(speed), 0, 0];
            r.g(0).live.local = s;
            r.step();
            let now = r.sim.clock.now();
            if t > 3.0 {
                if let Some(s) = r.g(1).live.sample(G1, now) {
                    assert!(
                        s.pos[0] + 0.5 >= last,
                        "{name}: avatar moved backwards {} -> {}",
                        last,
                        s.pos[0]
                    );
                    last = s.pos[0];
                    errs.push((s.pos[0] - speed * t).abs());
                }
            }
        }
        errs.sort_by(|a, b| a.total_cmp(b));
        assert!(!errs.is_empty(), "{name}");
        assert!(
            errs[errs.len() / 2] < 80.0,
            "{name}: median lag {} units",
            errs[errs.len() / 2]
        );

        // a cell change is a teleport: flagged once, no blending
        let mut s = at(2, 5000.0);
        s.vel = [0, 0, 0];
        r.g(0).live.local = s;
        let mut flagged = 0;
        for _ in 0..200 {
            r.step();
            let now = r.sim.clock.now();
            if let Some(smp) = r.g(1).live.sample(G1, now) {
                flagged += smp.teleported as u32;
            }
        }
        assert_eq!(flagged, 1, "{name}");
    }
}

#[test]
fn parts_reach_late_joiners_and_chat_flows() {
    let mut r = Rig::new(ChaosProfile::GOOD_WIFI, 1200);
    let parts = {
        let s = &mut r.g(0).live.sender;
        s.set_parts(vec![(1, FormId(0x77))], vec![9, 9])
    };
    r.guest_send(0, Msg::Avatar(parts));
    assert!(r.run_until(60_000, |r| r
        .g(1)
        .live
        .remote
        .get(&G1)
        .and_then(|a| a.parts.as_ref())
        .is_some()));
    let say = r.g(0).live.say(ChatChannel::All, "hello %s <you>");
    r.guest_send(0, say);
    assert!(r.run_until(60_000, |r| !r.g(1).live.chat_log.is_empty()));
    assert_eq!(r.g(1).live.chat_log[0].text, "hello ?s ?you?");
    r.run_ms(500);
    assert!(
        r.host_social
            .iter()
            .any(|m| matches!(m, Msg::Social(SocialMsg::Line { .. }))),
        "host player gets chat too"
    );
}

#[test]
fn live_messages_round_trip_and_respect_channels() {
    let msgs = vec![
        Msg::Avatar(AvatarMsg::State {
            player: H,
            seq: Seq(3),
            state: at(1, 2.0),
        }),
        Msg::Social(SocialMsg::Voice(VoicePacket {
            speaker: G1,
            seq: 9,
            payload: vec![5; crate::social::MAX_VOICE_PAYLOAD],
        })),
        Msg::Loot(LootMsg::Opened {
            container: BOX,
            rev: 2,
            items: (0..crate::loot::MAX_ITEMS as u32)
                .map(|i| (FormId(i), i))
                .collect(),
        }),
        Msg::Rules(RulesMsg::BarrierOpen {
            id: 1,
            tag: 2,
            required: vec![H, G1, G2],
        }),
    ];
    for m in msgs {
        let bytes = proto::encode(&m).expect("fits its channel");
        assert_eq!(proto::decode(m.channel(), &bytes).unwrap(), m);
    }
    let v = Msg::Social(SocialMsg::Voice(VoicePacket {
        speaker: G1,
        seq: 1,
        payload: vec![0; 1],
    }));
    assert_eq!(v.channel(), proto::Channel::Unreliable);
    assert_eq!(
        Msg::Loot(LootMsg::Open { container: BOX }).channel(),
        proto::Channel::ReliableOrdered
    );
    let big = Msg::Social(SocialMsg::Voice(VoicePacket {
        speaker: G1,
        seq: 1,
        payload: vec![0; crate::social::MAX_VOICE_PAYLOAD + 1],
    }));
    assert!(!big.is_sane());
    assert!(proto::PROTOCOL_VERSION >= 2);
}
