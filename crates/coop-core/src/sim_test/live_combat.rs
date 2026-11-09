//! Combat depth on the chaos simulator: NPC damage to a remote player,
//! shot and blast relay, VATS verdicts. Every link under each profile.

use super::live::*;
use super::*;
use crate::combat::{BlastEvent, CombatMsg, DamageKind, FireEvent, VatsShot};
use crate::ids::CellId;

const NPC: FormId = FormId(0x500);

fn rig(profile: ChaosProfile, seed: u64) -> Rig {
    let mut r = Rig::new(profile, seed);
    r.host_state = at(1, 0.0);
    r.g(0).live.local = at(1, 50.0);
    r.g(1).live.local = at(1, 90.0);
    for p in [H, G1, G2] {
        r.sim.host.live.combat.vitals.track(p, 100, 100);
    }
    r.run_ms(1_500);
    r
}

fn fire(shooter: PlayerId) -> FireEvent {
    FireEvent {
        shooter,
        id: 1,
        weapon: FormId(0x700),
        cell: CellId(1),
        origin: [0; 3],
        hit: [16 * 300, 0, 0],
        target: NPC,
        limb: 0,
        surface: 2,
    }
}

#[test]
fn guest_npc_hits_a_remote_player_under_every_profile() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let name = profile.name;
        let mut r = rig(profile, 1200 + n as u64);
        // G1's machine owns an NPC that shoots G2 and the host player
        let m1 = r
            .g(0)
            .live
            .combat
            .npc_hit(NPC, G2, 20, DamageKind::Bullet, 1);
        let m2 = r
            .g(0)
            .live
            .combat
            .npc_hit(NPC, H, 20, DamageKind::Bullet, 1);
        r.guest_send(0, Msg::Combat(m1));
        r.guest_send(0, Msg::Combat(m2));
        assert!(
            r.run_until(60_000, |r| r.g(1).live.combat.hurts_seen == 1),
            "{name}: victim never told"
        );
        let h = r.g(1).live.combat.hurts[0];
        assert_eq!(h.attacker, NPC);
        assert!(h.amount >= 20, "{name}: party scaling only raises it");
        assert_eq!(
            r.sim.host.live.combat.vitals.get(G2).unwrap().hp,
            100 - h.amount,
            "{name}"
        );
        assert!(
            r.sim.host.live.combat.vitals.get(H).unwrap().hp < 100,
            "{name}: host victim"
        );
        assert_eq!(
            r.g(0).live.combat.hurts_seen,
            0,
            "{name}: only the victim is told"
        );
    }
}

#[test]
fn shots_and_blasts_reach_the_other_peers_but_not_the_shooter() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let name = profile.name;
        let mut r = rig(profile, 1300 + n as u64);
        let f = r.g(0).live.combat.fire(fire(G1));
        r.guest_send(0, Msg::Combat(f));
        let b = r.g(0).live.combat.blast(BlastEvent {
            source: G1,
            id: 0,
            explosion: FormId(0x800),
            origin: FormId(0x801),
            cell: CellId(1),
            pos: [16 * 300, 0, 0],
            damage: 80,
            radius: 250,
            kind: DamageKind::Explosion,
        });
        r.guest_send(0, Msg::Combat(b));
        assert!(
            r.run_until(60_000, |r| r.g(1).live.combat.fires.len() == 1
                && r.g(1).live.combat.blasts.len() == 1),
            "{name}"
        );
        assert_eq!(r.g(1).live.combat.fires[0].shooter, G1, "{name}");
        assert!(
            r.g(0).live.combat.fires.is_empty() && r.g(0).live.combat.blasts.is_empty(),
            "{name}: no echo"
        );
        assert_eq!(
            r.sim.host.live.combat.blasts.len(),
            1,
            "{name}: the host world resolves the area"
        );
    }
}

#[test]
fn vats_queue_resolves_on_the_host_and_hits_the_actor_ledger() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let name = profile.name;
        let mut r = rig(profile, 1400 + n as u64);
        r.sim.host.live.actors.register(
            NPC,
            crate::actors::ActorState {
                cell: CellId(1),
                health: 400,
                ..Default::default()
            },
        );
        r.sim.host.live.combat.vats.known_ap.insert(G1, 60);
        let shot = VatsShot {
            target: NPC,
            limb: 0,
            cost: 20,
            chance: 1000,
            damage: 30,
        };
        let (id, m) = r.g(0).live.combat.vats(90, vec![shot; 4]);
        r.guest_send(0, Msg::Combat(m));
        assert!(
            r.run_until(60_000, |r| r.g(0).live.combat.vats_done.contains_key(&id)),
            "{name}"
        );
        let (res, spent) = r.g(0).live.combat.vats_done[&id].clone();
        assert_eq!(spent, 60, "{name}: the host charged the AP it knew about");
        assert_eq!(res.iter().filter(|x| x.fired).count(), 3, "{name}");
        let hits = res.iter().filter(|x| x.hit).count() as u32;
        assert_eq!(
            r.sim.host.live.actors.hits_applied, hits,
            "{name}: each hit went through the one ledger"
        );
        // nobody else's clock moved: they only hear a presentation mark
        r.guest_send(
            0,
            Msg::Combat(CombatMsg::VatsMark {
                player: G1,
                active: true,
            }),
        );
        assert!(
            r.run_until(60_000, |r| r.g(1).live.combat.vats_active.contains(&G1)),
            "{name}"
        );
    }
}

#[test]
fn a_flood_of_requests_is_bounded() {
    let mut r = rig(ChaosProfile::PERFECT, 1500);
    for _ in 0..200 {
        let m = r
            .g(0)
            .live
            .combat
            .npc_hit(NPC, G2, 1, DamageKind::Bullet, 1);
        r.guest_send(0, Msg::Combat(m));
    }
    r.run_ms(5_000);
    assert!(
        r.g(1).live.combat.hurts_seen <= 45,
        "{}",
        r.g(1).live.combat.hurts_seen
    );
}
