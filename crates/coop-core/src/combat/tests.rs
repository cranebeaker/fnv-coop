use super::*;
use crate::ids::{CellId, FormId, PlayerId};
use crate::interest::Audience;
use crate::rules::{Mode, Scaling};

const H: PlayerId = PlayerId(0);
const A: PlayerId = PlayerId(1);
const B: PlayerId = PlayerId(2);
const NPC: FormId = FormId(0x500);

fn party(n: u32) -> Scaling {
    Scaling::for_party(n, Mode::CoopBalanced)
}

fn host() -> CombatHost {
    let mut h = CombatHost::new();
    for p in [H, A, B] {
        h.vitals.track(p, 100, 100);
    }
    h
}

fn npc_hit(id: u32, victim: PlayerId, amount: u16) -> CombatMsg {
    CombatMsg::NpcHit {
        id,
        attacker: NPC,
        victim,
        amount,
        kind: DamageKind::Bullet,
        limb: 1,
    }
}

#[test]
fn npc_damage_reaches_the_victims_machine_scaled() {
    let mut h = host();
    let out = h.on_msg(H, npc_hit(1, A, 20), 1000, None, None, &party(3));
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].aud, Audience::To(A));
    let CombatMsg::PlayerHurt {
        amount,
        health,
        victim,
        ..
    } = out[0].msg
    else {
        panic!()
    };
    assert_eq!(victim, A);
    assert_eq!(amount, 24, "+20 percent for three players");
    assert_eq!(health, 76);
    assert_eq!(h.vitals.get(A).unwrap().hp, 76);
}

#[test]
fn duplicate_and_flooded_requests_are_refused() {
    let mut h = host();
    assert_eq!(
        h.on_msg(H, npc_hit(1, A, 5), 0, None, None, &party(2))
            .len(),
        1
    );
    assert!(h
        .on_msg(H, npc_hit(1, A, 5), 1, None, None, &party(2))
        .is_empty());
    let mut ok = 0;
    for i in 2..200 {
        ok += h
            .on_msg(H, npc_hit(i, A, 1), 10, None, None, &party(2))
            .len();
    }
    assert!(ok <= MAX_REQ_PER_SEC, "{ok}");
    assert!(h.vitals.stats.refused > 100);
}

#[test]
fn downed_players_take_no_more_and_revive_restores() {
    let mut h = host();
    h.vitals.report(A, 0, 0, true, false);
    assert!(h
        .on_msg(H, npc_hit(1, A, 50), 5000, None, None, &party(2))
        .is_empty());
    assert_eq!(h.vitals.stats.while_downed, 1);
    let out = CombatHost::deliver(h.vitals.revived(6000, A, 30));
    assert!(!h.vitals.get(A).unwrap().downed);
    assert_eq!(h.vitals.get(A).unwrap().hp, 30);
    assert!(out.iter().any(|o| matches!(
        o.msg,
        CombatMsg::PlayerEffect {
            effect: EffectKind::Cure,
            ..
        }
    )));
    assert_eq!(
        h.on_msg(H, npc_hit(2, A, 5), 7000, None, None, &party(2))
            .len(),
        1
    );
}

#[test]
fn friendly_fire_rule() {
    let mut v = VitalsHost::new();
    v.track(A, 100, 100);
    v.track(B, 100, 100);
    assert!(v
        .player_vs_player(0, A, B, 30, DamageKind::Bullet, 1)
        .is_empty());
    assert_eq!(v.stats.friendly_blocked, 1);
    v.friendly = FriendlyFire::Scaled(0.5);
    let m = v.player_vs_player(10, A, B, 30, DamageKind::Bullet, 1);
    assert!(matches!(m[0], CombatMsg::PlayerHurt { amount: 15, .. }));
    v.friendly = FriendlyFire::On;
    let m = v.player_vs_player(20, A, B, 30, DamageKind::Bullet, 1);
    assert!(matches!(m[0], CombatMsg::PlayerHurt { amount: 30, .. }));
    // own grenade always counts, even with the rule off
    v.friendly = FriendlyFire::Off;
    let m = v.player_vs_player(30, A, A, 10, DamageKind::Explosion, 7);
    assert!(matches!(m[0], CombatMsg::PlayerHurt { amount: 10, .. }));
}

#[test]
fn limbs_cripple_and_cure_clears() {
    let mut v = VitalsHost::new();
    v.track(A, 500, 500);
    let n = Scaling::NEUTRAL;
    let mut effects = 0;
    for t in 0..4 {
        let m = v.hurt(t * 10, A, NPC, 20, DamageKind::Bullet, 4, &n);
        effects += m
            .iter()
            .filter(|m| {
                matches!(
                    m,
                    CombatMsg::PlayerEffect {
                        effect: EffectKind::Cripple,
                        limb: 4,
                        ..
                    }
                )
            })
            .count();
    }
    assert_eq!(effects, 1, "crippled once");
    assert_eq!(v.get(A).unwrap().crippled, 1 << 4);
    v.effect(100, A, EffectKind::Cure, 0, 0, 0);
    assert_eq!(v.get(A).unwrap().crippled, 0);
}

#[test]
fn poison_bleed_and_rads_per_player() {
    let mut v = VitalsHost::new();
    v.track(A, 100, 100);
    v.track(B, 100, 100);
    let n = Scaling::NEUTRAL;
    v.effect(0, A, EffectKind::Poison, 3, 0, 3000);
    v.effect(0, A, EffectKind::Bleed, 0, 2, 2000);
    v.effect(0, A, EffectKind::Radiation, 40, 0, 0);
    let mut total = 0u32;
    for s in 1..=5u64 {
        for m in v.tick(s * 1000, &n) {
            if let CombatMsg::PlayerHurt { victim, amount, .. } = m {
                assert_eq!(victim, A);
                total += u32::from(amount);
            }
        }
    }
    assert_eq!(total, 3 * 3 + 2, "poison for 3 s plus bleeding for 2 s");
    assert_eq!(v.get(A).unwrap().rads, 40);
    assert_eq!(v.get(B).unwrap().rads, 0);
    assert_eq!(v.get(B).unwrap().hp, 100);
}

#[test]
fn health_report_does_not_undo_a_fresh_hit() {
    let mut v = VitalsHost::new();
    v.track(A, 100, 100);
    v.hurt(5000, A, NPC, 40, DamageKind::Bullet, 1, &Scaling::NEUTRAL);
    v.report(A, 5200, 100, false, false);
    assert_eq!(
        v.get(A).unwrap().hp,
        60,
        "the hurt message has not landed yet"
    );
    v.report(A, 7000, 55, false, false);
    assert_eq!(v.get(A).unwrap().hp, 55);
}

fn input(p: PlayerId, dist: f32) -> ThreatInput {
    ThreatInput {
        player: p,
        dist,
        seen: true,
        downed: false,
        dead: false,
        same_cell: true,
    }
}

#[test]
fn threat_prefers_near_and_sticks() {
    let mut d = ThreatDirector::new();
    let f = party(2).focus;
    let t = d
        .choose(NPC, 0, f, &[input(A, 500.0), input(B, 1500.0)])
        .unwrap();
    assert_eq!(t.target, A);
    // B steps a little closer: no flicker
    let t = d
        .choose(NPC, 1000, f, &[input(A, 700.0), input(B, 600.0)])
        .unwrap();
    assert_eq!(t.target, A);
    assert!(!t.switched);
    // B hurts it and is much closer, after the hold time
    d.damaged(NPC, B, 400.0);
    let t = d
        .choose(NPC, 5000, f, &[input(A, 2500.0), input(B, 300.0)])
        .unwrap();
    assert_eq!(t.target, B);
    assert!(t.switched);
}

#[test]
fn threat_takes_over_a_target_the_game_set() {
    let mut d = ThreatDirector::new();
    let f = party(2).focus;
    assert!(
        !d.adopt(NPC, B, 0),
        "no pick yet: the first choice stays free"
    );
    assert_eq!(d.current(NPC), None);
    let near = [input(A, 500.0), input(B, 1500.0)];
    assert_eq!(d.choose(NPC, 0, f, &near).unwrap().target, A);
    // a script turns the enemy on B: the director holds B from now on
    assert!(d.adopt(NPC, B, 1000));
    assert!(!d.adopt(NPC, B, 1000), "already B");
    assert_eq!(d.current(NPC), Some(B));
    let t = d.choose(NPC, 2000, f, &near).unwrap();
    assert_eq!((t.target, t.switched), (B, false), "inside the hold time");
    // A hurts it hard after the hold time: the director turns it back to A
    d.damaged(NPC, A, 400.0);
    let t = d.choose(NPC, 5000, f, &near).unwrap();
    assert_eq!((t.target, t.switched), (A, true));
}

#[test]
fn threat_skips_downed_and_spreads() {
    let mut d = ThreatDirector::new();
    let f = party(2).focus;
    let mut a = input(A, 400.0);
    a.downed = true;
    assert_eq!(
        d.choose(NPC, 0, f, &[a, input(B, 2000.0)]).unwrap().target,
        B
    );
    a.dead = true;
    let mut b = input(B, 1.0);
    b.downed = true;
    assert!(
        d.choose(FormId(0x501), 0, f, &[a, b]).is_none(),
        "everyone down: no target"
    );
    // five enemies equally placed do not all pick the same player
    let mut d = ThreatDirector::new();
    let picks: Vec<PlayerId> = (0..6)
        .map(|i| {
            d.choose(
                FormId(0x600 + i),
                0,
                f,
                &[input(A, 1000.0), input(B, 1000.0)],
            )
            .unwrap()
            .target
        })
        .collect();
    assert!(picks.contains(&A) && picks.contains(&B), "{picks:?}");
}

#[test]
fn threat_other_cell_scores_nothing() {
    let mut d = ThreatDirector::new();
    let mut a = input(A, 10.0);
    a.same_cell = false;
    assert_eq!(
        d.choose(NPC, 0, 1.0, &[a, input(B, 2900.0)])
            .unwrap()
            .target,
        B
    );
}

#[test]
fn extras_are_deterministic_capped_and_cleared() {
    let s = party(4);
    let mut p = ExtrasPlanner::new();
    let mut plans = Vec::new();
    for i in 0..400u32 {
        let e = FormId(0xFF00_0000 + i);
        if let Some(plan) = p.consider(&s, e, [0.0, 0.0, 0.0], false) {
            assert_eq!(plan.site, extra_site(e));
            plans.push(plan);
        }
    }
    // the same ids decide the same way on another machine
    let mut q = ExtrasPlanner::new();
    let again: Vec<FormId> = (0..400u32)
        .filter_map(|i| {
            q.consider(&s, FormId(0xFF00_0000 + i), [0.0; 3], false)
                .map(|x| x.origin)
        })
        .collect();
    assert_eq!(again, plans.iter().map(|x| x.origin).collect::<Vec<_>>());
    assert!(!plans.is_empty());
    assert!(
        p.alive() as u32 <= s.max_extras_alive,
        "{} <= {}",
        p.alive(),
        s.max_extras_alive
    );
    // solo: never
    let mut solo = ExtrasPlanner::new();
    assert!((0..400u32).all(|i| solo
        .consider(&Scaling::NEUTRAL, FormId(i), [0.0; 3], false)
        .is_none()));
    // clear when far from everyone, keep when near
    let plan = plans[0];
    p.created_as(&plan, FormId(0xFF10_0000), CellId(1));
    let near = p.to_clear(&[(A, [100.0, 0.0, 0.0])], |_| None);
    assert!(near.is_empty());
    let far = p.to_clear(&[(A, [100.0 + CLEAR_DISTANCE + 10.0, 0.0, 0.0])], |_| None);
    assert_eq!(far, vec![FormId(0xFF10_0000)]);
    // an extra never breeds
    assert!(p
        .consider(&s, FormId(0xFFFF_0001), [0.0; 3], true)
        .is_none());
}

fn fire_event(id: u32) -> FireEvent {
    FireEvent {
        shooter: PlayerId(77),
        id,
        weapon: FormId(0x700),
        cell: CellId(1),
        origin: [0, 0, 0],
        hit: [16 * 1000, 0, 0],
        target: NPC,
        limb: 0,
        surface: 3,
    }
}

#[test]
fn fire_is_relayed_to_the_others_with_the_real_shooter() {
    let mut h = host();
    let out = h.on_msg(
        A,
        CombatMsg::Fire(fire_event(1)),
        0,
        Some([0.0; 3]),
        Some(CellId(1)),
        &party(2),
    );
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].aud, Audience::CellExcept(CellId(1), A));
    let CombatMsg::Fire(e) = out[0].msg else {
        panic!()
    };
    assert_eq!(e.shooter, A, "a guest cannot name someone else");
    assert!(h
        .on_msg(
            A,
            CombatMsg::Fire(fire_event(1)),
            1,
            Some([0.0; 3]),
            None,
            &party(2)
        )
        .is_empty());
    let mut far = fire_event(2);
    far.hit = [16 * 90_000, 0, 0];
    assert!(h
        .on_msg(A, CombatMsg::Fire(far), 2, Some([0.0; 3]), None, &party(2))
        .is_empty());
    assert_eq!(h.fire.fires_ok, 1);
    assert_eq!(h.fires.len(), 1);
}

fn blast(id: u32, x: i32) -> BlastEvent {
    BlastEvent {
        source: PlayerId(99),
        id,
        explosion: FormId(0x800),
        origin: FormId(0x801),
        cell: CellId(1),
        pos: [x, 0, 0],
        damage: 90,
        radius: 300,
        kind: DamageKind::Explosion,
    }
}

#[test]
fn blasts_are_accepted_once_near_the_thrower_and_queued_for_the_world() {
    let mut h = host();
    let out = h.on_msg(
        A,
        CombatMsg::Blast(blast(1, 16 * 200)),
        0,
        Some([0.0; 3]),
        Some(CellId(1)),
        &party(2),
    );
    assert_eq!(out.len(), 1);
    assert_eq!(h.blasts.len(), 1);
    assert_eq!(h.blasts[0].source, A);
    assert!(h
        .on_msg(
            A,
            CombatMsg::Blast(blast(1, 16 * 200)),
            1,
            Some([0.0; 3]),
            None,
            &party(2)
        )
        .is_empty());
    // a blast on the other side of the map is not the thrower's
    assert!(h
        .on_msg(
            A,
            CombatMsg::Blast(blast(2, 16 * 50_000)),
            2,
            Some([0.0; 3]),
            None,
            &party(2)
        )
        .is_empty());
    assert_eq!(h.blasts.len(), 1);
}

#[test]
fn vats_never_freezes_the_shared_world() {
    let solo = vats_scales(false, 0.0);
    assert_eq!(solo.world, 0.0, "single player still pauses to plan");
    for wanted in [0.0, 0.1, 0.5] {
        let c = vats_scales(true, wanted);
        assert_eq!(c.world, 1.0);
        assert_eq!(c.presentation, wanted);
    }
}

fn shot(cost: u16, chance: u16, damage: u16) -> VatsShot {
    VatsShot {
        target: NPC,
        limb: 0,
        cost,
        chance,
        damage,
    }
}

#[test]
fn vats_pays_from_the_shooters_ap_and_is_idempotent() {
    let mut v = VatsHost::new();
    v.known_ap.insert(A, 40);
    let s = party(3);
    let shots = [shot(20, 1000, 50), shot(20, 1000, 50), shot(20, 1000, 50)];
    let (r, spent) = v.resolve(A, 1, 100, &shots, &s);
    assert_eq!(spent, 40, "the host knows A has 40, not 100");
    assert!(r[0].fired && r[1].fired && !r[2].fired);
    assert!(r[0].hit || r[1].hit || v.limits.max_chance < 1000);
    let again = v.resolve(A, 1, 100, &shots, &s);
    assert_eq!((r.clone(), spent), again);
    assert_eq!(v.queues, 1);
    // toughness applies
    for x in r.iter().filter(|x| x.hit && !x.critical) {
        assert_eq!(x.damage, 25, "50 / toughness 2.0");
    }
}

#[test]
fn vats_chance_is_capped_and_rolls_are_stable() {
    let mut v = VatsHost::new();
    let s = Scaling::NEUTRAL;
    let mut hits = 0;
    for id in 1..=300u32 {
        let (r, _) = v.resolve(A, id, 100, &[shot(1, 1000, 10)], &s);
        hits += usize::from(r[0].hit);
    }
    assert!(hits < 300 && hits > 250, "{hits}: capped at 95 percent");
    let mut w = VatsHost::new();
    let (r1, _) = w.resolve(A, 7, 100, &[shot(1, 500, 10)], &s);
    let mut x = VatsHost::new();
    let (r2, _) = x.resolve(A, 7, 100, &[shot(1, 500, 10)], &s);
    assert_eq!(r1, r2);
}

#[test]
fn vats_queue_message_produces_actor_hits_and_a_verdict() {
    let mut h = host();
    h.vats.known_ap.insert(A, 100);
    let msg = CombatMsg::VatsQueue {
        id: 5,
        ap: 100,
        shots: vec![shot(10, 1000, 30); 4],
    };
    let out = h.on_msg(A, msg, 0, None, None, &party(2));
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].aud, Audience::To(A));
    let CombatMsg::VatsDone {
        spent, ref results, ..
    } = out[0].msg
    else {
        panic!()
    };
    assert_eq!(spent, 40);
    assert_eq!(h.vats_hits.len(), results.iter().filter(|r| r.hit).count());
    let ids: std::collections::BTreeSet<u32> = h.vats_hits.iter().map(|x| x.hit).collect();
    assert_eq!(ids.len(), h.vats_hits.len(), "unique ledger ids");
}

#[test]
fn vats_marks_are_presentation_only_and_relayed() {
    let mut h = host();
    let out = h.on_msg(
        A,
        CombatMsg::VatsMark {
            player: B,
            active: true,
        },
        0,
        None,
        Some(CellId(1)),
        &party(2),
    );
    let CombatMsg::VatsMark { player, active } = out[0].msg else {
        panic!()
    };
    assert_eq!((player, active), (A, true), "the sender is the one in VATS");
    assert!(h.vats.active().any(|p| *p == A));
}

#[test]
fn client_applies_each_hurt_once_and_only_its_own() {
    let mut c = CombatClient::new(A);
    let hurt = |victim, seq| CombatMsg::PlayerHurt {
        victim,
        seq,
        amount: 9,
        kind: DamageKind::Bullet,
        limb: 1,
        attacker: NPC,
        health: 50,
    };
    c.on_msg(hurt(A, 1));
    c.on_msg(hurt(A, 1));
    c.on_msg(hurt(B, 2));
    assert_eq!(c.hurts.len(), 1);
    let mut mine = fire_event(1);
    mine.shooter = A;
    c.on_msg(CombatMsg::Fire(mine));
    assert!(c.fires.is_empty(), "own shots are not replayed");
    mine.shooter = B;
    c.on_msg(CombatMsg::Fire(mine));
    assert_eq!(c.fires.len(), 1);
}

#[test]
fn insane_messages_are_rejected() {
    let mut h = host();
    assert!(h
        .on_msg(A, npc_hit(1, B, 0), 0, None, None, &party(2))
        .is_empty());
    assert!(h
        .on_msg(
            A,
            npc_hit(2, B, MAX_PLAYER_HIT + 1),
            0,
            None,
            None,
            &party(2)
        )
        .is_empty());
    let big = CombatMsg::VatsQueue {
        id: 1,
        ap: 1,
        shots: vec![shot(1, 1, 1); MAX_VATS_SHOTS + 1],
    };
    assert!(!big.is_sane());
}

#[test]
fn messages_round_trip_on_the_wire() {
    let m = crate::proto::Msg::Combat(npc_hit(3, A, 12));
    let bytes = postcard::to_stdvec(&m).unwrap();
    let back: crate::proto::Msg = postcard::from_bytes(&bytes).unwrap();
    assert_eq!(back, m);
    assert_eq!(m.channel(), crate::proto::Channel::ReliableOrdered);
}
