// HitValidator: every rule of the victim owner's judge, each with a case that a mutation twin (FNVCOOP_MUT=301..322) breaks.
#include "fake_world.h"

using namespace combat_test;

namespace {
HitVerdict judge(HitValidator& v, const HitEvent& h, PlayerId sender, PlayerId self, const FakeWorld& w, uint64_t now = kNow) {
    return v.judge(h, sender, self, now, w);
}
}  // namespace

TEST(hits, valid_hit_applies) {
    FakeWorld w = standardWorld();
    HitValidator v;
    HitVerdict r = judge(v, goodHit(1), 1, 0, w);
    CHECK(r.apply);
    CHECK(r.reason == HitReason::Ok);
    CHECK(r.damage == 20);
    CHECK(v.judged() == 1 && v.rejected() == 0);
}

TEST(hits, only_the_victims_owner_resolves_a_hit) {   // twin 302
    FakeWorld w = standardWorld();
    HitValidator atP1;   // machine 1 does not own actor 100 (p0 does)
    HitVerdict r = judge(atP1, goodHit(1), 1, 1, w);
    CHECK(!r.apply);
    CHECK(r.reason == HitReason::NotVictimOwner);
    CHECK(r.forwardTo == 0);
    // ... and the owner takes the very same hit
    HitValidator atP0;
    CHECK(judge(atP0, goodHit(1), 1, 0, w).apply);
}

TEST(hits, a_hit_resolves_once) {   // twin 301
    FakeWorld w = standardWorld();
    HitValidator v;
    CHECK(judge(v, goodHit(7), 1, 0, w).apply);
    HitVerdict again = judge(v, goodHit(7), 1, 0, w);
    CHECK(!again.apply);
    CHECK(again.reason == HitReason::Duplicate);
    CHECK(v.seen(7));
    // a different id is a different hit
    CHECK(judge(v, goodHit(8), 1, 0, w).apply);
    // after reset (new session) the same id is new again
    v.reset();
    CHECK(judge(v, goodHit(7), 1, 0, w).apply);
}

TEST(hits, idempotency_memory_is_bounded) {
    FakeWorld w = standardWorld();
    HitValidator v(defaultRules(), 4);
    CombatRules r = defaultRules();
    r.maxHitsPerSec = 1000;
    v.setRules(r);
    w.weapons.clear();
    for (uint64_t i = 1; i <= 10; ++i) {
        HitEvent h = goodHit(i);
        h.weapon = FormRef();
        h.claimedRange = 3000;
        CHECK(judge(v, h, 1, 0, w).apply);
    }
    CHECK(v.seen(10));
    CHECK(!v.seen(1));   // fell out of the window
}

TEST(hits, dead_victim_takes_no_hits) {   // twin 303
    FakeWorld w = standardWorld();
    w.f[Entity::actor(100)].alive = false;
    HitValidator v;
    HitVerdict r = judge(v, goodHit(1), 1, 0, w);
    CHECK(!r.apply);
    CHECK(r.reason == HitReason::VictimDead);
}

TEST(hits, damage_bounds) {   // twin 304
    FakeWorld w = standardWorld();
    HitValidator v;
    uint64_t id = 100;
    for (int32_t d : {0, -5, 81, 5000}) {   // rifle max is 80
        HitEvent h = goodHit(++id);
        h.damage = d;
        HitVerdict r = judge(v, h, 1, 0, w);
        CHECK_MSG(!r.apply && r.reason == HitReason::DamageOutOfBounds, "damage " + std::to_string(d));
    }
    HitEvent top = goodHit(++id);
    top.damage = 80;
    CHECK(judge(v, top, 1, 0, w).apply);
    // unknown weapon: the global bound (500)
    HitEvent unk = goodHit(++id);
    unk.weapon = FormRef();
    unk.claimedRange = 2000;
    unk.damage = 501;
    CHECK(judge(v, unk, 1, 0, w).reason == HitReason::DamageOutOfBounds);
}

TEST(hits, claimed_range_is_bounded_when_the_weapon_is_unknown) {   // twin 305
    FakeWorld w = standardWorld();
    w.f[Entity::player(1)].pos = {20000, 0, 0};
    w.f[Entity::actor(100)].pos = {20500, 0, 0};
    HitValidator v;
    HitEvent h = goodHit(1);
    h.weapon = FormRef();
    h.impactPoint = {20500, 0, 0};
    h.claimedRange = 100000;   // "my gun reaches the moon"
    CHECK(judge(v, h, 1, 0, w).reason == HitReason::WeaponRangeOutOfBounds);
    h.hitId = 2; h.claimedRange = 0;
    CHECK(judge(v, h, 1, 0, w).reason == HitReason::WeaponRangeOutOfBounds);
    h.hitId = 3; h.claimedRange = 1000;
    CHECK(judge(v, h, 1, 0, w).apply);   // 500 units away, claims 1000: fine
}

TEST(hits, stale_and_future_hits_are_dropped) {   // twins 306, 315
    FakeWorld w = standardWorld();
    HitValidator v;
    HitEvent old = goodHit(1);
    old.firedAtTick = static_cast<uint32_t>(kNow - 5000);
    CHECK(judge(v, old, 1, 0, w).reason == HitReason::TooOld);
    HitEvent future = goodHit(2);
    future.firedAtTick = static_cast<uint32_t>(kNow + 2000);
    CHECK(judge(v, future, 1, 0, w).reason == HitReason::FromFuture);
    HitEvent slightlyAhead = goodHit(3);
    slightlyAhead.firedAtTick = static_cast<uint32_t>(kNow + 300);   // inside the 500 ms slack
    CHECK(judge(v, slightlyAhead, 1, 0, w).apply);
    // the 32-bit stamp wraps: a hit stamped just before the wrap, judged just after it, is recent
    HitValidator v2;
    HitEvent wrap = goodHit(4);
    wrap.firedAtTick = 0xFFFFFF00u;
    CHECK(v2.judge(wrap, 1, 0, 0x100000000ull + 100, w).apply);
    // an unstamped hit is allowed unless the rules demand a stamp
    HitEvent bare = goodHit(5);
    bare.firedAtTick = 0;
    CHECK(judge(v, bare, 1, 0, w).apply);
    CombatRules strict = defaultRules();
    strict.requireTick = true;
    HitValidator v3(strict);
    bare.hitId = 6;
    CHECK(!judge(v3, bare, 1, 0, w).apply);
}

TEST(hits, a_player_attacker_must_be_the_sender) {   // twin 307
    FakeWorld w = standardWorld();
    HitValidator v;
    HitVerdict r = judge(v, goodHit(1), 0, 0, w);   // p0 sends a hit that claims to come from p1
    CHECK(!r.apply);
    CHECK(r.reason == HitReason::AttackerNotSender);
    HitEvent bad = goodHit(2);
    bad.attacker = Entity::player(200);   // not even a slot
    CHECK(!judge(v, bad, 200, 0, w).apply);
}

TEST(hits, an_npc_attacker_must_belong_to_the_sender) {   // twin 308
    FakeWorld w = standardWorld();
    w.f[Entity::actor(101)].pos = {100, 0, 0};   // raider 101 (owned by p1, epoch 5) next to player 0
    HitEvent h;
    h.hitId = 1; h.attacker = Entity::actor(101); h.victim = Entity::player(0); h.damage = 8; h.claimedRange = 150;
    h.impactPoint = {0, 0, 0}; h.firedAtTick = 10000; h.attackerEpoch = 5;
    HitValidator v;
    HitVerdict forged = judge(v, h, 0, 0, w);   // p0 itself claims p1's raider hit p0
    CHECK(!forged.apply);
    CHECK(forged.reason == HitReason::ForgedAttacker);
    h.hitId = 2;
    HitVerdict real = judge(v, h, 1, 0, w);   // the raider's owner reports it
    CHECK(real.apply);
}

TEST(hits, attacker_epochs) {   // twins 309, 318
    FakeWorld w = standardWorld();
    w.f[Entity::actor(101)].pos = {100, 0, 0};
    HitEvent h;
    h.attacker = Entity::actor(101); h.victim = Entity::player(0); h.damage = 8; h.claimedRange = 150;
    h.impactPoint = {0, 0, 0}; h.firedAtTick = 10000;
    HitValidator v;
    h.hitId = 1; h.attackerEpoch = 4;   // reported under an older owner: stale
    CHECK(judge(v, h, 1, 0, w).reason == HitReason::StaleAttackerEpoch);
    h.hitId = 2; h.attackerEpoch = 6;   // the reporter knows a newer epoch than our table: do not apply, ask for a resync
    HitVerdict ahead = judge(v, h, 1, 0, w);
    CHECK(!ahead.apply);
    CHECK(ahead.reason == HitReason::FutureAttackerEpoch);
    CHECK(ahead.resync);
    h.hitId = 3; h.attackerEpoch = 5;
    CHECK(judge(v, h, 1, 0, w).apply);
}

TEST(hits, cells) {   // twin 310
    FakeWorld w = standardWorld();
    HitValidator v;
    w.f[Entity::player(1)].cell = 9;
    CHECK(judge(v, goodHit(1), 1, 0, w).reason == HitReason::OtherCell);
    w.f[Entity::player(1)].cell = 2;   // adjacent
    CHECK(judge(v, goodHit(2), 1, 0, w).apply);
}

TEST(hits, weapon_range_and_the_table_wins_over_the_claim) {   // twins 311, 319
    FakeWorld w = standardWorld();
    w.f[Entity::player(1)].pos = {10000, 0, 0};   // 9000 units from actor 100, rifle reaches 3000
    HitValidator v;
    HitEvent h = goodHit(1);
    h.claimedRange = 20000;   // the attacker claims a long range; the weapon table says 3000
    HitVerdict r = judge(v, h, 1, 0, w);
    CHECK(!r.apply);
    CHECK(r.reason == HitReason::OutOfRange);
    // a shotgun (800) at 2000 units
    w.f[Entity::player(1)].pos = {3000, 0, 0};
    HitEvent s = goodHit(2);
    s.weapon = shotgun();
    s.claimedRange = 6000;
    CHECK(judge(v, s, 1, 0, w).reason == HitReason::OutOfRange);
    // slack: just past the nominal range is tolerated (positions are from different frames), a replica attacker gets a little more
    w.f[Entity::player(1)].pos = {1000 + 800 + 650, 0, 0};
    s.hitId = 3;
    CHECK(judge(v, s, 1, 0, w).apply);
}

TEST(hits, impact_point_must_be_near_the_victim) {   // twin 312
    FakeWorld w = standardWorld();
    HitValidator v;
    HitEvent h = goodHit(1);
    h.impactPoint = {5000, 0, 0};
    CHECK(judge(v, h, 1, 0, w).reason == HitReason::ImpactNotNearVictim);
    h.hitId = 2; h.impactPoint = {1100, 50, 0};
    CHECK(judge(v, h, 1, 0, w).apply);
    // a big creature has a radius that widens the check
    w.f[Entity::actor(100)].radius = 600;
    h.hitId = 3; h.impactPoint = {1800, 0, 0};
    CHECK(judge(v, h, 1, 0, w).apply);
    // an unknown impact point (zero) skips the check
    h.hitId = 4; h.impactPoint = {};
    CHECK(judge(v, h, 1, 0, w).apply);
}

TEST(hits, line_of_fire) {   // twin 322
    FakeWorld w = standardWorld();
    w.los = false;
    HitValidator v;
    CHECK(judge(v, goodHit(1), 1, 0, w).reason == HitReason::NoLineOfFire);
    w.los = true;
    CHECK(judge(v, goodHit(2), 1, 0, w).apply);
}

TEST(hits, rate_of_fire_per_weapon) {   // twin 313
    FakeWorld w = standardWorld();
    HitValidator v;
    CombatRules r = defaultRules();
    v.setRules(r);
    // rifle: 200 ms between shots -> 5 per second + 2 slack = 7
    int accepted = 0;
    for (int i = 0; i < 12; ++i) {
        HitEvent h = goodHit(1000 + static_cast<uint64_t>(i));
        h.firedAtTick = 10000;
        if (judge(v, h, 1, 0, w, 10000 + static_cast<uint64_t>(i)).apply) ++accepted;
    }
    CHECK_MSG(accepted == 7, "accepted " + std::to_string(accepted));
    // a second later the budget is back
    HitEvent later = goodHit(2000);
    later.firedAtTick = 11100;
    CHECK(judge(v, later, 1, 0, w, 11100).apply);
    // a shotgun shot is 8 pellets at once: one trigger pull is never rejected
    int pellets = 0;
    for (int i = 0; i < 8; ++i) {
        HitEvent h = goodHit(3000 + static_cast<uint64_t>(i));
        h.weapon = shotgun();
        h.firedAtTick = 20000;
        h.damage = 10;
        if (judge(v, h, 1, 0, w, 20000).apply) ++pellets;
    }
    CHECK(pellets == 8);
    // two different players have separate budgets
    HitEvent p0 = goodHit(4000);
    p0.attacker = Entity::player(0);
    p0.victim = Entity::actor(100);
    p0.firedAtTick = 10000;
    HitValidator v2;
    int a = 0, b = 0;
    for (int i = 0; i < 12; ++i) {
        HitEvent h1 = goodHit(5000 + static_cast<uint64_t>(i));
        HitEvent h0 = p0; h0.hitId = 6000 + static_cast<uint64_t>(i);
        if (judge(v2, h1, 1, 0, w, 10000).apply) ++a;
        if (judge(v2, h0, 0, 0, w, 10000).apply) ++b;
    }
    CHECK(a == 7 && b == 7);
}

TEST(hits, rejected_hits_do_not_spend_the_rate_budget) {
    FakeWorld w = standardWorld();
    HitValidator v;
    for (int i = 0; i < 50; ++i) {
        HitEvent bad = goodHit(100 + static_cast<uint64_t>(i));
        bad.damage = 100000;
        CHECK(!judge(v, bad, 1, 0, w).apply);
    }
    CHECK(judge(v, goodHit(1), 1, 0, w).apply);
}

TEST(hits, no_self_hits) {   // twin 314
    FakeWorld w = standardWorld();
    HitValidator v;
    HitEvent h;
    h.hitId = 1; h.attacker = Entity::player(0); h.victim = Entity::player(0); h.damage = 10; h.claimedRange = 100; h.firedAtTick = 10000;
    CHECK(judge(v, h, 0, 0, w).reason == HitReason::SelfHit);
}

TEST(hits, a_dead_attacker_cannot_hit) {   // twin 316
    FakeWorld w = standardWorld();
    w.f[Entity::player(1)].alive = false;
    HitValidator v;
    CHECK(judge(v, goodHit(1), 1, 0, w).reason == HitReason::AttackerDead);
}

TEST(hits, a_victim_changing_owner_is_not_hit_here) {   // twin 317
    FakeWorld w = standardWorld();
    w.f[Entity::actor(100)].handingOff = true;
    HitValidator v;
    HitVerdict r = judge(v, goodHit(1), 1, 0, w);
    CHECK(!r.apply);
    CHECK(r.reason == HitReason::HandoffInProgress);
    // the handoff did not consume the id: the new owner can still judge the same hit once it is there
    w.f[Entity::actor(100)].handingOff = false;
    CHECK(judge(v, goodHit(1), 1, 0, w).apply);
}

TEST(hits, projectile_must_belong_to_the_attacker) {   // twin 320
    FakeWorld w = standardWorld();
    w.projectiles[77] = Entity::player(0);   // p0 fired projectile 77
    HitValidator v;
    HitEvent h = goodHit(1);
    h.projectile = 77;   // p1 claims it hit with p0's projectile
    CHECK(judge(v, h, 1, 0, w).reason == HitReason::ProjectileShooterMismatch);
    w.projectiles[77] = Entity::player(1);
    h.hitId = 2;
    CHECK(judge(v, h, 1, 0, w).apply);
    CombatRules r = defaultRules();
    r.requireKnownProjectile = true;
    HitValidator strict(r);
    h.hitId = 3; h.projectile = 999;
    CHECK(judge(strict, h, 1, 0, w).reason == HitReason::UnknownProjectile);
}

TEST(hits, explosion_hits_are_judged_against_the_blast) {   // twin 321
    FakeWorld w = standardWorld();
    HitValidator v;
    HitEvent h;
    h.hitId = explosionHitId(9, Entity::actor(100));
    h.attacker = Entity::player(1); h.victim = Entity::actor(100); h.damage = 40; h.explosionId = 9;
    h.impactPoint = {5000, 0, 0}; h.claimedRange = 300; h.firedAtTick = 10000;   // blast centre 4000 units from the victim
    CHECK(judge(v, h, 1, 0, w).reason == HitReason::ExplosionOutOfRadius);
    h.hitId = explosionHitId(10, Entity::actor(100)); h.explosionId = 10; h.impactPoint = {1100, 0, 0};
    CHECK(judge(v, h, 1, 0, w).apply);
    // explosion damage may hurt the thrower
    HitEvent self;
    self.hitId = explosionHitId(11, Entity::player(1)); self.explosionId = 11;
    self.attacker = Entity::player(1); self.victim = Entity::player(1); self.damage = 30; self.impactPoint = {500, 0, 0}; self.claimedRange = 300; self.firedAtTick = 10000;
    HitValidator atP1;
    CHECK(atP1.judge(self, 1, 1, kNow, w).apply);
    // ... and a thrower who died with the grenade still in the air does not cancel the blast
    w.f[Entity::player(1)].alive = false;
    h.hitId = explosionHitId(12, Entity::actor(100)); h.explosionId = 12; h.impactPoint = {1000, 0, 0};
    CHECK(judge(v, h, 1, 0, w).apply);
}

TEST(hits, unknown_things) {
    FakeWorld w = standardWorld();
    HitValidator v;
    HitEvent h = goodHit(1);
    h.victim = Entity::actor(9999);
    CHECK(judge(v, h, 1, 0, w).reason == HitReason::UnknownVictim);
    h = goodHit(2);
    h.attacker = Entity::actor(9999);
    CHECK(!judge(v, h, 1, 0, w).apply);
    h = goodHit(3);
    h.attacker = Entity();
    CHECK(!judge(v, h, 1, 0, w).apply);
    CHECK(std::string(toString(HitReason::OutOfRange)) == "out of weapon range");
}

TEST(hits, a_players_hit_from_before_its_rejoin_is_refused) {   // twin 366 (docs/entity-core-plan.md S7)
    FakeWorld w = standardWorld();
    w.f[Entity::player(1)].epoch = 3;          // player 1 rejoined twice: its pinned entity is at epoch 3
    HitValidator v;
    HitEvent h = goodHit(1);
    h.attackerEpoch = 2;                       // sent by the session before the rejoin
    CHECK(judge(v, h, 1, 0, w).reason == HitReason::StaleAttackerEpoch);
    h.hitId = 2; h.attackerEpoch = 3;
    CHECK(judge(v, h, 1, 0, w).apply);
    h.hitId = 3; h.attackerEpoch = 0;          // a reporter that does not know the epoch yet: judged as before
    CHECK(judge(v, h, 1, 0, w).apply);
}
