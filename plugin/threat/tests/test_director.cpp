// plugin/threat tests, part 2: the director over a fake world (threat_director.cpp). Twins 1020..1031 must each make one of these fail.
#include <cmath>
#include <set>
#include <string>

#include "minitest.h"
#include "rig.h"

using namespace nvth;

namespace {

constexpr RefId kE1 = 0x100;

// Two players, one enemy between them fighting the host: the guest is nearer.
void duel(Rig& r) {
    r.w.addPlayer(0, 0, 0);
    r.w.addPlayer(1, 300, 0);
    r.w.addEnemy(kE1, 500, 0, kHostRef);
}

int onSlot(const Rig& r, Slot s) {
    int n = 0;
    for (const EnemyView& e : r.w.es)
        if (e.combatTarget == (s == 0 ? kHostRef : puppetRef(s))) ++n;
    return n;
}

}  // namespace

TEST(threat_director, the_enemy_moves_to_the_player_who_hurt_it_and_asks_the_engine_once) {
    Rig r;
    duel(r);
    r.tick();                                                    // host 10 + 6, guest 13: it stays
    CHECK(r.w.retargets.empty());
    CHECK(r.d.targetOf(kE1) == 0);
    r.d.noteHit(kE1, 1, 40, r.now);
    r.tick();
    REQUIRE(r.w.retargets.size() == 1);
    CHECK(r.w.retargets[0].first == kE1 && r.w.retargets[0].second == puppetRef(1));
    CHECK(r.d.targetOf(kE1) == 1);
    CHECK(r.d.stats().switches == 1);
    r.ticks(4);
    CHECK(r.w.retargets.size() == 1);                           // and it stays there
}

TEST(threat_director, a_player_that_only_stands_a_little_closer_does_not_move_it) {
    Rig r;
    duel(r);
    r.w.player(1).pos = {450, 0, 0};                            // 50 units from the enemy, the host 500
    r.ticks(6);
    CHECK(r.w.retargets.empty());                               // 14.5 against 10 + 6
}

TEST(threat_director, it_does_not_flicker_when_two_players_trade_blows) {
    Rig r;
    duel(r);
    r.tick();
    r.d.noteHit(kE1, 1, 40, r.now);
    r.tick();
    CHECK(r.d.stats().switches == 1);
    for (int i = 0; i < 24; ++i) {                              // twelve seconds of both hitting it about equally
        r.d.noteHit(kE1, i % 2 == 0 ? 0 : 1, 12, r.now);
        r.tick();
    }
    CHECK(r.d.stats().switches == 1);                           // twin 1004 / 1005 live in the model suite; here the whole loop must stay quiet
    CHECK(r.w.retargets.size() == 1);
}

TEST(threat_director, a_target_is_kept_for_the_minimum_time_then_given_up_for_a_clear_lead) {
    Rig r;
    duel(r);
    r.tick();
    r.d.noteHit(kE1, 1, 40, r.now);
    r.tick();                                                   // on the guest since now
    const uint64_t at = r.now - 500;
    r.d.noteHit(kE1, 0, 400, r.now);                            // the host hurts it terribly
    while (r.now < at + 3000) {
        r.tick();
        CHECK(r.w.retargets.size() == 1);                       // twin 1005 is in the model suite; the director has to carry the verdict through
    }
    CHECK(r.d.stats().held > 0);
    r.tick();
    r.tick();
    REQUIRE(r.w.retargets.size() == 2);
    CHECK(r.w.retargets[1].second == kHostRef);
}

TEST(threat_director, four_enemies_on_one_player_are_spread_over_two) {
    Rig r;
    r.w.addPlayer(0, 0, 0);
    r.w.addPlayer(1, 0, 200);
    for (int i = 0; i < 4; ++i) r.w.addEnemy(0x101 + i, 500.0f + 10.0f * static_cast<float>(i), 100, kHostRef);
    r.ticks(6);
    CHECK(onSlot(r, 0) == 2);                                   // twin 1006 / 1027: they all stay on the host
    CHECK(onSlot(r, 1) == 2);
    CHECK(r.d.stats().switches <= 3);                           // two moved (a third would be thrashing)
}

TEST(threat_director, six_enemies_over_three_players_are_two_each) {
    Rig r;
    r.w.addPlayer(0, 500, 0);
    r.w.addPlayer(1, -500, 0);
    r.w.addPlayer(2, 0, 500);
    for (int i = 0; i < 6; ++i) r.w.addEnemy(0x101 + i, 3.0f * static_cast<float>(i), 0, kHostRef);
    r.ticks(8);
    CHECK(onSlot(r, 0) == 2);
    CHECK(onSlot(r, 1) == 2);
    CHECK(onSlot(r, 2) == 2);
}

TEST(threat_director, enemies_far_from_each_other_are_not_one_group) {
    Rig r;
    r.w.addPlayer(0, 0, 0);
    r.w.addPlayer(1, 0, 200);
    r.w.addEnemy(0x101, 500, 100, kHostRef);
    r.w.addEnemy(0x102, 9000, 100, kHostRef);                   // another fight a long way off
    r.ticks(6);
    CHECK(onSlot(r, 0) == 2);                                   // a lone enemy has nobody to spread from
}

TEST(threat_director, a_downed_player_is_never_picked_and_a_downed_target_is_left_at_once) {
    Rig r;
    duel(r);
    r.w.player(1).downed = true;
    r.tick();
    r.d.noteHit(kE1, 1, 500, r.now);
    r.ticks(3);
    CHECK(r.w.retargets.empty());                               // twin 1020: the downed guest is picked for the damage it did
    r.w.player(1).downed = false;
    r.w.player(0).downed = true;                                // now the host it fights is down
    r.tick();
    REQUIRE(r.w.retargets.size() == 1);
    CHECK(r.w.retargets[0].second == puppetRef(1));
    CHECK(r.d.stats().fresh == 1);
}

TEST(threat_director, a_player_in_another_space_is_not_a_target) {
    Rig r;
    r.w.addPlayer(0, 0, 0, 1);
    r.w.addPlayer(1, 300, 0, 2);                                // inside a building: coordinates mean something else
    r.w.addEnemy(kE1, 500, 0, kHostRef, 1);
    r.tick();
    r.d.noteHit(kE1, 1, 500, r.now);
    r.ticks(3);
    CHECK(r.w.retargets.empty());                               // twin 1021
}

TEST(threat_director, a_player_too_far_away_is_not_worth_walking_to) {
    Rig r;
    r.w.addPlayer(0, 0, 0);
    r.w.addPlayer(1, 9000, 0);
    r.w.addEnemy(kE1, 500, 0, kHostRef);
    r.tick();
    r.d.noteHit(kE1, 1, 500, r.now);
    r.ticks(3);
    CHECK(r.w.retargets.empty());                               // twin 1022
    // but the player it already fights stays a candidate however far (a sniper)
    r.w.enemy(kE1).combatTarget = puppetRef(1);
    r.ticks(3);
    CHECK(r.w.retargets.empty());
}

TEST(threat_director, at_most_the_budget_of_enemies_is_retargeted_a_tick) {
    Config c;
    c.maxSwitches = 1;
    Rig r(c);
    r.w.addPlayer(0, 0, 0);
    r.w.addPlayer(1, 0, 200);
    for (int i = 0; i < 4; ++i) r.w.addEnemy(0x101 + i, 500.0f + 10.0f * static_cast<float>(i), 100, kHostRef);
    r.tick();
    CHECK(r.w.retargets.size() == 1);                           // twin 1023: all that want to go go at once
    r.ticks(6);
    CHECK(onSlot(r, 1) == 2);
}

TEST(threat_director, a_retarget_the_engine_refuses_is_not_counted_and_tried_again_later) {
    Rig r;
    duel(r);
    r.tick();
    r.d.noteHit(kE1, 1, 60, r.now);
    r.w.retargetOk = false;
    r.tick();
    CHECK(r.w.retargets.size() == 1);
    CHECK(r.d.stats().failed == 1 && r.d.stats().switches == 0);   // twin 1024: counted and remembered as done
    CHECK(r.d.targetOf(kE1) == 0);
    r.ticks(4);
    CHECK(r.w.retargets.size() == 1);                           // not every tick
    r.w.retargetOk = true;
    r.d.noteHit(kE1, 1, 60, r.now);
    r.ticks(4);
    CHECK(r.w.retargets.size() == 2);
    CHECK(r.d.targetOf(kE1) == 1);
    CHECK(r.d.stats().switches == 1);
}

TEST(threat_director, an_engine_that_keeps_going_back_is_left_alone_for_a_while) {
    Rig r;
    duel(r);
    r.w.engineOverrules = true;                                 // the fake engine accepts the retarget and keeps its AI on the host
    r.tick();
    for (int i = 0; i < 40; ++i) {                              // twenty seconds
        r.d.noteHit(kE1, 1, 60, r.now);
        r.tick();
    }
    CHECK(r.d.stats().quieted == 1);                            // twin 1025: it is never noticed, so it is retargeted over and over
    CHECK(r.d.stats().overrides == 3);
    CHECK(r.w.retargets.size() == 3);
}

TEST(threat_director, damage_is_inferred_from_a_health_drop_for_the_players_aiming_and_not_twice_with_a_feed) {
    Rig r;
    duel(r);
    r.w.enemy(kE1).pos = {0, 500, 0};
    r.w.player(0).pos = {0, 0, 0};
    r.w.player(0).yaw = 0;                                      // the host faces +Y: at the enemy
    r.w.player(0).weaponOut = true;
    r.w.player(1).pos = {600, 500, 0};
    r.w.player(1).weaponOut = false;
    r.w.hp[kE1] = 100;
    r.tick();
    r.w.hp[kE1] = 70;
    r.tick();
    const Record* rec = r.d.record(kE1);
    REQUIRE(rec != nullptr);
    CHECK(std::fabs(damageOf(*rec, 0, r.now, r.cfg) - 30) < 3.0f);
    CHECK(damageOf(*rec, 1, r.now, r.cfg) == 0);
    CHECK(r.d.stats().damageInferred == 30);

    Rig f;                                                      // the same drop with a feed that reported the blow
    duel(f);
    f.w.enemy(kE1).pos = {0, 500, 0};
    f.w.player(0).weaponOut = true;
    f.w.hp[kE1] = 100;
    f.tick();
    f.d.noteHit(kE1, 0, 30, f.now);
    f.w.hp[kE1] = 70;
    f.tick();
    const Record* fr = f.d.record(kE1);
    REQUIRE(fr != nullptr);
    CHECK(damageOf(*fr, 0, f.now, f.cfg) < 31);                 // twin 1026: 60
    CHECK(f.d.stats().damageInferred == 0);
}

TEST(threat_director, companions_story_npcs_calm_actors_and_other_fights_are_never_touched) {
    Rig r;
    r.w.addPlayer(0, 0, 0);
    r.w.addPlayer(1, 300, 0);
    r.w.addEnemy(0x101, 500, 0, kHostRef).teammate = true;
    r.w.addEnemy(0x102, 500, 0, kHostRef).unique = true;
    r.w.addEnemy(0x103, 500, 0, 0x0A0A0A);                      // fights somebody else
    r.w.addEnemy(0x104, 500, 0, 0);                             // not fighting at all
    r.w.addEnemy(0x105, 500, 0, kHostRef).mine = false;         // another machine's
    r.w.addEnemy(0x106, 500, 0, kHostRef).dead = true;
    r.tick();
    for (RefId e = 0x101; e <= 0x106; ++e) r.d.noteHit(e, 1, 500, r.now);
    r.ticks(4);
    CHECK(r.w.retargets.empty());                               // twins 1008 / 1009 / 1010 in the model suite; here the director must honour the verdicts
    const Stats& s = r.d.stats();
    CHECK(s.skipped[static_cast<size_t>(Skip::Companion)] > 0);
    CHECK(s.skipped[static_cast<size_t>(Skip::Story)] > 0);
    CHECK(s.skipped[static_cast<size_t>(Skip::NotAboutPlayers)] > 0);
    CHECK(s.skipped[static_cast<size_t>(Skip::Calm)] > 0);
    CHECK(s.skipped[static_cast<size_t>(Skip::NotMine)] > 0);
    CHECK(s.skipped[static_cast<size_t>(Skip::Gone)] > 0);
}

TEST(threat_director, a_fight_that_ended_is_forgotten_after_a_while) {
    Rig r;
    duel(r);
    r.tick();
    CHECK(r.d.tracked() == 1);
    r.w.enemy(kE1).inCombat = false;
    r.w.enemy(kE1).combatTarget = 0;
    r.ticks(10);
    CHECK(r.d.tracked() == 1);                                  // five seconds: it may start again
    r.ticks(40);
    CHECK(r.d.tracked() == 0);                                  // twin 1029
}

TEST(threat_director, a_single_player_has_nobody_to_split_between) {
    Rig r;
    r.w.addPlayer(0, 0, 0);
    r.w.addEnemy(kE1, 500, 0, kHostRef);
    r.tick();
    r.d.noteHit(kE1, 1, 500, r.now);
    r.ticks(3);
    CHECK(r.w.retargets.empty());
    CHECK(r.d.stats().soloTicks == 4);
}

TEST(threat_director, sight_checks_are_limited_per_tick_and_remembered) {
    Config c;
    c.maxSightChecks = 4;
    Rig r(c);
    r.w.addPlayer(0, 0, 0);
    r.w.addPlayer(1, 300, 0);
    for (int i = 0; i < 10; ++i) r.w.addEnemy(0x101 + i, 500, 0, kHostRef);
    r.tick();
    CHECK(r.w.sightCalls == 4);                                 // twin 1031: 20
    r.tick();                                                   // the next four (the first four are good for a second)
    CHECK(r.w.sightCalls == 8);
    r.ticks(3);                                                 // all twenty pairs are checked in time ...
    CHECK(r.w.sightCalls >= 20);
    const int calls = r.w.sightCalls;
    Rig q;                                                      // ... and a fast ticker does not ask more than once a second per pair
    duel(q);
    q.d.tick(1000);
    q.d.tick(1500);
    const int first = q.w.sightCalls;
    q.d.tick(1600);
    q.d.tick(1999);
    CHECK(q.w.sightCalls == first);
    q.d.tick(2000);
    CHECK(q.w.sightCalls == first + 2);                         // a second later it asks again
    (void)calls;
}

TEST(threat_director, sight_decides_when_a_player_the_enemy_can_see_is_much_closer) {
    Rig r;
    r.w.addPlayer(0, -600, 0);
    r.w.addPlayer(1, 420, 0);
    r.w.addEnemy(kE1, 500, 0, kHostRef);                        // host: 4 + 6 = 10, guest: 14.2 (4.2 ahead: not clear), 22.2 when it can see them
    r.w.defaultSight = Sight::Hidden;
    r.ticks(3);
    CHECK(r.w.retargets.empty());
    r.w.sights[{kE1, puppetRef(1)}] = Sight::Seen;
    r.ticks(4);
    REQUIRE(r.w.retargets.size() == 1);
    CHECK(r.w.retargets[0].second == puppetRef(1));
}

TEST(threat_director, off_means_off_and_the_tick_rate_is_honoured) {
    Config off;
    off.enabled = false;
    Rig r(off);
    duel(r);
    r.d.noteHit(kE1, 1, 500, r.now);
    r.ticks(4);
    CHECK(r.w.retargets.empty());
    CHECK(r.d.stats().ticks == 0);
    Rig t;
    duel(t);
    t.d.tick(5000);
    t.d.tick(5100);
    t.d.tick(5499);
    CHECK(t.d.stats().ticks == 1);
    t.d.tick(5500);
    CHECK(t.d.stats().ticks == 2);
}

TEST(threat_director, a_blow_of_nothing_makes_no_record_and_reset_forgets_everything) {
    Rig r;
    duel(r);
    r.d.noteHit(kE1, 1, 0, r.now);
    CHECK(r.d.tracked() == 0);                                  // twin 1030
    r.d.noteHit(kE1, 1, 5, r.now);
    CHECK(r.d.tracked() == 1);
    r.d.reset();
    CHECK(r.d.tracked() == 0);
}

TEST(threat_director, the_status_is_one_json_object_with_the_counters) {
    Rig r;
    duel(r);
    r.tick();
    r.d.noteHit(kE1, 1, 40, r.now);
    r.tick();
    const std::string j = r.d.statusJson();
    CHECK(j.front() == '{' && j.back() == '}');
    CHECK(j.find("\"switches\":1") != std::string::npos);
    CHECK(j.find("\"enabled\":true") != std::string::npos);
    CHECK(!r.d.recent().empty());
}
