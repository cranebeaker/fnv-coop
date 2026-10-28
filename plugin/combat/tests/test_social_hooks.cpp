// plugin/combat: the hook points plugin/companions plugs into (Config::social, FULL-SYNC-PLAN 3.4 / 3.8 / 3.10). The hooks here are fakes that
// record what combat asks and reports; the real ones (nvcoop::social::glue*) are wired in combat_module.cpp under NVCOOP_WITH_COMPANIONS.
// Runtime twins 781..788 (combat_sync.cpp) must each break at least one test here:
//   781 the party policy is never asked          782 its answer is ignored (every hit is harmless)     783 combat pays kill XP although the social module does
//   784 every machine reports the same kill     785 the native-payment mask is forgotten               786 no crime burst throttle
//   787 the engine's own reputation is forgotten 788 every machine reports the murder
//   796 the story-actor protection (rules FriendlyFireGuard) is never asked    797 its answer is ignored
#include "harness.h"

using namespace ctest;

namespace {

constexpr uint32_t A100 = 100, A101 = 101;
const FormRef kBandits("falloutnv.esm", 0x777);
const FormRef kPlayerFaction("falloutnv.esm", 0x1BF3E);

struct Spy {
    bool xpHandled = false;
    std::vector<fnvcoop::social::KillReport> kills;
    std::vector<fnvcoop::social::CrimeEvent> crimes;
    std::set<uint32_t> partyVictims;                      // victim refs the fake policy calls "inside the party"
    std::vector<std::pair<uint32_t, uint32_t>> asked;     // (attackerRef, victimRef) the gate asked about
};

// Config tweak that installs the fake hooks of `spies[cfg.self]` (the array outlives the cluster).
std::function<void(Config&)> wire(Spy* spies, bool xpHandled, bool crimeSync = false) {
    return [spies, xpHandled, crimeSync](Config& c) {
        Spy& s = spies[c.self];
        s.xpHandled = xpHandled;
        c.crimeSync = crimeSync;
        c.social.xpHandled = [&s] { return s.xpHandled; };
        c.social.reportKill = [&s](const fnvcoop::social::KillReport& r) { s.kills.push_back(r); };
        c.social.reportCrime = [&s](const fnvcoop::social::CrimeEvent& e) { s.crimes.push_back(e); };
        c.social.neutralizeHit = [&s](uint32_t a, uint32_t v) {
            s.asked.push_back({a, v});
            return s.partyVictims.count(v) != 0;
        };
    };
}

void giveFaction(Cluster& c, uint32_t actor, FormRef f = kBandits) { c.forAll(actor, [f](Rec& r) { r.faction = f; }); }
int callsOn(Cluster& c, int machine) { return static_cast<int>(c.at(machine).eng.damage.size()); }

// ---------------------------------------------------------------------------------------------------- kill reports and kill XP
TEST(combat_social_kill, a_kill_is_reported_once_by_its_declarer_and_combat_does_not_pay_the_xp_a_second_time) {
    Spy spy[3];
    Cluster c(3, wire(spy, true));
    c.copies = [](const Datagram&) { return 2; };           // every datagram twice
    c.addActor(A100, 1, 3, {800, 0, 0}, 50);
    giveFaction(c, A100);
    for (auto& m : c.m) m->eng.xpValue = 75;
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 60));
    c.run(300);
    CHECK(c.cs(1).stats().deathsDeclared == 1);
    CHECK(c.cs(0).stats().deathsAccepted == 1);
    CHECK(c.cs(2).stats().deathsAccepted == 1);
    REQUIRE(spy[1].kills.size() == 1);                       // the victim's owner declared it: the only reporter
    CHECK(spy[0].kills.empty());
    CHECK(spy[2].kills.empty());
    const fnvcoop::social::KillReport& r = spy[1].kills[0];
    CHECK(r.deathId != 0);
    CHECK(r.deathId == c.cs(2).deaths().record(cc::Entity::actor(A100))->deathId);
    CHECK(r.victim.who == cc::Entity::actor(A100));
    CHECK(r.victim.xp == 75);
    CHECK(r.victim.faction == kBandits);
    CHECK(r.killer == cc::Entity::player(2));
    CHECK(r.nativeMask == 0);
    CHECK(r.cell == 1);
    for (int i = 0; i < 3; ++i) CHECK_MSG(c.at(i).eng.xp.empty(), "machine " + std::to_string(i));   // the social module pays it, combat stays out
    CHECK(c.cs(2).stats().socialXpHandled == 1);
    CHECK(c.cs(2).stats().xpAwards == 0);
}

TEST(combat_social_kill, without_the_social_module_paying_combat_keeps_paying_the_killing_blow_and_the_report_still_goes_out) {
    Spy spy[3];
    Cluster c(3, wire(spy, false));
    c.addActor(A100, 1, 3, {800, 0, 0}, 50);
    for (auto& m : c.m) m->eng.xpValue = 75;
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 60));
    c.run(300);
    REQUIRE(c.at(2).eng.xp.size() == 1);
    CHECK(c.at(2).eng.xp[0] == 75);
    CHECK(c.at(0).eng.xp.empty());
    CHECK(c.at(1).eng.xp.empty());
    CHECK(spy[1].kills.size() == 1);
    CHECK(c.cs(2).stats().socialXpHandled == 0);
}

TEST(combat_social_kill, a_native_kill_by_the_local_player_tells_the_social_module_its_engine_already_paid) {
    Spy spy[2];
    Cluster c(2, wire(spy, true));
    c.addActor(A100, 1, 3, {500, 0, 0}, 50);
    for (auto& m : c.m) m->eng.xpValue = 60;
    CHECK(c.cs(1).onEngineHit(c.shot(1, refOfActor(A100), 80)).action == GateAction::Proceed);   // machine 1 owns the actor AND shoots it
    Rec& r = c.at(1).world.recs[cc::Entity::actor(A100)];
    r.health = 0; r.alive = false;
    c.cs(1).onActorDied(refOfActor(A100), true, false, true, 0);
    c.run(100);
    REQUIRE(spy[1].kills.size() == 1);
    CHECK(spy[0].kills.empty());
    CHECK(spy[1].kills[0].nativeMask == (1u << 1));          // the social module must not pay slot 1 again
    CHECK(spy[1].kills[0].killer == cc::Entity::player(1));
    CHECK(c.at(1).eng.xp.empty());
    CHECK(c.at(0).eng.xp.empty());
}

TEST(combat_social_kill, a_companions_kill_names_its_recruiter_and_an_environment_death_still_reports_with_no_killer) {
    Spy spy[2];
    Cluster c(2, wire(spy, true));
    c.addActor(A100, 1, 3, {500, 0, 0}, 50);
    c.addActor(A101, 1, 3, {520, 0, 0}, 50);
    c.forAll(A101, [](Rec& r) { r.recruiter = 0; });          // A101 follows player 0
    EngineHit h;
    h.attackerRef = refOfActor(A101); h.victimRef = refOfActor(A100); h.healthDmg = 80; h.weapon = rifle(); h.weaponRange = 3000;
    c.cs(1).onEngineHit(h);                                   // the companion (simulated on machine 1) shoots the victim (also machine 1's)
    Rec& r = c.at(1).world.recs[cc::Entity::actor(A100)];
    r.health = 0; r.alive = false;
    c.cs(1).onActorDied(refOfActor(A100), true, false, true, 0);
    c.run(100);
    REQUIRE(spy[1].kills.size() == 1);
    CHECK(spy[1].kills[0].killer == cc::Entity::actor(A101));
    CHECK(spy[1].kills[0].killerRecruiter == 0);
    CHECK(spy[1].kills[0].nativeMask == 0);                   // the engine credits the companion, not a player
    // a fall: no hit, no killer
    c.at(1).world.recs[cc::Entity::actor(A101)].alive = false;
    c.cs(1).onActorDied(refOfActor(A101), true, false, true, 0);
    c.run(100);
    REQUIRE(spy[1].kills.size() == 2);
    CHECK(spy[1].kills[1].killer.none());
}

// ---------------------------------------------------------------------------------------------------- the party hit policy
TEST(combat_social_hit, a_hit_inside_the_party_is_neutralized_before_anything_is_sent_or_applied) {
    Spy spy[2];
    Cluster c(2, wire(spy, false));
    c.addActor(A100, 1, 3, {300, 0, 0});                      // somebody's companion, owned (simulated) by machine 1
    c.addActor(A101, 1, 3, {320, 0, 0});                      // an ordinary actor
    spy[0].partyVictims.insert(refOfActor(A100));
    spy[1].partyVictims.insert(refOfActor(A100));
    // the host's player shoots the copy: Neutralize, nothing goes out
    const GateDecision g = c.cs(0).onEngineHit(c.shot(0, refOfActor(A100), 20));
    CHECK(g.action == GateAction::Neutralize);
    c.run(100);
    CHECK(c.cs(0).stats().hitsSent == 0);
    CHECK(c.cs(0).stats().socialHitsNeutralized == 1);
    CHECK(callsOn(c, 1) == 0);
    // the owner's own player shoots its companion: the native path would apply it, the gate says no
    CHECK(c.cs(1).onEngineHit(c.shot(1, refOfActor(A100), 20)).action == GateAction::Neutralize);
    CHECK(c.cs(1).stats().naturalHits == 0);
    // an ordinary actor is untouched by the policy
    CHECK(c.cs(0).onEngineHit(c.shot(0, refOfActor(A101), 20)).action == GateAction::Clamp);
    c.run(100);
    CHECK(callsOn(c, 1) == 1);
    CHECK(c.cs(0).stats().socialHitsNeutralized == 1);
    REQUIRE(!spy[0].asked.empty());
    CHECK(spy[0].asked[0].first == kLocalPlayerRef);
    CHECK(spy[0].asked[0].second == refOfActor(A100));
}

TEST(combat_social_hit, a_blast_of_a_party_member_does_not_hurt_the_party) {
    Spy spy[3];
    Cluster c(3, wire(spy, false));
    c.addActor(A100, 1, 3, {1000, 0, 0});
    c.addActor(A101, 2, 4, {1000, 200, 0});                   // inside the radius, never iterated by the local engine: the planner hits it
    spy[0].partyVictims.insert(refOfActor(A101));
    EngineHit h;
    h.attackerRef = kLocalPlayerRef; h.victimRef = refOfActor(A100); h.healthDmg = 80; h.weapon = grenade(); h.explosionKey = 7;
    h.blastCenter = {1000, 100, 0}; h.blastRadius = 500; h.weaponRange = 3000;
    c.cs(0).onEngineHit(h);
    c.run(400);
    CHECK(callsOn(c, 1) == 1);                                // the stranger is hurt
    CHECK(callsOn(c, 2) == 0);                                // the companion is not
    CHECK(c.cs(0).stats().socialHitsNeutralized == 1);
}

// ---------------------------------------------------------------------------------------------------- crime reports
TEST(combat_social_crime, an_attack_is_reported_once_per_burst_by_the_machine_that_simulates_the_attacker) {
    Spy spy[3];
    Cluster c(3, wire(spy, false));
    c.addActor(A100, 1, 3, {800, 0, 0}, 5000);
    giveFaction(c, A100);
    for (int i = 0; i < 10; ++i) c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 5));   // an automatic weapon
    c.run(200);
    REQUIRE(spy[2].crimes.size() == 1);
    CHECK(spy[0].crimes.empty());
    CHECK(spy[1].crimes.empty());                              // the victim's owner applies the hit but cannot vouch for who fired
    const fnvcoop::social::CrimeEvent& e = spy[2].crimes[0];
    CHECK(e.kind == fnvcoop::social::CrimeKind::Assault);
    CHECK(e.criminal == 2);
    CHECK(!e.byCompanion);
    CHECK(e.victim == cc::Entity::actor(A100));
    CHECK(e.faction == kBandits);
    CHECK(e.cell == 1);
    CHECK(!e.victimDead);
    CHECK(!e.victimHostile);
    CHECK(e.nativeMask == 0);                                  // the victim's owner resolves it: no engine paid anything on machine 2
    CHECK(c.cs(2).stats().socialCrimesReported == 1);
    c.run(3500);                                               // a new burst after the throttle window is a new report
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 5));
    c.run(100);
    CHECK(spy[2].crimes.size() == 2);
}

TEST(combat_social_crime, an_attack_on_something_already_hostile_says_so_and_a_native_attack_says_the_engine_handled_it) {
    Spy spy[3];
    Cluster c(3, wire(spy, false, true));                      // crimeSync on too: combat's own hostility path books the faction as an enemy
    c.addActor(A100, 1, 3, {800, 0, 0}, 5000);
    c.addActor(A101, 2, 3, {900, 0, 0}, 5000);
    giveFaction(c, A100);
    giveFaction(c, A101, FormRef("falloutnv.esm", 0x778));
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 5));
    c.run(100);
    REQUIRE(spy[2].crimes.size() == 1);
    CHECK(!spy[2].crimes[0].victimHostile);
    CHECK(c.cs(2).hostility().hostile(kPlayerFaction, kBandits));
    c.run(3500);
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 5));       // the faction is an enemy by now (the host's SetEnemy reached everybody)
    c.run(100);
    REQUIRE(spy[2].crimes.size() == 2);
    CHECK(spy[2].crimes[1].victimHostile);
    // machine 2's player hits an actor machine 2 itself simulates: its own engine already applied the consequences
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A101), 5));
    c.run(100);
    REQUIRE(spy[2].crimes.size() == 3);
    CHECK(spy[2].crimes[2].victim == cc::Entity::actor(A101));
    CHECK(spy[2].crimes[2].nativeMask == (1u << 2));
}

TEST(combat_social_crime, a_companions_attack_is_charged_to_its_recruiter_and_an_attack_by_a_stranger_actor_is_not_a_player_crime) {
    Spy spy[2];
    Cluster c(2, wire(spy, false));
    c.addActor(A100, 1, 3, {500, 0, 0}, 5000);
    c.addActor(A101, 1, 3, {520, 0, 0}, 5000);
    giveFaction(c, A100);
    c.forAll(A101, [](Rec& r) { r.recruiter = 0; });
    EngineHit h;
    h.attackerRef = refOfActor(A101); h.victimRef = refOfActor(A100); h.healthDmg = 5; h.weapon = rifle(); h.weaponRange = 3000;
    c.cs(1).onEngineHit(h);
    REQUIRE(spy[1].crimes.size() == 1);
    CHECK(spy[1].crimes[0].criminal == 0);
    CHECK(spy[1].crimes[0].byCompanion);
    c.forAll(A101, [](Rec& r) { r.recruiter = kNoPlayer; });
    c.run(3500);
    c.cs(1).onEngineHit(h);                                    // an NPC fighting an NPC
    CHECK(spy[1].crimes.size() == 1);
}

TEST(combat_social_crime, a_murder_is_reported_by_the_killers_machine_only) {
    Spy spy[3];
    Cluster c(3, wire(spy, false));
    c.addActor(A100, 1, 3, {800, 0, 0}, 50);
    giveFaction(c, A100);
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 60));
    c.run(300);
    int murders2 = 0;
    for (const auto& e : spy[2].crimes) if (e.kind == fnvcoop::social::CrimeKind::Murder) { ++murders2; CHECK(e.victimDead); CHECK(e.criminal == 2); }
    CHECK(murders2 == 1);
    for (int i = 0; i < 2; ++i)
        for (const auto& e : spy[i].crimes) CHECK_MSG(e.kind != fnvcoop::social::CrimeKind::Murder, "machine " + std::to_string(i));
}

TEST(combat_social_crime, a_native_murder_by_the_local_player_carries_the_native_mask) {
    Spy spy[2];
    Cluster c(2, wire(spy, false));
    c.addActor(A100, 1, 3, {500, 0, 0}, 50);
    giveFaction(c, A100);
    CHECK(c.cs(1).onEngineHit(c.shot(1, refOfActor(A100), 80)).action == GateAction::Proceed);
    Rec& r = c.at(1).world.recs[cc::Entity::actor(A100)];
    r.health = 0; r.alive = false;
    c.cs(1).onActorDied(refOfActor(A100), true, false, true, 0);
    c.run(100);
    bool found = false;
    for (const auto& e : spy[1].crimes)
        if (e.kind == fnvcoop::social::CrimeKind::Murder) { found = true; CHECK(e.nativeMask == (1u << 1)); }
    CHECK(found);
}

}  // namespace

// ---------------------------------------------------------------------------------------------------- G40: protected story actors
TEST(combat_social_hit, an_accidental_player_hit_on_a_protected_story_actor_is_dropped_on_the_attackers_gate_and_at_the_owner) {
    Cluster c(3, [](Config& cfg) {
        // machine 0 is the host and holds the rules: it protects actor 100 against every hit by a player; the others hold none
        if (cfg.self == 0) cfg.social.blockProtectedHit = [](uint32_t victimRef, PlayerId, bool) { return victimRef == refOfActor(A100); };
    });
    c.addActor(A100, 1, 3, {800, 0, 0}, 50);                  // the story actor, simulated (owned) by machine 1
    c.addActor(A101, 1, 3, {820, 0, 0}, 50);
    // the host's own player shoots it: dropped at the attacker's own gate, nothing is sent
    CHECK(c.cs(0).onEngineHit(c.shot(0, refOfActor(A100), 20)).action == GateAction::Neutralize);
    c.run(100);
    CHECK(c.cs(0).stats().hitsSent == 0 && c.cs(0).stats().protectedHitsBlocked == 1);
    CHECK(callsOn(c, 1) == 0);
    // an ordinary actor is not protected
    CHECK(c.cs(0).onEngineHit(c.shot(0, refOfActor(A101), 20)).action == GateAction::Clamp);
}

TEST(combat_social_hit, a_remote_players_accepted_hit_on_a_protected_actor_is_not_applied_by_its_owner) {
    Cluster c(3, [](Config& cfg) {
        if (cfg.self == 1) cfg.social.blockProtectedHit = [](uint32_t victimRef, PlayerId, bool) { return victimRef == refOfActor(A100); };   // the OWNER holds the rules
    });
    c.addActor(A100, 1, 3, {800, 0, 0}, 50);
    c.addActor(A101, 1, 3, {820, 0, 0}, 50);
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A100), 20));      // guest 2 shoots the protected actor: forwarded to its owner, which refuses it
    c.cs(2).onEngineHit(c.shot(2, refOfActor(A101), 20));      // and an ordinary one: applied
    c.run(300);
    CHECK(c.cs(1).stats().protectedHitsBlocked == 1);
    CHECK(callsOn(c, 1) == 1);                                 // only the ordinary actor took damage
}
