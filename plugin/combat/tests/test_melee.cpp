// plugin/combat: a guest's blow on a host-owned NPC copy (2026-10-29 playtest: the guest punched a non-hostile Goodsprings settler; on the guest it is a restrained
// copy driven from snapshots, and nothing happened, with no combat line in the guest's log). Every blow a combatant simulated here lands on a synced actor copy has to reach
// the owner as a hit request, hostile victim or not, armed or not, whatever its size: the owner applies it natively. Twins 801..807, 809..812 (combat_sync.cpp, combat_wire.cpp).
#include "harness.h"

using namespace ctest;

namespace {

constexpr uint32_t SETTLER = 100;

EngineHit fist(uint32_t victimRef, float dmg) {
    EngineHit h;
    h.attackerRef = kLocalPlayerRef; h.victimRef = victimRef; h.healthDmg = dmg;
    h.hitLocation = 0;   // no weapon form (fists), no reach, no impact point, no projectile: all the engine gave
    return h;
}
int health(Cluster& c, int machine, uint32_t actor) { return c.at(machine).world.recs[cc::Entity::actor(actor)].health; }
int damageOn(Cluster& c, int machine, uint32_t victimRef) {
    int n = 0;
    for (auto& d : c.at(machine).eng.damage) if (d.ref == victimRef) n += d.amount;
    return n;
}

// host (machine 0) owns the settler, the guest (machine 1) shows a copy of it
void settlerRoom(Cluster& c) { c.addActor(SETTLER, 0, 3, {150, 0, 0}); }

TEST(combat_melee, a_guests_punch_on_the_hosts_settler_reaches_the_owner_and_lands_for_both) {
    Cluster c(2);
    settlerRoom(c);
    const uint32_t ref = refOfActor(SETTLER);
    const GateDecision g = c.cs(1).onEngineHit(fist(ref, 3.0f));
    CHECK(g.action == GateAction::Clamp);                  // the copy plays the reaction, the owner decides the damage
    c.run(100);
    CHECK(c.cs(1).stats().hitsSent == 1);
    CHECK(c.cs(0).stats().hitsApplied == 1);
    CHECK(damageOn(c, 0, ref) == 3);                       // applied once, by the owner
    CHECK(c.at(1).eng.damage.empty());                     // never by the attacker's machine
    CHECK(health(c, 0, SETTLER) == 97);
    CHECK(health(c, 1, SETTLER) == 97);                    // the copy follows the owner's vitals
    REQUIRE(c.at(0).eng.provokes.size() == 1);             // the settler turns on the guest's puppet
    CHECK(c.at(0).eng.provokes[0].first == ref);
    CHECK(c.at(0).eng.provokes[0].second == refOfPlayer(0, 1));
    REQUIRE(c.at(0).eng.damage.size() == 1);
    CHECK(c.at(0).eng.damage[0].attackerRef == refOfPlayer(0, 1));   // the replay's source is the puppet: the blow is credited to the guest
    CHECK(c.cs(1).lastGate().find("forwarded") != std::string::npos);
}

bool near(float a, float b) { return a > b - 0.011f && a < b + 0.011f; }   // the wire carries hundredths

TEST(combat_melee, a_light_blow_arrives_exactly_as_the_engine_computed_it) {
    // the engine's blows are fractions: a first-level fist on a settler is a few tenths. It used to round to 0, be neutralised at the gate and leave no trace anywhere; it
    // must not round UP either (a 0.5 fist landing as 1 doubles unarmed damage): the owner applies exactly the amount, natively, off the engine's float health.
    Cluster c(2);
    settlerRoom(c);
    const uint32_t ref = refOfActor(SETTLER);
    for (float dmg : {0.5f, 0.2f, 0.25f, 0.05f}) {
        const GateDecision g = c.cs(1).onEngineHit(fist(ref, dmg));
        CHECK_MSG(g.action == GateAction::Clamp, std::to_string(dmg));
        c.run(60);
    }
    CHECK(c.cs(1).stats().hitsSent == 4);
    CHECK(c.cs(1).stats().neutralized == 0);
    CHECK(c.cs(1).stats().lightHitsSent == 4);
    REQUIRE(c.at(0).eng.touches.size() == 4);
    CHECK(near(c.at(0).eng.touches[0].amount, 0.5f));
    CHECK(near(c.at(0).eng.touches[1].amount, 0.2f));
    CHECK(near(c.at(0).eng.touches[2].amount, 0.25f));
    CHECK(near(c.at(0).eng.touches[3].amount, 0.05f));
    CHECK(c.at(0).eng.touches[0].attacker == refOfPlayer(0, 1));   // credited to the guest's puppet
    CHECK(c.at(0).eng.damage.empty());                      // never as a whole point
    CHECK(c.cs(0).stats().lightHitsApplied == 4);
    CHECK(c.cs(0).stats().touchesApplied == 0);             // they hurt, however little: not touches
    CHECK(c.cs(0).stats().hitsApplied == 0);
    CHECK(c.at(0).eng.provokes.size() == 4);
    // 1.0 points in all (float sums: 99.0 or a hair under): the copy follows the owner's published vitals, never a whole point per blow
    CHECK(health(c, 0, SETTLER) >= 98 && health(c, 0, SETTLER) <= 99);
    CHECK(health(c, 1, SETTLER) == health(c, 0, SETTLER));
}

TEST(combat_melee, two_half_point_fists_take_exactly_one_point) {
    Cluster c(2);
    settlerRoom(c);
    for (int i = 0; i < 2; ++i) {
        c.cs(1).onEngineHit(fist(refOfActor(SETTLER), 0.5f));
        c.run(60);
    }
    CHECK(health(c, 0, SETTLER) == 99);
    CHECK(health(c, 1, SETTLER) == 99);
    CHECK(c.cs(0).stats().vitalsSent >= 1);
}

TEST(combat_melee, a_fractional_blow_of_a_point_or_more_keeps_its_fraction) {
    Cluster c(2);
    settlerRoom(c);
    c.cs(1).onEngineHit(fist(refOfActor(SETTLER), 12.4f));
    c.run(100);
    REQUIRE(c.at(0).eng.exactDamage.size() == 1);
    CHECK(near(c.at(0).eng.exactDamage[0], 12.4f));
    REQUIRE(c.at(0).eng.damage.size() == 1);
    CHECK(c.at(0).eng.damage[0].amount == 12);            // the rules' whole points are unchanged
    CHECK(c.cs(0).stats().hitsApplied == 1);
}

TEST(combat_melee, a_run_of_punches_lands_each_once_in_order) {
    Cluster c(2);
    settlerRoom(c);
    const uint32_t ref = refOfActor(SETTLER);
    for (int i = 0; i < 6; ++i) {
        CHECK(c.cs(1).onEngineHit(fist(ref, 2.0f)).action == GateAction::Clamp);
        c.run(50);
    }
    CHECK(c.cs(0).stats().hitsApplied == 6);
    CHECK(damageOn(c, 0, ref) == 12);
    CHECK(health(c, 0, SETTLER) == 88);
    CHECK(health(c, 1, SETTLER) == 88);
    CHECK(c.at(0).eng.provokes.size() == 6);
    CHECK(c.cs(0).stats().hitDuplicates == 0);
}

TEST(combat_melee, a_blow_with_no_health_damage_reaches_the_owner_as_a_touch) {
    // armour took it all: the engine's hit has health damage 0, but the victim still reels and turns on its attacker. The owner must hear of it, and take nothing off.
    Cluster c(2);
    settlerRoom(c);
    const uint32_t ref = refOfActor(SETTLER);
    const GateDecision g = c.cs(1).onEngineHit(fist(ref, 0.0f));
    CHECK(g.action == GateAction::Clamp);
    CHECK(g.maxHealthDamage == 99);
    CHECK(c.cs(1).lastGate().find("touch") != std::string::npos);
    c.run(100);
    CHECK(c.cs(1).stats().touchesSent == 1);
    CHECK(c.cs(0).stats().touchesApplied == 1);
    CHECK(c.cs(0).stats().touchesEngine == 1);
    REQUIRE(c.at(0).eng.touches.size() == 1);              // the owner's engine replays the blow for its reaction
    CHECK(c.at(0).eng.touches[0].victim == ref);
    CHECK(c.at(0).eng.touches[0].attacker == refOfPlayer(0, 1));
    CHECK(c.at(0).eng.touches[0].amount == 0.0f);
    CHECK(c.at(0).eng.damage.empty());                     // ... and no health leaves the settler
    CHECK(health(c, 0, SETTLER) == 100);
    CHECK(health(c, 1, SETTLER) == 100);
    CHECK(c.cs(0).stats().hitsApplied == 0);
    REQUIRE(c.at(0).eng.provokes.size() == 1);
    CHECK(c.at(0).eng.provokes[0].second == refOfPlayer(0, 1));
}

TEST(combat_melee, only_a_real_blow_on_an_actor_is_a_touch) {
    Cluster c(2);
    settlerRoom(c);
    const uint32_t ref = refOfActor(SETTLER);
    EngineHit env = fist(ref, 0.0f);
    env.attackerRef = 0;                                   // nobody: a trap, a fall
    CHECK(c.cs(1).onEngineHit(env).action == GateAction::Neutralize);
    EngineHit blast = fist(ref, 0.0f);
    blast.explosionKey = 77; blast.blastCenter = {150, 0, 0};   // an explosion's zero-damage pass is not a blow
    CHECK(c.cs(1).onEngineHit(blast).action == GateAction::Neutralize);
    EngineHit heal = fist(ref, -5.0f);
    CHECK(c.cs(1).onEngineHit(heal).action == GateAction::Neutralize);
    c.run(100);
    CHECK(c.cs(1).stats().hitsSent == 0);
    CHECK(c.cs(1).stats().touchesSent == 0);
    CHECK(c.at(0).eng.touches.empty());
}

TEST(combat_melee, a_touch_on_a_player_is_not_sent_it_is_only_an_actors_reaction) {
    Cluster c(3);
    c.addActor(SETTLER, 1, 3, {150, 0, 0});                // an NPC owned by machine 1 hits the player on machine 2 for nothing
    EngineHit h;
    h.attackerRef = refOfActor(SETTLER); h.victimRef = refOfPlayer(1, 2); h.healthDmg = 0;
    CHECK(c.cs(1).onEngineHit(h).action == GateAction::Neutralize);
    c.run(100);
    CHECK(c.sentTotal == 0);
}

TEST(combat_melee, a_touch_takes_no_health_off_the_settler) {
    // a touch travels as damage 1 so the rules accept it; an owner that did not honour the flag would take a point off
    Cluster c(2);
    settlerRoom(c);
    c.cs(1).onEngineHit(fist(refOfActor(SETTLER), 0.0f));
    c.run(100);
    CHECK(health(c, 0, SETTLER) == 100);
    CHECK(c.at(0).eng.damage.empty());
    CHECK(c.cs(0).stats().damageApplied == 0);
}

TEST(combat_melee, a_hit_on_a_stand_in_copy_and_a_hostile_or_friendly_victim_travel_the_same_way) {
    Cluster c(2);
    settlerRoom(c);
    c.at(1).world.recs[cc::Entity::actor(SETTLER)].standIn = true;   // the guest shows a stand-in clone of the host's settler
    c.at(1).world.recs[cc::Entity::actor(SETTLER)].named = true;      // a story NPC
    c.at(0).world.recs[cc::Entity::actor(SETTLER)].named = true;
    c.at(1).world.recs[cc::Entity::actor(SETTLER)].faction = FormRef("falloutnv.esm", 0x777);
    CHECK(c.cs(1).onEngineHit(fist(refOfActor(SETTLER), 1.0f)).action == GateAction::Clamp);
    c.run(100);
    CHECK(damageOn(c, 0, refOfActor(SETTLER)) == 1);
    CHECK(c.cs(1).stats().neutralized == 0);
}

TEST(combat_melee, a_punch_during_a_handoff_is_sent_again_when_the_new_owner_is_known) {
    Cluster c(3);
    c.addActor(SETTLER, 1, 3, {150, 0, 0});
    c.at(1).world.recs[cc::Entity::actor(SETTLER)].pending = true;    // machine 1 is handing the settler over
    c.cs(2).onEngineHit(fist(refOfActor(SETTLER), 0.0f));              // a touch, the lightest blow there is
    c.run(60);
    CHECK(c.at(1).eng.touches.empty());
    CHECK(c.cs(1).stats().nacksSent == 1);
    c.setOwner(SETTLER, 0, 4);
    c.run(400);
    CHECK(c.at(0).eng.touches.size() == 1);                            // still a touch after the retry: no point of damage appears
    CHECK(c.at(0).eng.damage.empty());
    CHECK(c.cs(2).stats().hitRetries == 1);
}

TEST(combat_melee, a_punch_on_an_actor_the_guest_owns_itself_is_native) {
    Cluster c(2);
    c.addActor(SETTLER, 1, 3, {150, 0, 0});
    const GateDecision g = c.cs(1).onEngineHit(fist(refOfActor(SETTLER), 0.3f));
    CHECK(g.action == GateAction::Proceed);                // the engine applies it as it computed it, fractions and all
    CHECK(c.cs(1).stats().lightHitsSent == 0);
    c.run(100);
    CHECK(c.sentTotal <= 1);                               // only the coalesced vitals, no hit request
    CHECK(c.cs(1).stats().hitsSent == 0);
}

}  // namespace
