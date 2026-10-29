// plugin/combat: choreography tests on a cluster of machines (harness.h). Every test names the rule it guards; the mutation twins 701..721 in
// combat_sync.cpp (plus synccore/combat's 301..362, which this binary also links) must each break at least one of them.
#include "harness.h"

using namespace ctest;

namespace {

constexpr uint32_t A100 = 100, A101 = 101, A102 = 102, A200 = 200;

Bytes hitDatagram(PlayerId src, PlayerId dest, const cc::HitEvent& h, uint8_t hops = 0) {
    Envelope e; e.src = src; e.dest = dest; e.hops = hops; e.body = MHit{h};
    return encodeEnvelope(e);
}
cc::HitEvent playerHit(uint64_t id, PlayerId attacker, cc::Entity victim, int32_t dmg) {
    cc::HitEvent h;
    h.hitId = id; h.attacker = cc::Entity::player(attacker); h.victim = victim; h.damage = dmg; h.weapon = rifle(); h.claimedRange = 3000;
    return h;
}
int damageOn(Cluster& c, int machine, uint32_t victimRef) {
    int n = 0;
    for (auto& d : c.at(machine).eng.damage) if (d.ref == victimRef) n += d.amount;
    return n;
}
int callsOn(Cluster& c, int machine) { return static_cast<int>(c.at(machine).eng.damage.size()); }
int health(Cluster& c, int machine, uint32_t actor) { return c.at(machine).world.recs[cc::Entity::actor(actor)].health; }
bool alive(Cluster& c, int machine, uint32_t actor) { return c.at(machine).world.recs[cc::Entity::actor(actor)].alive; }

// ---------------------------------------------------------------------------------------------------- hits: one resolution, by the owner
TEST(combat_hits, remote_owner_applies_once_and_everyone_mirrors) {
    Cluster c(3);
    c.addActor(A100, 1, 3, {800, 0, 0});
    const GateDecision g = c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 20));
    CHECK(g.action == GateAction::Clamp);
    CHECK(g.maxHealthDamage == 99);                 // the copy keeps >= 1 health: a prediction can never kill it
    c.run(100);
    CHECK(callsOn(c, 1) == 1);
    CHECK(damageOn(c, 1, refOfActor(A100)) == 20);
    CHECK(callsOn(c, 0) == 0);                      // the host only relays
    CHECK(callsOn(c, 2) == 0);                      // the attacker's machine never applies the damage itself
    CHECK(health(c, 1, A100) == 80);
    CHECK(health(c, 0, A100) == 80);                // vitals mirror
    CHECK(health(c, 2, A100) == 80);
    CHECK(c.cs(1).stats().hitsApplied == 1);
    CHECK(c.cs(0).stats().hitsApplied == 0);
    CHECK(c.cs(2).stats().hitsApplied == 0);
    CHECK(c.cs(2).inflightHits() == 1);             // forgotten after the ttl
    REQUIRE(c.at(1).eng.provokes.size() == 1);      // the victim fights back at the attacker's puppet
    CHECK(c.at(1).eng.provokes[0].second == 0x2002);
    c.run(5000);
    CHECK(c.cs(2).inflightHits() == 0);
}

TEST(combat_hits, host_attacker_and_host_owner_paths) {
    Cluster c(2);
    c.addActor(A100, 1, 3, {500, 0, 0});
    c.addActor(A101, 0, 1, {500, 100, 0});
    // the host shoots a client's actor: host -> owner directly
    c.cs(0).onEngineHit(c.shot(0, refOfActor(A100), 30));
    c.run(60);
    CHECK(damageOn(c, 1, refOfActor(A100)) == 30);
    // the client shoots the host's actor: the host is the owner and the only receiver
    c.cs(1).onEngineHit(c.shot(1, refOfActor(A101), 25));
    c.run(60);
    CHECK(damageOn(c, 0, refOfActor(A101)) == 25);
    CHECK(callsOn(c, 1) == 1);
}

TEST(combat_hits, a_hit_on_an_actor_this_machine_owns_is_native_and_publishes_vitals_later) {
    Cluster c(2);
    c.addActor(A100, 1, 3, {500, 0, 0});
    const GateDecision g = c.cs(1).onEngineHit(c.shot(1, refOfActor(A100), 25));
    CHECK(g.action == GateAction::Proceed);        // the engine applies it; CombatSync only records and publishes
    CHECK(c.cs(1).stats().naturalHits == 1);
    CHECK(callsOn(c, 1) == 0);
    c.at(1).world.recs[cc::Entity::actor(A100)].health = 75;   // what the engine did
    c.run(300);
    CHECK(health(c, 0, A100) == 75);               // the coalesced vitals reached the host's copy
    CHECK(c.cs(1).stats().vitalsSent == 1);
}

TEST(combat_hits, duplicate_delivery_resolves_the_hit_once) {
    Cluster c(3);
    c.copies = [](const Datagram&) { return 2; };
    c.addActor(A100, 1, 3, {800, 0, 0});
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 20));
    c.run(100);
    CHECK(callsOn(c, 1) == 1);
    CHECK(damageOn(c, 1, refOfActor(A100)) == 20);
    CHECK(c.cs(1).stats().hitDuplicates >= 1);
    CHECK(health(c, 1, A100) == 80);
}

TEST(combat_hits, a_machine_that_does_not_own_the_victim_never_applies_a_hit) {
    Cluster c(3);
    c.addActor(A100, 1, 3, {800, 0, 0});
    // a hit delivered to the WRONG machine (stale routing): it is not applied there, it is NACKed back to the author
    const Bytes wrong = hitDatagram(0, 2, playerHit(77, 0, cc::Entity::actor(A100), 20));
    c.cs(2).receive(0, wrong.data(), wrong.size());
    c.run(100);
    CHECK(callsOn(c, 2) == 0);
    CHECK(c.cs(2).stats().hitsApplied == 0);
    CHECK(c.cs(2).stats().nacksSent == 1);
    CHECK(callsOn(c, 1) == 0);                     // and nobody re-sent it under the author's name
    // the real owner applies it once when it arrives the proper way
    const Bytes right = hitDatagram(0, 1, playerHit(77, 0, cc::Entity::actor(A100), 20));
    c.cs(0).receive(0, right.data(), right.size());   // (the host routes it to the owner by its own table)
    c.run(100);
    CHECK(callsOn(c, 1) == 1);
    CHECK(callsOn(c, 2) == 0);
}

TEST(combat_hits, the_host_re_routes_a_hit_aimed_at_a_stale_owner) {
    Cluster c(3);
    c.addActor(A100, 1, 3, {800, 0, 0});
    // the host already moved the actor to itself (epoch 4); the attacker's table still says machine 1
    c.setOwner(A100, 0, 4, 0);
    c.setOwner(A100, 0, 4, 1);
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 20));
    c.run(100);
    CHECK(callsOn(c, 0) == 1);                     // the host (new owner) resolved it
    CHECK(callsOn(c, 1) == 0);
}

TEST(combat_hits, stale_attacker_epoch_is_rejected_and_the_current_one_accepted) {
    Cluster c(3);
    c.addActor(A101, 2, 5, {300, 0, 0});           // an NPC owned by machine 2 at epoch 5
    cc::HitEvent h;
    h.hitId = 9001; h.attacker = cc::Entity::actor(A101); h.victim = cc::Entity::player(0); h.damage = 10; h.attackerEpoch = 4; h.claimedRange = 400;
    c.cs(0).receive(2, hitDatagram(2, 0, h).data(), hitDatagram(2, 0, h).size());
    c.run(50);
    CHECK(callsOn(c, 0) == 0);                     // the host (the player's machine) refused the stale report
    h.hitId = 9002; h.attackerEpoch = 5;
    c.cs(0).receive(2, hitDatagram(2, 0, h).data(), hitDatagram(2, 0, h).size());
    c.run(50);
    CHECK(callsOn(c, 0) == 1);
    CHECK(damageOn(c, 0, 0x14) == 10);
}

TEST(combat_hits, a_forged_attacker_is_rejected) {
    Cluster c(3);
    c.addActor(A101, 1, 5, {300, 0, 0});           // owned by machine 1, but machine 2 claims to speak for it
    cc::HitEvent h;
    h.hitId = 9100; h.attacker = cc::Entity::actor(A101); h.victim = cc::Entity::player(0); h.damage = 10; h.attackerEpoch = 5; h.claimedRange = 400;
    const Bytes b = hitDatagram(2, 0, h);
    c.cs(0).receive(2, b.data(), b.size());
    c.run(50);
    CHECK(callsOn(c, 0) == 0);
    cc::HitEvent p = playerHit(9101, 1, cc::Entity::actor(A101), 10);   // a player hit "from" another player
    const Bytes b2 = hitDatagram(2, 1, p);
    c.cs(0).receive(2, b2.data(), b2.size());
    c.run(50);
    CHECK(callsOn(c, 1) == 0);
}

// ---------------------------------------------------------------------------------------------------- the gate
TEST(combat_gate, a_copy_that_hits_something_is_neutralised_and_reports_nothing) {
    Cluster c(3);
    c.addActor(A101, 1, 4, {300, 0, 0});           // owned by machine 1: a COPY on machine 2
    EngineHit h;
    h.attackerRef = refOfActor(A101); h.victimRef = kLocalPlayerRef; h.healthDmg = 25; h.weapon = rifle();
    const GateDecision g = c.cs(2).onEngineHit(h);
    CHECK(g.action == GateAction::Neutralize);     // a visual-only shot/swing: no damage to the local player
    CHECK(c.cs(2).stats().ghostHitsNeutralized == 1);
    CHECK(c.sentTotal == 0);
    // the same hit made by the real NPC on its owner is reported to the player's machine
    h.victimRef = 0x2002;                          // the puppet of player 2 as machine 1 sees it
    const GateDecision g1 = c.cs(1).onEngineHit(h);
    CHECK(g1.action == GateAction::Clamp);
    c.run(100);
    CHECK(callsOn(c, 2) == 1);                     // the player's machine applied it ...
    CHECK(damageOn(c, 2, 0x14) == 25);             // ... to the local player
    CHECK(callsOn(c, 1) == 0);
}

TEST(combat_gate, the_copy_keeps_at_least_one_health_whatever_the_local_engine_computed) {
    Cluster c(2);
    c.addActor(A100, 0, 3, {800, 0, 0}, 30);
    GateDecision g = c.cs(1).onEngineHit(c.shot(1, refOfActor(A100), 500));
    CHECK(g.action == GateAction::Clamp);
    CHECK(g.maxHealthDamage == 29);
    c.at(1).world.recs[cc::Entity::actor(A100)].health = 1;
    g = c.cs(1).onEngineHit(c.shot(1, refOfActor(A100), 500));
    CHECK(g.action == GateAction::Clamp);
    CHECK(g.maxHealthDamage == 0);
}

TEST(combat_gate, environment_hits_on_a_copy_are_neutralised_and_on_an_owned_victim_proceed) {
    Cluster c(2);
    c.addActor(A100, 0, 3, {800, 0, 0});
    EngineHit h; h.victimRef = refOfActor(A100); h.healthDmg = 10;
    CHECK(c.cs(1).onEngineHit(h).action == GateAction::Neutralize);   // a trap on a copy: the owner's machine runs the same trap
    CHECK(c.cs(0).onEngineHit(h).action == GateAction::Proceed);
    h.victimRef = 0x777;                           // not a synced combatant at all
    CHECK(c.cs(0).onEngineHit(h).action == GateAction::Proceed);
}

TEST(combat_gate, an_actor_nobody_owns_yet_is_resolved_locally) {
    Cluster c(2);
    c.addActor(A100, kNoPlayer, 0, {800, 0, 0});
    CHECK(c.cs(1).onEngineHit(c.shot(1, refOfActor(A100), 20)).action == GateAction::Proceed);
    CHECK(c.sentTotal == 0);
}

TEST(combat_gate, player_versus_player_damage_is_off_unless_friendly_fire_is_on) {
    Cluster off(2);
    EngineHit h; h.attackerRef = kLocalPlayerRef; h.victimRef = 0x2000; h.healthDmg = 10; h.weapon = rifle(); h.weaponRange = 3000;
    CHECK(off.cs(1).onEngineHit(h).action == GateAction::Neutralize);
    CHECK(off.cs(1).stats().friendlyNeutralized == 1);
    CHECK(off.sentTotal == 0);
    Cluster on(2, [](Config& cfg) { cfg.friendlyFire = true; });
    CHECK(on.cs(1).onEngineHit(h).action == GateAction::Clamp);
    on.run(100);
    CHECK(damageOn(on, 0, 0x14) == 10);
}

TEST(combat_gate, friendly_fire_follows_the_campaigns_choice_while_the_session_runs) {
    Cluster c(2);
    EngineHit h; h.attackerRef = kLocalPlayerRef; h.victimRef = 0x2000; h.healthDmg = 10; h.weapon = rifle(); h.weaponRange = 3000;
    CHECK(c.cs(1).onEngineHit(h).action == GateAction::Neutralize);
    c.cs(1).setFriendlyFire(true);                     // the host's campaign has it on: told after the session started
    CHECK(c.cs(1).friendlyFire());
    CHECK(c.cs(1).onEngineHit(h).action == GateAction::Clamp);
    c.cs(1).setFriendlyFire(false);
    CHECK(c.cs(1).onEngineHit(h).action == GateAction::Neutralize);
}

TEST(combat_gate, vats_shots_travel_the_same_path_and_keep_their_flag) {
    Cluster c(3);
    c.addActor(A100, 1, 3, {800, 0, 0});
    c.at(2).eng.vats = true;
    EngineHit h = c.shot(2, refOfActor(A100), 20, 1);   // location 1 = head
    c.cs(2).onEngineHit(h);
    c.run(100);
    REQUIRE(c.at(1).eng.damage.size() == 1);
    CHECK(c.at(1).eng.damage[0].vats);
    CHECK(c.at(1).eng.damage[0].limb == cc::Limb::Head);
    c.at(2).eng.vats = false;
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 5));
    c.run(100);
    REQUIRE(c.at(1).eng.damage.size() == 2);
    CHECK(!c.at(1).eng.damage[1].vats);
}

TEST(combat_gate, a_weapon_the_table_does_not_know_still_gets_a_plausible_reach) {
    Cluster c(3);
    c.addActor(A100, 1, 3, {800, 0, 0});
    EngineHit h = c.shot(2, refOfActor(A100), 20);
    h.weapon = FormRef("falloutnv.esm", 0x4242);   // not in the weapon table
    h.weaponRange = 0;                              // and the engine gave no reach
    c.cs(2).onEngineHit(h);
    c.run(100);
    CHECK(damageOn(c, 1, refOfActor(A100)) == 20);
    EngineHit melee = c.shot(2, refOfActor(A100), 5);
    melee.weapon = FormRef(); melee.weaponRange = 0; melee.melee = true;
    c.cs(2).onEngineHit(melee);
    c.run(100);
    CHECK(damageOn(c, 1, refOfActor(A100)) == 25);
}

TEST(combat_gate, an_impact_vector_that_is_not_a_world_position_does_not_get_a_good_hit_rejected) {
    Cluster c(3);
    c.addActor(A100, 1, 3, {800, 0, 0});
    EngineHit h = c.shot(2, refOfActor(A100), 20);
    h.impact = {5, 7, 9};                           // far from the victim: local-space or uninitialised, not a place in the world
    c.cs(2).onEngineHit(h);
    h.impact = {810, 5, 40};                        // a real impact point on the victim
    c.cs(2).onEngineHit(h);
    c.run(100);
    CHECK_MSG(damageOn(c, 1, refOfActor(A100)) == 40, c.cs(1).lastRejections());
    CHECK(c.cs(1).stats().hitsRejected == 0);
}

TEST(combat_gate, a_hit_with_nothing_to_report_sends_nothing) {
    // (a light blow, 0.2, is NOT nothing: it is one point, tests/test_melee.cpp; a blow with no health damage is a touch there too)
    Cluster c(2);
    c.addActor(A100, 0, 3, {800, 0, 0});
    EngineHit h = c.shot(1, refOfActor(A100), 0.0f);
    h.attackerRef = 0;                              // no attacker: nobody to report
    CHECK(c.cs(1).onEngineHit(h).action == GateAction::Neutralize);
    CHECK(c.sentTotal == 0);
}

// ---------------------------------------------------------------------------------------------------- retries: a hit is never silently lost
TEST(combat_retry, a_handoff_in_flight_nacks_the_hit_and_the_attacker_re_sends_it_to_the_new_owner) {
    Cluster c(3);
    c.addActor(A100, 1, 3, {800, 0, 0});
    c.at(1).world.recs[cc::Entity::actor(A100)].pending = true;   // machine 1 is handing the actor over
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 20));
    c.run(60);
    CHECK(callsOn(c, 1) == 0);
    CHECK(c.cs(1).stats().nacksSent == 1);
    CHECK(c.cs(2).stats().nacksReceived == 1);
    c.setOwner(A100, 0, 4);                         // the handoff finishes: the host owns it now
    c.run(400);
    CHECK(callsOn(c, 0) == 1);
    CHECK(damageOn(c, 0, refOfActor(A100)) == 20);
    CHECK(callsOn(c, 1) == 0);
    CHECK(c.cs(2).stats().hitRetries == 1);
}

TEST(combat_retry, an_ownership_table_that_is_behind_gets_the_hit_again_under_a_fresh_id) {
    Cluster c(3);
    c.addActor(A100, 1, 3, {800, 0, 0});
    c.addActor(A101, 2, 6, {900, 0, 0});            // machine 2 just got NPC 101 (epoch 6) ...
    c.setOwner(A101, 2, 5, 1);                      // ... but machine 1 still thinks epoch 5
    c.setOwner(A101, 2, 5, 0);
    EngineHit h;
    h.attackerRef = refOfActor(A101); h.victimRef = refOfActor(A100); h.healthDmg = 15; h.weapon = rifle(); h.weaponRange = 3000;
    c.cs(2).onEngineHit(h);
    c.run(60);
    CHECK(callsOn(c, 1) == 0);
    CHECK(c.cs(1).stats().resyncHints == 1);
    c.setOwner(A101, 2, 6, 1);
    c.setOwner(A101, 2, 6, 0);
    c.run(400);
    CHECK(callsOn(c, 1) == 1);
    CHECK(damageOn(c, 1, refOfActor(A100)) == 15);
}

TEST(combat_retry, a_hit_that_never_finds_an_owner_is_abandoned_not_looped) {
    Cluster c(3);
    c.addActor(A100, 1, 3, {800, 0, 0});
    c.at(1).world.recs[cc::Entity::actor(A100)].pending = true;
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 20));
    c.run(3000);
    CHECK(callsOn(c, 1) == 0);
    CHECK(c.cs(2).stats().hitRetries <= 3);
    CHECK(c.cs(2).stats().hitsAbandoned == 1);
    CHECK(c.cs(2).inflightHits() == 0);
}

// ---------------------------------------------------------------------------------------------------- deaths
TEST(combat_death, a_lethal_hit_kills_once_and_the_killer_gets_the_xp_once) {
    Cluster c(3);
    c.copies = [](const Datagram&) { return 2; };   // every datagram twice
    c.addActor(A100, 1, 3, {800, 0, 0}, 50);
    for (auto& m : c.m) m->eng.xpValue = 75;
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 60));
    c.run(300);
    CHECK(!alive(c, 1, A100));
    CHECK(!alive(c, 0, A100));
    CHECK(!alive(c, 2, A100));
    CHECK(c.cs(1).stats().deathsDeclared == 1);
    CHECK(c.cs(0).stats().deathsDeclared == 0);
    CHECK(c.cs(2).stats().deathsDeclared == 0);
    CHECK(c.cs(0).stats().deathsAccepted == 1);
    CHECK(c.cs(2).stats().deathsAccepted == 1);
    REQUIRE(c.at(2).eng.xp.size() == 1);            // the killing blow was machine 2's player
    CHECK(c.at(2).eng.xp[0] == 75);
    CHECK(c.at(0).eng.xp.empty());
    CHECK(c.at(1).eng.xp.empty());
    CHECK(c.cs(2).deaths().isDead(cc::Entity::actor(A100)));
    const cc::DeathDecl* d = c.cs(2).deaths().record(cc::Entity::actor(A100));
    REQUIRE(d != nullptr);
    CHECK(d->killer == cc::Entity::player(2));
    CHECK(d->weapon == rifle());
}

// ---- G04: a guest's kill is a kill BY that guest on the host (IsKiller player) --------------------------------------------------------------
TEST(combat_killer, the_host_owner_kills_a_victim_of_a_remote_players_lethal_shot_as_that_players_puppet) {
    Cluster c(3);
    c.addActor(A100, 0, 3, {200, 0, 0}, 50);                       // the host owns it
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 80));          // guest 2 shoots it dead
    c.run(300);
    CHECK(!alive(c, 0, A100));
    CHECK_MSG(c.at(0).eng.killsBy.size() == 1, "the lethal blow was not credited to the guest");
    if (!c.at(0).eng.killsBy.empty()) CHECK(c.at(0).eng.killsBy[0].first == refOfActor(A100) && c.at(0).eng.killsBy[0].second == refOfPlayer(0, 2));
    CHECK(c.cs(0).stats().killsAttributed >= 1);
    // a shot of the host's own player stays a plain damage line (the engine attributes the local player itself)
    c.addActor(A200, 0, 3, {250, 0, 0}, 50);
    c.cs(0).onEngineHit(c.shot(0, refOfActor(A200), 80));
    c.run(300);
    CHECK(c.at(0).eng.killsBy.size() == 1);
}

TEST(combat_killer, the_host_copy_of_a_guests_victim_is_killed_with_the_guests_credit_whoever_owns_it) {
    Cluster c(3);
    c.addActor(A100, 1, 3, {800, 0, 0}, 50);                       // guest 1 owns it, guest 2 shoots it
    CHECK(c.cs(0).killerPuppetOf(refOfActor(A100)) == 0);          // nobody has shot it yet
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 80));
    c.run(300);
    CHECK(!alive(c, 0, A100));
    CHECK(c.cs(0).killerPuppetOf(refOfActor(A100)) == refOfPlayer(0, 2));   // what the actor port asks when it kills the host's copy
    CHECK(c.cs(1).killerPuppetOf(refOfActor(A100)) == 0 && c.cs(2).killerPuppetOf(refOfActor(A100)) == 0);   // only the host runs the story
}

TEST(combat_killer, a_corpse_that_was_already_here_gets_the_killer_written_and_a_lethal_blow_with_a_float_sliver_is_still_credited) {
    Cluster c(3);
    c.propagateDeaths = false;
    c.addActor(A100, 1, 3, {800, 0, 0}, 50);
    c.at(0).world.recs[cc::Entity::actor(A100)].alive = false;     // the host's copy died before the declaration got there
    c.at(0).world.recs[cc::Entity::actor(A100)].health = 0;
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 80));
    c.run(300);
    CHECK_MSG(c.at(0).eng.killerSet.size() == 1, "the corpse never got its killer");
    if (!c.at(0).eng.killerSet.empty()) CHECK(c.at(0).eng.killerSet[0].second == refOfPlayer(0, 2));
    // the lethal blow of a remote player on a victim the engine would leave standing (a float sliver): the credited kill finishes it in one go
    Cluster d(3);
    d.addActor(A100, 0, 3, {200, 0, 0}, 12);
    d.at(0).world.recs[cc::Entity::actor(A100)].sliver = 0.4f;
    d.at(0).eng.refuseLethal = true;
    d.cs(2).onEngineHit(d.shot(2, refOfActor(A100), 20));
    d.run(900);
    CHECK(!alive(d, 0, A100));
    CHECK(!d.at(0).eng.killsBy.empty() && d.at(0).eng.killsBy.back().second == refOfPlayer(0, 2));
}

TEST(combat_death, a_native_kill_by_the_local_player_is_not_paid_twice) {
    Cluster c(2);
    c.addActor(A100, 1, 3, {500, 0, 0}, 50);
    for (auto& m : c.m) m->eng.xpValue = 60;
    // machine 1 owns the actor AND its player shoots it: the local engine applies the hit and (as in the vanilla game) pays the kill XP itself
    CHECK(c.cs(1).onEngineHit(c.shot(1, refOfActor(A100), 80)).action == GateAction::Proceed);
    Rec& r = c.at(1).world.recs[cc::Entity::actor(A100)];
    r.health = 0; r.alive = false;
    c.cs(1).onActorDied(refOfActor(A100), true, false, true, 0);
    c.run(100);
    CHECK(c.cs(1).stats().deathsDeclared == 1);
    CHECK(c.at(1).eng.xp.empty());                  // CombatSync does not add a second payment
    CHECK(c.cs(1).stats().xpEnginePaid == 1);
    CHECK(c.at(0).eng.xp.empty());                  // and nobody else earned it
}

TEST(combat_death, the_owner_declares_a_death_once_even_if_the_port_reports_it_twice) {
    Cluster c(2);
    c.addActor(A100, 1, 3, {800, 0, 0}, 50);
    c.at(1).world.recs[cc::Entity::actor(A100)].alive = false;
    c.cs(1).onActorDied(refOfActor(A100), true, false, true, 0);
    c.cs(1).onActorDied(refOfActor(A100), true, false, true, 0);
    c.run(100);
    CHECK(c.cs(1).stats().deathsDeclared == 1);
    CHECK(c.cs(1).stats().deathsRejected == 1);     // the second one: AlreadyDead
    CHECK(c.cs(0).stats().deathsAccepted == 1);
}

TEST(combat_death, a_death_that_is_not_authorised_or_not_mine_declares_nothing) {
    Cluster c(2);
    c.addActor(A100, 0, 3, {800, 0, 0}, 50);
    c.at(1).world.recs[cc::Entity::actor(A100)].alive = false;
    c.cs(1).onActorDied(refOfActor(A100), false, false, false, 0);   // a copy died locally by prediction
    c.run(50);
    CHECK(c.cs(1).stats().deathsDeclared == 0);
    CHECK(c.cs(1).stats().deathsRefusedLocal == 1);
    CHECK(c.sentTotal == 0);
}

TEST(combat_death, a_named_npc_is_never_killed_by_a_message_only_the_host_story_can) {
    Cluster c(3);
    c.addActor(A102, 1, 2, {500, 0, 0});
    c.forAll(A102, [](Rec& r) { r.named = true; });
    cc::DeathDecl d;
    d.deathId = 5001; d.who = cc::Entity::actor(A102); d.source = cc::DeathSource::Owner; d.declarer = 1; d.epoch = 2;
    Envelope e; e.src = 1; e.dest = kNoPlayer; e.body = MDeath{d};
    Bytes b = encodeEnvelope(e);
    c.cs(0).receive(1, b.data(), b.size());          // the owner (a client) declares the death of a story NPC
    c.run(50);
    CHECK(!c.cs(0).deaths().isDead(cc::Entity::actor(A102)));
    CHECK(!c.cs(2).deaths().isDead(cc::Entity::actor(A102)));
    d.deathId = 5002; d.source = cc::DeathSource::StoryHost; d.declarer = 1;
    e.body = MDeath{d};
    b = encodeEnvelope(e);
    c.cs(0).receive(1, b.data(), b.size());          // a client claims to be the story host
    c.run(50);
    CHECK(!c.cs(0).deaths().isDead(cc::Entity::actor(A102)));
    d.deathId = 5003; d.declarer = 0; e.src = 0;
    e.body = MDeath{d};
    b = encodeEnvelope(e);
    c.cs(2).receive(0, b.data(), b.size());          // the host's KillActor effect
    CHECK(c.cs(2).deaths().isDead(cc::Entity::actor(A102)));
}

TEST(combat_death, a_story_kill_of_a_named_npc_is_declared_by_the_host_only) {
    Cluster c(3);
    c.addActor(A102, 1, 2, {500, 0, 0});
    c.forAll(A102, [](Rec& r) { r.named = true; });
    for (auto& m : c.m) m->eng.xpValue = 90;
    // the host's KillActor effect runs on every machine: each actor port reports an authorised death (the owner says mine = true)
    c.cs(1).onActorDied(refOfActor(A102), true, false, true, 0);
    c.cs(2).onActorDied(refOfActor(A102), true, false, false, 0);
    CHECK(c.cs(1).stats().deathsDeclared == 0);
    CHECK(c.cs(2).stats().deathsDeclared == 0);
    c.cs(0).onActorDied(refOfActor(A102), true, false, false, 0);
    c.run(100);
    CHECK(c.cs(0).stats().deathsDeclared == 1);
    CHECK(c.cs(1).deaths().isDead(cc::Entity::actor(A102)));
    CHECK(c.cs(2).deaths().isDead(cc::Entity::actor(A102)));
    const cc::DeathDecl* d = c.cs(1).deaths().record(cc::Entity::actor(A102));
    REQUIRE(d != nullptr);
    CHECK(d->source == cc::DeathSource::StoryHost);
}

TEST(combat_death, combat_alone_floors_a_named_npc_at_one_health) {
    Cluster c(3);
    c.addActor(A102, 1, 2, {500, 0, 0});
    c.forAll(A102, [](Rec& r) { r.named = true; });
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A102), 400));
    c.run(100);
    CHECK(health(c, 1, A102) == 1);
    CHECK(alive(c, 1, A102));
    CHECK(c.cs(1).stats().deathsDeclared == 0);
}

TEST(combat_death, a_death_declared_by_a_machine_that_is_not_the_owner_is_refused) {
    Cluster c(3);
    c.addActor(A100, 1, 3, {800, 0, 0});
    for (auto& m : c.m) m->eng.xpValue = 40;
    cc::DeathDecl d;
    d.deathId = 6001; d.who = cc::Entity::actor(A100); d.source = cc::DeathSource::Owner; d.declarer = 2; d.epoch = 3; d.killer = cc::Entity::player(2);
    Envelope e; e.src = 2; e.dest = kNoPlayer; e.body = MDeath{d};
    const Bytes b = encodeEnvelope(e);
    c.cs(0).receive(2, b.data(), b.size());
    c.run(50);
    CHECK(!c.cs(0).deaths().isDead(cc::Entity::actor(A100)));
    CHECK(c.at(2).eng.xp.empty());
    CHECK(c.cs(0).stats().deathsRejected == 1);
}

TEST(combat_death, a_declaration_the_ownership_table_is_behind_for_is_retried) {
    Cluster c(3);
    c.addActor(A100, 1, 3, {800, 0, 0});
    cc::DeathDecl d;
    d.deathId = 6101; d.who = cc::Entity::actor(A100); d.source = cc::DeathSource::Owner; d.declarer = 1; d.epoch = 4;   // the owner is already at epoch 4
    Envelope e; e.src = 1; e.dest = kNoPlayer; e.body = MDeath{d};
    const Bytes b = encodeEnvelope(e);
    c.cs(2).receive(0, b.data(), b.size());          // as relayed by the host; machine 2's table says epoch 3
    CHECK(!c.cs(2).deaths().isDead(cc::Entity::actor(A100)));
    CHECK(c.cs(2).stats().deathsDeferred == 1);
    c.setOwner(A100, 1, 4, 2);                       // the table catches up
    c.run(200);
    CHECK(c.cs(2).deaths().isDead(cc::Entity::actor(A100)));
}

TEST(combat_death, crippled_limbs_show_on_the_corpse_even_when_the_kill_arrives_late) {
    Cluster c(3);
    c.propagateDeaths = true;
    c.deathLatency = 400;                            // ActorSync's kill is slow
    c.addActor(A100, 1, 3, {800, 0, 0}, 100);
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 35, 1));   // head: 35 >= 30 cripples it
    c.run(80);
    CHECK((c.at(1).world.recs[cc::Entity::actor(A100)].limbMask & cc::LimbState::bit(cc::Limb::Head)) != 0);
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 80, 0));
    c.run(100);                                      // dead on the owner, not yet on machine 0 and 2
    CHECK(alive(c, 2, A100));
    CHECK(c.cs(2).stats().deathsAccepted == 1);
    CHECK(c.at(2).eng.corpses.empty());
    c.run(600);
    CHECK(!alive(c, 2, A100));
    REQUIRE(!c.at(2).eng.corpses.empty());
    CHECK((c.at(2).eng.corpses[0].second & cc::LimbState::bit(cc::Limb::Head)) != 0);
}

TEST(combat_death, the_local_player_dies_and_can_die_again_after_a_respawn) {
    Cluster c(2);
    c.at(0).world.recs[cc::Entity::player(0)].alive = false;
    c.cs(0).onPlayerDied(0);
    c.cs(0).onPlayerDied(0);
    c.run(100);
    CHECK(c.cs(0).stats().deathsDeclared == 1);
    CHECK(c.cs(1).stats().deathsAccepted == 1);
    CHECK(c.cs(1).deaths().isDead(cc::Entity::player(0)));
    c.cs(0).onPlayerAlive();
    c.at(0).world.recs[cc::Entity::player(0)].alive = true;
    c.cs(0).onPlayerDied(0);
    c.run(100);
    CHECK(c.cs(0).stats().deathsDeclared == 2);
}

// ---------------------------------------------------------------------------------------------------- limbs and vitals
TEST(combat_vitals, limb_damage_accumulates_on_the_owner_and_never_un_cripples_on_a_copy) {
    Cluster c(3);
    c.addActor(A100, 1, 3, {800, 0, 0});
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 20, 3));   // left arm 20 < 25
    c.run(60);
    CHECK(c.at(1).eng.cripples.empty());
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 10, 3));   // 30 >= 25: crippled
    c.run(60);
    REQUIRE(c.at(1).eng.cripples.size() == 1);
    CHECK(c.at(1).eng.cripples[0].second == cc::Limb::LeftArm);
    CHECK((c.at(0).world.recs[cc::Entity::actor(A100)].limbMask & cc::LimbState::bit(cc::Limb::LeftArm)) != 0);
    CHECK((c.at(2).world.recs[cc::Entity::actor(A100)].limbMask & cc::LimbState::bit(cc::Limb::LeftArm)) != 0);
    // a late, older vitals message (no limbs) cannot clear it
    MVitals v; v.who = cc::Entity::actor(A100); v.epoch = 3; v.seq = 1; v.health = 100; v.maxHealth = 100; v.limbMask = 0;
    Envelope e; e.src = 1; e.dest = kNoPlayer; e.body = v;
    const Bytes b = encodeEnvelope(e);
    c.cs(2).receive(0, b.data(), b.size());
    CHECK((c.at(2).world.recs[cc::Entity::actor(A100)].limbMask & cc::LimbState::bit(cc::Limb::LeftArm)) != 0);
    CHECK(health(c, 2, A100) == 70);                 // and the stale health did not come back either
}

TEST(combat_vitals, only_the_owner_speaks_for_an_actor) {
    Cluster c(3);
    c.addActor(A100, 1, 3, {800, 0, 0});
    MVitals v; v.who = cc::Entity::actor(A100); v.epoch = 3; v.seq = 9; v.health = 5; v.maxHealth = 100;
    Envelope e; e.src = 2; e.dest = kNoPlayer; e.body = v;
    const Bytes b = encodeEnvelope(e);
    c.cs(0).receive(2, b.data(), b.size());          // machine 2 is not the owner
    c.run(50);
    CHECK(health(c, 0, A100) == 100);
    CHECK(c.cs(0).stats().spoofed == 1);
    e.src = 1; e.body = v;                           // the real owner at a STALE epoch
    v.epoch = 2; e.body = v;
    const Bytes b2 = encodeEnvelope(e);
    c.cs(0).receive(1, b2.data(), b2.size());
    CHECK(health(c, 0, A100) == 100);
}

TEST(combat_vitals, a_mirrored_health_never_kills_a_living_copy) {
    Cluster c(2);
    c.addActor(A100, 0, 3, {800, 0, 0});
    MVitals v; v.who = cc::Entity::actor(A100); v.epoch = 3; v.seq = 1; v.health = 0; v.maxHealth = 100; v.dead = false;
    Envelope e; e.src = 0; e.dest = kNoPlayer; e.body = v;
    const Bytes b = encodeEnvelope(e);
    c.cs(1).receive(0, b.data(), b.size());
    CHECK(health(c, 1, A100) == 1);
    CHECK(alive(c, 1, A100));
}

// ---------------------------------------------------------------------------------------------------- explosions and projectiles
TEST(combat_blast, each_victim_is_hit_once_and_the_blast_is_visual_elsewhere) {
    Cluster c(3);
    c.addActor(A100, 1, 3, {1000, 0, 0});
    c.addActor(A101, 2, 4, {1000, 200, 0});          // inside the radius, but the local engine never iterated it
    c.addActor(A102, 1, 2, {1000, 3000, 0});         // far outside
    EngineHit h;
    h.attackerRef = kLocalPlayerRef; h.victimRef = refOfActor(A100); h.healthDmg = 80; h.weapon = grenade(); h.explosionKey = 7;
    h.blastCenter = {1000, 100, 0}; h.blastRadius = 500; h.weaponRange = 3000;
    c.cs(0).onEngineHit(h);
    c.cs(0).onEngineHit(h);                          // the engine marks it twice: still one explosion, one hit
    c.run(400);
    CHECK(c.cs(0).stats().explosionsLocal == 1);
    CHECK(c.at(1).eng.explosions.size() == 1);
    CHECK(c.at(2).eng.explosions.size() == 1);
    CHECK(c.at(0).eng.explosions.empty());
    CHECK(callsOn(c, 1) == 1);                       // victim 100: once (the second engine hit and nothing else)
    CHECK(damageOn(c, 1, refOfActor(A100)) == 80);
    CHECK(damageOn(c, 2, refOfActor(A101)) > 0);     // planned for the victim the engine did not iterate
    CHECK(callsOn(c, 2) == 1);
    CHECK(damageOn(c, 1, refOfActor(A102)) == 0);    // out of range
    // the planning pass ran once: asking again produces the same ids, which every validator drops
    c.cs(0).onEngineHit(h);
    c.run(400);
    CHECK(callsOn(c, 1) == 1);
    CHECK(callsOn(c, 2) == 1);
}

TEST(combat_blast, a_clients_explosion_is_passed_on_by_the_host_and_its_visual_copy_cannot_hurt_anybody) {
    Cluster c(3);
    c.addActor(A100, 0, 3, {1000, 0, 0});            // owned by the host
    EngineHit h;
    h.attackerRef = kLocalPlayerRef; h.victimRef = refOfActor(A100); h.healthDmg = 60; h.weapon = grenade(); h.explosionKey = 11;
    h.blastCenter = {1000, 50, 0}; h.blastRadius = 400; h.weaponRange = 3000; h.explosionBase = FormRef("falloutnv.esm", 0x5000);
    c.cs(2).onEngineHit(h);                          // machine 2 throws it
    c.run(300);
    CHECK(c.at(0).eng.explosions.size() == 1);       // the host shows it
    CHECK(c.at(1).eng.explosions.size() == 1);       // and passed it on to machine 1
    CHECK(c.at(2).eng.explosions.empty());
    CHECK(damageOn(c, 0, refOfActor(A100)) == 60);
    // machine 1 now plays the blast; its engine "hits" its local player and an actor it owns, with no attacker at all (the explosion is sourceless there)
    EngineHit ghost;
    ghost.victimRef = kLocalPlayerRef; ghost.healthDmg = 70; ghost.explosionKey = 99; ghost.blastCenter = {1010, 40, 0}; ghost.explosionBase = FormRef("falloutnv.esm", 0x5000);
    CHECK(c.cs(1).onEngineHit(ghost).action == GateAction::Neutralize);   // a visual-only explosion: no damage to the local player
    ghost.blastCenter = {5000, 5000, 0};             // a blast somewhere else is a real one (a trap, another grenade)
    CHECK(c.cs(1).onEngineHit(ghost).action == GateAction::Proceed);
}

TEST(combat_projectile, the_hits_of_a_visual_projectile_are_harmless) {
    Cluster c(2);
    c.addActor(A101, 0, 4, {300, 0, 0});
    cc::ProjectileSpawn p;
    p.shooter = cc::Entity::actor(A101); p.weapon = rifle(); p.origin = {300, 0, 50}; p.velocity = {5000, 0, 0}; p.cell = 1;
    EngineHit h;
    h.victimRef = kLocalPlayerRef; h.healthDmg = 30; h.projectileKey = 1234;   // a bullet with no simulated attacker lands on the local player
    CHECK(c.cs(1).onEngineHit(h).action == GateAction::Proceed);             // before the shot is announced: a plain environment hit
    c.cs(0).onProjectileFired(p);
    c.run(100);
    CHECK(c.at(1).eng.projectiles.size() == 1);
    CHECK(c.cs(1).onEngineHit(h).action == GateAction::Neutralize);          // while the visual copy flies, nothing it hits is hurt
    c.run(4000);
    CHECK(c.cs(1).onEngineHit(h).action == GateAction::Proceed);             // the window closes
}

TEST(combat_blast, a_visual_explosion_never_hurts_the_machine_that_only_shows_it) {
    Cluster c(2);
    EngineHit h;   // machine 1 sees machine 0's grenade: its engine runs the blast on its copies; the source is a puppet there
    h.attackerRef = 0x2000; h.victimRef = kLocalPlayerRef; h.healthDmg = 90; h.weapon = grenade(); h.explosionKey = 3; h.blastCenter = {100, 0, 0}; h.blastRadius = 400;
    CHECK(c.cs(1).onEngineHit(h).action == GateAction::Neutralize);
    CHECK(c.cs(1).stats().explosionsLocal == 0);
}

TEST(combat_projectile, a_shot_is_real_for_its_shooter_and_a_harmless_copy_everywhere_else) {
    Cluster c(3);
    c.addActor(A101, 1, 4, {300, 0, 0});
    cc::ProjectileSpawn p;
    p.shooter = cc::Entity::actor(A101); p.weapon = rifle(); p.origin = {300, 0, 50}; p.velocity = {5000, 0, 0}; p.cell = 1;
    c.cs(1).onProjectileFired(p);
    c.run(100);
    CHECK(c.cs(1).stats().projectilesLocal == 1);
    CHECK(c.at(1).eng.projectiles.empty());          // the shooter has the real one
    CHECK(c.at(0).eng.projectiles.size() == 1);
    CHECK(c.at(2).eng.projectiles.size() == 1);
    CHECK(c.cs(0).projectiles().liveCount() == 1);
    // a machine that does not simulate the shooter cannot declare its shots
    c.cs(2).onProjectileFired(p);
    CHECK(c.cs(2).stats().projectilesRejected == 1);
    c.run(100);
    CHECK(c.at(0).eng.projectiles.size() == 1);
}

TEST(combat_projectile, a_duplicate_projectile_message_shows_one_copy) {
    Cluster c(3);
    c.copies = [](const Datagram&) { return 3; };
    c.addActor(A101, 1, 4, {300, 0, 0});
    cc::ProjectileSpawn p;
    p.shooter = cc::Entity::actor(A101); p.weapon = rifle(); p.origin = {300, 0, 50}; p.velocity = {5000, 0, 0}; p.cell = 1;
    c.cs(1).onProjectileFired(p);
    c.run(100);
    CHECK(c.at(2).eng.projectiles.size() == 1);
    CHECK(c.at(0).eng.projectiles.size() == 1);
}


// ---------------------------------------------------------------------------------------------------- bench 2026-10-27: a guest's kill must reach the owner
// (out/bench/dino-20261027-144058, real_fight: "a kill by the client shows dead on the host: 8 volleys did not kill it; still alive on the host")
TEST(combat_retry, a_hit_nacked_in_a_handoff_that_ends_with_the_attacker_as_owner_is_resolved_by_the_attacker) {
    Cluster c(3);
    c.addActor(A100, 1, 3, {800, 0, 0});
    c.at(1).world.recs[cc::Entity::actor(A100)].pending = true;   // machine 1 is handing the actor to machine 2
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 20));          // machine 2 fires: the copy is clamped, the hit goes to machine 1
    c.run(60);
    CHECK(c.cs(1).stats().nacksSent == 1);
    c.setOwner(A100, 2, 4);                                        // the handoff ends: machine 2 owns it, nobody else will ever resolve this hit
    c.run(500);
    CHECK_MSG(damageOn(c, 2, refOfActor(A100)) == 20, "the hit was lost: damage " + std::to_string(damageOn(c, 2, refOfActor(A100))));
    CHECK(c.cs(2).stats().hitsResolvedLate == 1);
    CHECK(c.cs(2).stats().hitsAbandoned == 0);
    CHECK(callsOn(c, 0) == 0 && callsOn(c, 1) == 0);
}

TEST(combat_retry, a_lethal_hit_nacked_in_a_handoff_to_the_attacker_kills_once_and_everyone_agrees) {
    Cluster c(3);
    c.addActor(A100, 1, 3, {800, 0, 0}, 1);                        // the host weakened it to 1 health
    c.at(1).world.recs[cc::Entity::actor(A100)].pending = true;
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 15));
    c.run(60);
    c.setOwner(A100, 2, 4);
    c.run(600);
    for (int i = 0; i < 3; ++i) CHECK_MSG(!alive(c, i, A100), "machine " + std::to_string(i) + " still shows it alive");
    CHECK(c.cs(2).stats().deathsDeclared == 1);
    CHECK(c.cs(0).stats().deathsAccepted == 1 && c.cs(1).stats().deathsAccepted == 1);
}

TEST(combat_death, a_lethal_hit_replays_the_whole_blow_so_a_float_sliver_cannot_keep_the_victim_alive) {
    Cluster c(3);
    c.addActor(A100, 1, 3, {800, 0, 0}, 12);
    c.at(1).world.recs[cc::Entity::actor(A100)].sliver = 0.4f;     // the engine's health is 12.4, the model's 12
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 20));
    c.run(300);
    CHECK_MSG(!alive(c, 1, A100), "a replay of the rounded remainder (12) leaves 0.4 standing");
    CHECK(!alive(c, 0, A100) && !alive(c, 2, A100));
    CHECK(c.at(1).eng.kills.empty());                              // the whole blow was enough: no forced kill
}

TEST(combat_death, an_accepted_lethal_hit_the_engine_did_not_finish_is_finished_by_the_owner) {
    Cluster c(3);
    c.addActor(A100, 1, 3, {800, 0, 0}, 12);
    c.at(1).world.recs[cc::Entity::actor(A100)].sliver = 0.4f;
    c.at(1).eng.refuseLethal = true;                               // armour, a clamp inside the engine: the replay can never take the last 0.4
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 20));
    c.run(100);
    CHECK(alive(c, 1, A100));                                      // not yet: the owner waits killConfirmMs for the engine's own death
    c.run(800);
    CHECK_MSG(!alive(c, 1, A100), "the owner never killed the victim its own rules called dead");
    CHECK(c.at(1).eng.kills.size() == 1);
    CHECK(c.cs(1).stats().killsForced == 1);
    CHECK(!alive(c, 0, A100) && !alive(c, 2, A100));               // the death reached the others through the owner's declaration
    CHECK(c.cs(1).stats().deathsDeclared == 1);
}

TEST(combat_death, a_victim_that_died_by_the_replay_is_not_killed_twice_and_a_protected_one_is_not_killed_at_all) {
    Cluster c(3);
    c.addActor(A100, 1, 3, {800, 0, 0}, 5);
    c.addActor(A102, 1, 2, {500, 0, 0}, 5);
    c.forAll(A102, [](Rec& r) { r.named = true; });
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 50));
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A102), 50));
    c.run(1500);
    CHECK(!alive(c, 1, A100));
    CHECK(alive(c, 1, A102));
    CHECK(c.at(1).eng.kills.empty());
}

// Lab, 2026-10-27, vats room: the host shot a weakened gecko in VATS; shots 1 to 3 agreed, shot 4 took the health to -14 on the host and the client kept a
// living gecko. The shot goes through the same hit funnel (the gate logged PROCEED, "victim is resolved here"), so the owner's engine applied it; what a
// VATS kill (slow motion, kill cam) does not do is flip the life state, and the actor port declares only a life state death.
TEST(combat_death, a_natural_lethal_hit_that_leaves_the_life_state_alive_is_finished_by_the_owner_and_declared) {
    Cluster c(2);
    c.addActor(A100, 0, 1, {500, 0, 0}, 12);                        // the host owns the gecko and shoots it itself
    c.cs(0).onEngineHit(c.shot(0, refOfActor(A100), 26));
    c.at(0).world.recs[cc::Entity::actor(A100)].health = 0;         // what the engine did: health -14 (shown as 0 here), life state untouched
    c.at(0).world.recs[cc::Entity::actor(A100)].alive = false;
    c.at(0).world.recs[cc::Entity::actor(A100)].lifeLag = true;
    c.run(1000);
    CHECK(c.cs(0).stats().deathsDeclared == 0);                     // not yet: the engine gets lethalSettleMs for its own death
    CHECK(alive(c, 1, A100));
    c.run(1500);
    CHECK_MSG(c.at(0).eng.kills.size() == 1, "the owner never finished the victim of its own lethal hit");
    CHECK(c.cs(0).stats().naturalLethal == 1 && c.cs(0).stats().naturalLethalStuck == 1 && c.cs(0).stats().killsForced == 1);
    CHECK_MSG(!alive(c, 1, A100), "the death never reached the other machine");
    CHECK(c.cs(0).stats().deathsDeclared == 1);
    CHECK(c.cs(1).stats().deathsAccepted == 1);
}

TEST(combat_death, a_natural_lethal_hit_the_engine_finishes_itself_is_not_killed_a_second_time) {
    Cluster c(2);
    c.addActor(A100, 0, 1, {500, 0, 0}, 12);
    c.cs(0).onEngineHit(c.shot(0, refOfActor(A100), 26));
    auto& r = c.at(0).world.recs[cc::Entity::actor(A100)];
    r.health = 0; r.alive = false;                                  // health and life state together, as a normal shot does it
    c.at(0).eng.deaths.push_back(refOfActor(A100));
    c.run(3000);
    CHECK(c.at(0).eng.kills.empty());
    CHECK(c.cs(0).stats().naturalLethalStuck == 0 && c.cs(0).stats().killsForced == 0);
    CHECK(!alive(c, 1, A100));
    CHECK(c.cs(0).stats().deathsDeclared == 1);
}

TEST(combat_death, a_protected_victim_of_a_natural_lethal_hit_is_knocked_down_not_killed) {
    Cluster c(2);
    c.addActor(A100, 0, 1, {500, 0, 0}, 12);
    c.addActor(A101, 0, 1, {500, 100, 0}, 12);
    c.forAll(A100, [](Rec& r) { r.named = true; });
    c.forAll(A101, [](Rec& r) { r.essential = true; });
    for (uint32_t a : {A100, A101}) {
        c.cs(0).onEngineHit(c.shot(0, refOfActor(a), 26));
        auto& r = c.at(0).world.recs[cc::Entity::actor(a)];
        r.health = 0; r.alive = false; r.lifeLag = true;
    }
    c.run(4000);
    CHECK(c.at(0).eng.kills.empty());
    CHECK(c.cs(0).stats().deathsDeclared == 0);
    CHECK(alive(c, 1, A100) && alive(c, 1, A101));
}

TEST(combat_death, a_natural_lethal_victim_that_was_healed_in_the_settle_time_is_left_alone) {
    Cluster c(2);
    c.addActor(A100, 0, 1, {500, 0, 0}, 12);
    c.cs(0).onEngineHit(c.shot(0, refOfActor(A100), 26));
    auto& r = c.at(0).world.recs[cc::Entity::actor(A100)];
    r.health = 0; r.alive = false; r.lifeLag = true;
    c.run(500);
    r.health = 40; r.alive = true; r.lifeLag = false;               // a stimpak, a script
    c.run(3000);
    CHECK(c.at(0).eng.kills.empty());
    CHECK(alive(c, 1, A100));
}

TEST(combat_status, the_last_gate_decision_names_what_happened_to_a_shot_at_a_synced_victim) {
    Cluster c(3);
    c.addActor(A100, 0, 3, {800, 0, 0});
    c.cs(1).onEngineHit(c.shot(1, refOfActor(A100), 20));
    CHECK_MSG(c.cs(1).lastGate().find("clamp") == 0, c.cs(1).lastGate());
    CHECK(c.cs(1).lastGate().find("forwarded") != std::string::npos);
}

// ---------------------------------------------------------------------------------------------------- crime and hostility
TEST(combat_crime, a_witnessed_attack_on_a_faction_turns_it_hostile_for_everyone_exactly_once) {
    Cluster c(3, [](Config& cfg) { cfg.crimeSync = true; });
    c.addActor(A100, 1, 3, {800, 0, 0});
    c.forAll(A100, [](Rec& r) { r.faction = FormRef("falloutnv.esm", 0x777); });
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 10));
    c.run(100);
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 10));
    c.run(100);
    for (int i = 0; i < 3; ++i) CHECK_MSG(c.at(i).eng.hostility.size() == 1, "machine " + std::to_string(i));
    CHECK(c.at(2).eng.hostility[0].kind == cc::HostilityKind::SetEnemy);
    CHECK(c.cs(0).hostility().hostile(FormRef("falloutnv.esm", 0x1BF3E), FormRef("falloutnv.esm", 0x777)));
    // a client cannot dictate hostility to the others
    MHostility fake; fake.e.id = 99; fake.e.kind = cc::HostilityKind::SetAlly; fake.e.factionA = FormRef("a.esm", 1); fake.e.factionB = FormRef("a.esm", 2);
    Envelope e; e.src = 2; e.dest = kNoPlayer; e.body = fake;
    const Bytes b = encodeEnvelope(e);
    c.cs(1).receive(0, b.data(), b.size());
    CHECK(c.at(1).eng.hostility.size() == 2);        // from the host: applied (the host is the authority)
    c.cs(0).receive(2, b.data(), b.size());
    CHECK(c.at(0).eng.hostility.size() == 1);        // the host ignores one that comes from a client
}

}  // namespace
