// plugin/threat tests, part 1: the scoring (threat_model.cpp). Twins 1001..1013 must each make one of these fail.
#include <cmath>

#include "minitest.h"
#include "rig.h"

using namespace nvth;

namespace {

// Plain arithmetic: only damage counts, so a score IS the faded damage.
Config plain() {
    Config c;
    c.nearBonus = 0;
    c.seenBonus = 0;
    c.engagedBonus = 0;
    c.spreadPenalty = 10;   // (the arithmetic of these tests, not the default)
    return c;
}

Candidate cand(Slot s, float distance = 500, bool engaged = false, int others = 0, bool seen = false) {
    Candidate c;
    c.slot = s;
    c.distance = distance;
    c.engaged = engaged;
    c.others = others;
    c.seen = seen;
    return c;
}

constexpr uint64_t kNow = 100000;

}  // namespace

TEST(threat_model, damage_fades_linearly_and_is_gone_after_the_fade_time) {
    Config c = plain();
    Record r;
    addDamage(r, 1, 40, kNow, c);
    CHECK(std::fabs(damageOf(r, 1, kNow, c) - 40) < 0.01f);
    CHECK(std::fabs(damageOf(r, 1, kNow + c.fadeMs / 2, c) - 20) < 0.01f);   // twin 1001: no fade
    CHECK(damageOf(r, 1, kNow + c.fadeMs, c) == 0);
    CHECK(damageOf(r, 1, kNow + c.fadeMs + 5000, c) == 0);
    CHECK(damageOf(r, 2, kNow, c) == 0);
}

TEST(threat_model, blows_add_up_after_fading_the_older_ones) {
    Config c = plain();
    Record r;
    addDamage(r, 1, 40, kNow, c);
    addDamage(r, 1, 10, kNow + c.fadeMs / 2, c);                              // the 40 is worth 20 by now
    CHECK(std::fabs(damageOf(r, 1, kNow + c.fadeMs / 2, c) - 30) < 0.01f);   // twin 1002: a new blow replaces the old ones (10)
    addDamage(r, 1, 0, kNow, c);
    addDamage(r, 1, -5, kNow, c);
    addDamage(r, kNoSlot, 50, kNow, c);
    CHECK(r.hurts.size() == 1);
}

TEST(threat_model, a_player_who_hurt_it_much_more_wins_the_pick) {
    Config c = plain();
    Record r;
    r.since = 0;
    addDamage(r, 1, 100, kNow, c);
    std::vector<Candidate> cs{cand(0, 500, true), cand(1)};
    const Decision d = decide(r, cs, kNow, c);
    CHECK(d.verdict == Verdict::Switch);
    CHECK(d.to == 1);
}

TEST(threat_model, hysteresis_a_small_lead_does_not_move_it_a_clear_one_does) {
    Config c = plain();
    Record r;
    r.since = 0;
    addDamage(r, 0, 20, kNow, c);
    addDamage(r, 1, 28, kNow, c);                                             // needs more than 20 * 1.25 + 4 = 29
    std::vector<Candidate> cs{cand(0, 500, true), cand(1)};
    CHECK(decide(r, cs, kNow, c).verdict == Verdict::Keep);                   // twin 1004: any lead moves it
    Record r2;
    r2.since = 0;
    addDamage(r2, 0, 20, kNow, c);
    addDamage(r2, 1, 30, kNow, c);
    std::vector<Candidate> cs2{cand(0, 500, true), cand(1)};
    const Decision d = decide(r2, cs2, kNow, c);
    CHECK(d.verdict == Verdict::Switch);
    CHECK(d.to == 1);
}

TEST(threat_model, the_minimum_time_on_a_target_holds_it_even_against_a_clear_lead) {
    Config c = plain();
    Record r;
    r.since = kNow - 1000;
    addDamage(r, 1, 200, kNow, c);
    std::vector<Candidate> cs{cand(0, 500, true), cand(1)};
    CHECK(decide(r, cs, kNow, c).verdict == Verdict::Hold);                   // twin 1005: no minimum time
    r.since = kNow - c.minOnTargetMs;
    std::vector<Candidate> cs2{cand(0, 500, true), cand(1)};
    CHECK(decide(r, cs2, kNow, c).verdict == Verdict::Switch);
}

TEST(threat_model, a_target_it_does_not_fight_among_the_candidates_is_replaced_at_once) {
    Config c = plain();
    Record r;
    r.since = kNow;                                                           // just changed: the minimum time does not apply to a target that is gone
    std::vector<Candidate> cs{cand(1, 300), cand(2, 900)};
    const Decision d = decide(r, cs, kNow, c);
    CHECK(d.verdict == Verdict::Fresh);
    c.nearBonus = 15;
    std::vector<Candidate> cs2{cand(1, 300), cand(2, 900)};
    CHECK(decide(r, cs2, kNow, c).to == 1);                                   // the nearer one
    std::vector<Candidate> none;
    CHECK(decide(r, none, kNow, c).verdict == Verdict::None);
}

TEST(threat_model, others_already_on_a_player_count_against_that_player) {
    Config c = plain();
    Record r;
    r.since = 0;
    addDamage(r, 0, 30, kNow, c);
    addDamage(r, 1, 30, kNow, c);
    std::vector<Candidate> cs{cand(0, 500, true, 3), cand(1, 500, false, 0)};   // three others on slot 0, none on slot 1: 0 against 30
    const Decision d = decide(r, cs, kNow, c);
    CHECK(d.verdict == Verdict::Switch);                                      // twin 1006: no spreading
    CHECK(d.to == 1);
    Record r2;
    r2.since = 0;
    addDamage(r2, 0, 30, kNow, c);
    addDamage(r2, 1, 28, kNow, c);
    std::vector<Candidate> few{cand(0, 500, true, 1), cand(1, 500, false, 0)};  // one other: 20 against 28 is not clear (needs more than 20 * 1.25 + 4 = 29)
    CHECK(decide(r2, few, kNow, c).verdict == Verdict::Keep);
}

TEST(threat_model, nearness_counts_less_the_further_a_player_is) {
    Config c;
    c.seenBonus = 0;
    c.engagedBonus = 0;
    Record r;
    CHECK(std::fabs(baseScore(r, cand(1, 0), kNow, c) - c.nearBonus) < 0.01f);
    const float mid = baseScore(r, cand(1, c.nearRange / 2), kNow, c);
    CHECK(std::fabs(mid - c.nearBonus / 2) < 0.01f);                          // twin 1007: flat inside the range
    CHECK(baseScore(r, cand(1, c.nearRange), kNow, c) == 0);
    CHECK(baseScore(r, cand(1, c.nearRange * 3), kNow, c) == 0);
}

TEST(threat_model, sight_and_the_current_fight_add_their_bonuses) {
    Config c;
    c.nearBonus = 0;
    Record r;
    CHECK(baseScore(r, cand(1, 900, false, 0, true), kNow, c) == c.seenBonus);       // twin 1011
    CHECK(baseScore(r, cand(1, 900, true, 0, false), kNow, c) == c.engagedBonus);    // twin 1012
    CHECK(baseScore(r, cand(1, 900, true, 0, true), kNow, c) == c.seenBonus + c.engagedBonus);
}

TEST(threat_model, a_tie_goes_to_the_one_it_fights_then_to_the_lower_slot) {
    Config c = plain();
    Record r;
    r.since = 0;
    std::vector<Candidate> a{cand(2), cand(1)};
    CHECK(decide(r, a, kNow, c).to == 1);                                     // Fresh, equal: the lower slot
    std::vector<Candidate> b{cand(0), cand(1, 500, true)};
    CHECK(decide(r, b, kNow, c).verdict == Verdict::Keep);                    // it stays with the one it fights
}

TEST(threat_model, only_a_fight_against_a_player_is_ever_touched) {
    Config c;
    std::vector<PlayerView> ps(2);
    ps[0].slot = 0; ps[0].ref = kHostRef;
    ps[1].slot = 1; ps[1].ref = puppetRef(1);
    Slot f = 7;
    EnemyView e;
    e.ref = 0x100; e.mine = true; e.inCombat = true; e.combatTarget = puppetRef(1);
    CHECK(eligibility(e, c, ps, f) == Skip::None);
    CHECK(f == 1);
    EnemyView calm = e; calm.inCombat = false; calm.combatTarget = 0;
    CHECK(eligibility(calm, c, ps, f) == Skip::Calm);
    CHECK(f == kNoSlot);
    EnemyView other = e; other.combatTarget = 0x0A0A0A;                       // a companion, or another faction
    CHECK(eligibility(other, c, ps, f) == Skip::NotAboutPlayers);             // twin 1009
    EnemyView comp = e; comp.teammate = true;
    CHECK(eligibility(comp, c, ps, f) == Skip::Companion);                    // twin 1008
    EnemyView named = e; named.unique = true;
    CHECK(eligibility(named, c, ps, f) == Skip::Story);                       // twin 1010
    c.includeNamed = true;
    CHECK(eligibility(named, c, ps, f) == Skip::None);
    EnemyView theirs = e; theirs.mine = false;
    CHECK(eligibility(theirs, c, ps, f) == Skip::NotMine);
    EnemyView clone = e; clone.standIn = true;
    CHECK(eligibility(clone, c, ps, f) == Skip::NotMine);
    EnemyView dead = e; dead.dead = true;
    CHECK(eligibility(dead, c, ps, f) == Skip::Gone);
}

TEST(threat_model, aim_is_measured_from_the_facing_with_yaw_zero_pointing_north) {
    CHECK(std::fabs(aimCosine({0, 0, 0}, 0.0f, {0, 100, 0}) - 1.0f) < 1e-4f);          // +Y
    CHECK(std::fabs(aimCosine({0, 0, 0}, 1.5707963f, {100, 0, 0}) - 1.0f) < 1e-4f);    // a quarter turn clockwise: +X
    CHECK(std::fabs(aimCosine({0, 0, 0}, 0.0f, {100, 0, 0})) < 1e-4f);
    CHECK(aimCosine({0, 0, 0}, 0.0f, {0, -100, 0}) < -0.99f);
    CHECK(aimCosine({5, 5, 0}, 0.3f, {5, 5, 9}) == 1.0f);                              // straight above: no direction, counts as aimed
}

TEST(threat_model, a_health_drop_is_split_between_those_aiming_at_it) {
    Config c;
    Shooter a{0, 800, 0.99f, true}, b{1, 800, 0.95f, true}, lowered{2, 800, 0.99f, false}, wide{3, 800, 0.5f, true}, far{4, 9000, 0.99f, true};
    const auto parts = splitDamage(30, {a, b, lowered, wide, far}, c);
    CHECK(parts.size() == 2);                                                 // twin 1013: weapon-out and the cone are not asked
    float total = 0;
    for (const auto& p : parts) total += p.second;
    CHECK(std::fabs(total - 30) < 0.01f);
    CHECK(parts.size() == 2 && parts[0].first == 0 && parts[0].second > parts[1].second);   // the better aim gets more
    CHECK(splitDamage(30, {lowered, wide, far}, c).empty());
    CHECK(splitDamage(0, {a}, c).empty());
    CHECK(splitDamage(-4, {a}, c).empty());
}
