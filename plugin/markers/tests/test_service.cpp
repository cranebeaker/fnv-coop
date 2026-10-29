// plugin/markers service tests (game-free): whole machines (a host and up to three guests) over an in-memory order channel with latency, a fake rules
// engine on the host. Twins 1401..1419 of mk_service.cpp must each fail this file.
#include <string>

#include "minitest.h"
#include "mk_rig.h"

using namespace nvmk;
using namespace mk_test;

namespace {

bool has(const std::string& s, const char* part) { return s.find(part) != std::string::npos; }
bool anyNote(const Machine& m, const char* part) {
    for (const auto& n : m.notes) if (has(n, part)) return true;
    return false;
}
const HudCompassMark* compassOf(const Machine& m, Slot s) {
    for (const auto& c : m.frame.compass) if (c.slot == s) return &c;
    return nullptr;
}
const HudTag* tagOf(const Machine& m, Slot s) {
    for (const auto& t : m.frame.tags) if (t.slot == s) return &t;
    return nullptr;
}

// a host and two guests standing close together, the table has gone round
void settle(Rig& r) { r.step(1500); }

// ---------------------------------------------------------------------------------------------------------------------------------- markers
TEST(service, everybody_sees_everybody_after_a_second) {
    Rig r(3);
    r.m[0]->local.at = outdoors(0, 0);
    r.m[1]->local.at = outdoors(700, 0);
    r.m[2]->local.at = outdoors(0, 1400);
    settle(r);
    for (int i = 0; i < 3; ++i) {
        CHECK(r.m[i]->svc->board().members().size() == 2);
        CHECK(r.m[i]->frame.tags.size() == 2);
        CHECK(r.m[i]->frame.compass.size() == 2);
        CHECK(r.m[i]->frame.map.size() == 2);
        CHECK(r.m[i]->frame.list.size() == 2);
    }
    // the guest sees the other guest by the name the other guest gave
    CHECK(tagOf(*r.m[1], 2) && tagOf(*r.m[1], 2)->text.find("Bo") == 0);
    CHECK(tagOf(*r.m[2], 1) && tagOf(*r.m[2], 1)->text.find("Alex") == 0);
    CHECK(tagOf(*r.m[1], 0) && tagOf(*r.m[1], 0)->text.find("Host") == 0);
    // Alex (700 east of the host) is on the host's right, Bo (1400 north) straight ahead
    const HudCompassMark* a = compassOf(*r.m[0], 1);
    const HudCompassMark* b = compassOf(*r.m[0], 2);
    REQUIRE(a && b);
    CHECK(a->x > 0.99f);
    CHECK(b->edge == 0 && b->x > -0.01f && b->x < 0.01f);
}

TEST(service, a_far_friend_follows_on_the_compass_and_is_named_by_sector) {
    Rig r(2);
    r.m[0]->local.at = outdoors(0, 0);
    r.m[1]->local.at = outdoors(0, 30000);
    settle(r);
    const HudCompassMark* c = compassOf(*r.m[0], 1);
    REQUIRE(c);
    CHECK(c->edge == 0);
    CHECK(!tagOf(*r.m[0], 1));   // beyond the tag range
    REQUIRE(r.m[0]->frame.list.size() == 1);
    CHECK(has(r.m[0]->frame.list[0], "Mojave Wasteland, sector 0,7"));
    // he walks round to the west: the marker moves along the strip as the table catches up
    r.m[1]->local.at = outdoors(-30000, -30000);
    settle(r);
    c = compassOf(*r.m[0], 1);
    REQUIRE(c);
    CHECK(c->edge == -1);
    // and the host turns to face him (south-west)
    r.m[0]->local.heading = -3.0f * kPi / 4.0f;
    r.step(100);
    c = compassOf(*r.m[0], 1);
    REQUIRE(c);
    CHECK(c->edge == 0 && c->x > -0.01f && c->x < 0.01f);
}

TEST(service, a_friend_who_went_inside_is_where_he_went_in) {
    Rig r(2);
    r.m[0]->local.at = outdoors(0, 0);
    r.m[1]->local.at = indoors(0x500, 10, 10, "Vault 21");
    r.m[1]->local.ext = outdoors(2000, 2000);
    settle(r);
    const HudCompassMark* c = compassOf(*r.m[0], 1);
    REQUIRE(c);
    CHECK(c->x > 0.4f && c->x < 0.6f);
    REQUIRE(r.m[0]->frame.map.size() == 1);
    CHECK(r.m[0]->frame.map[0].world && !r.m[0]->frame.map[0].local);
    CHECK(has(r.m[0]->frame.list[0], "inside Vault 21, near sector 0,0"));
    // from inside, the host does not see him on the compass; the other way round the host is where the guest last stood outdoors
    r.m[0]->local.at = indoors(0x500, 20, 20, "Vault 21");
    r.m[0]->local.ext = outdoors(2100, 2100);
    settle(r);
    const HudTag* t = tagOf(*r.m[0], 1);
    REQUIRE(t);   // same interior: in view
    CHECK(t->alpha > 0.99f);
}

TEST(service, a_guest_that_leaves_disappears_everywhere) {
    Rig r(3);
    settle(r);
    CHECK(r.m[0]->svc->board().members().size() == 2);
    r.left.insert(2);
    r.step(3000);
    CHECK(r.m[0]->svc->board().members().size() == 1);
    CHECK(r.m[1]->svc->board().members().size() == 1);
    CHECK(!tagOf(*r.m[1], 2));
}

TEST(service, a_guest_that_goes_silent_drops_out_of_the_table) {
    Rig r(3);
    settle(r);
    CHECK(r.m[1]->svc->board().find(2) != nullptr);
    r.frozen.insert(2);
    r.step(12000);
    CHECK(r.m[1]->svc->board().find(2) == nullptr);
    CHECK(!tagOf(*r.m[1], 2));
    CHECK(!tagOf(*r.m[0], 2));
}

TEST(service, the_downed_timer_counts_down_before_the_first_table_arrives) {
    Rig r(2);
    settle(r);
    r.m[1]->svc->onLocalDowned(60000, r.now);
    CHECK(r.m[1]->svc->localWindowLeftMs(r.now) == 60000);
    CHECK(r.m[1]->svc->localWindowLeftMs(r.now + 15000) == 45000);
    CHECK(r.m[1]->svc->localWindowLeftMs(r.now + 61000) == 0);
    r.m[1]->svc->onLocalDowned(0, r.now);
    CHECK(r.m[1]->svc->localWindowLeftMs(r.now) == 0);
}

TEST(service, an_older_table_that_arrives_late_is_ignored) {
    Rig r(2);
    settle(r);
    PartyTable newer, older;
    newer.seq = 1000;
    older.seq = 999;
    Member h;
    h.slot = 0;
    h.name = "Host";
    h.at = outdoors(111, 0);
    newer.members = {h};
    h.at = outdoors(222, 0);
    older.members = {h};
    r.m[1]->svc->onGuestOrder(encodeParty(newer), r.now);
    r.m[1]->svc->onGuestOrder(encodeParty(older), r.now);
    const Member* got = r.m[1]->svc->board().find(0);
    REQUIRE(got);
    CHECK(got->at.x == 111);
}

TEST(service, a_beacon_only_counts_from_a_joined_guest) {
    Rig r(2);
    settle(r);
    Beacon b;
    b.name = "Mallory";
    b.at = outdoors(5, 5);
    const std::string json = encodeBeacon(b);
    CHECK(r.m[0]->svc->onHostOrder(0, json, r.now) != "Ok");   // the host's own slot
    CHECK(r.m[0]->svc->onHostOrder(5, json, r.now) != "Ok");   // nobody joined there
    CHECK(r.m[0]->svc->onHostOrder(1, "garbage", r.now) == "BadOrder");
    CHECK(r.m[0]->svc->onHostOrder(1, json, r.now) == "Ok");   // slot 1 is joined: it may rename itself
    settle(r);
    CHECK(r.m[0]->svc->board().find(5) == nullptr);
}

TEST(service, the_markers_off_switch_hides_every_marker_but_the_revive_still_works) {
    ServiceConfig c;
    c.markers.enabled = false;
    Rig r(2, c);
    r.m[0]->local.at = outdoors(0, 0);
    r.m[1]->local.at = outdoors(60, 0);
    settle(r);
    CHECK(r.m[0]->frame.tags.empty() && r.m[0]->frame.compass.empty() && r.m[0]->frame.map.empty() && r.m[0]->frame.list.empty());
    r.kill(1);
    r.step(600);
    r.m[0]->local.activateHeld = true;
    r.step(3600);
    CHECK(r.revivedLog.size() == 1);
}

TEST(service, with_both_features_off_nothing_is_sent_or_drawn) {
    ServiceConfig c;
    c.markers.enabled = false;
    c.revive.enabled = false;
    Rig r(2, c);
    r.step(2000);
    CHECK(r.sent == 0);
    CHECK(r.m[0]->frame.banner.empty());
}

TEST(service, session_end_clears_the_board_and_the_hud) {
    Rig r(2);
    settle(r);
    CHECK(!r.m[1]->frame.tags.empty());
    r.m[1]->svc->onSessionEnd();
    CHECK(r.m[1]->svc->board().members().empty());
    CHECK(r.m[1]->frame.tags.empty());
}

// ---------------------------------------------------------------------------------------------------------------------------------- revive
TEST(revive_flow, the_host_gets_a_downed_guest_up_by_holding_the_key) {
    Rig r(2);
    r.m[0]->local.at = outdoors(100, 0);
    r.m[1]->local.at = outdoors(0, 0);
    settle(r);
    r.kill(1);
    r.step(600);
    // the host sees him down with a timer and is offered the revive
    REQUIRE(r.m[0]->svc->board().find(1) != nullptr);
    CHECK(r.m[0]->svc->board().find(1)->downed);
    CHECK(has(r.m[0]->frame.banner, "Hold the activate key to get Alex up"));
    CHECK(has(r.m[0]->frame.banner, "1:00") || has(r.m[0]->frame.banner, "0:59"));
    CHECK(has(r.m[0]->frame.list[0], "(down)"));
    // the downed guest sees his timer
    CHECK(r.m[1]->frame.local);
    CHECK(has(r.m[1]->frame.banner, "You are down"));
    CHECK(has(r.m[1]->frame.banner, "1:00") || has(r.m[1]->frame.banner, "0:59"));
    // holding
    r.m[0]->local.activateHeld = true;
    r.step(1500);
    CHECK(has(r.m[0]->frame.banner, "Reviving Alex"));
    CHECK(r.m[0]->frame.bannerProgress > 0.3f && r.m[0]->frame.bannerProgress < 0.7f);
    CHECK(has(r.m[1]->frame.banner, "Host is helping"));
    CHECK(r.revivedLog.empty());
    r.step(2200);
    REQUIRE(r.revivedLog.size() == 1);
    CHECK(r.revivedLog[0].slot == 1);
    CHECK(r.revivedLog[0].frac > 0.29f && r.revivedLog[0].frac < 0.31f);
    CHECK(r.revivedLog[0].by == "Host");
    CHECK(!r.m[1]->svc->localDowned());
    CHECK(anyNote(*r.m[1], "Host got you back on your feet"));
    CHECK(anyNote(*r.m[0], "back on their feet"));
    CHECK(r.respawnedLog.empty());   // and the policy respawn never fires
    r.step(80000);
    CHECK(r.respawnedLog.empty());
}

TEST(revive_flow, a_guest_gets_a_downed_host_and_another_guest_up) {
    Rig r(3);
    r.m[0]->local.at = outdoors(0, 0);
    r.m[1]->local.at = outdoors(60, 0);
    r.m[2]->local.at = outdoors(30000, 0);   // far away: no part in it
    settle(r);
    r.kill(0);
    r.step(900);
    CHECK(has(r.m[1]->frame.banner, "Hold the activate key to get Host up"));
    CHECK(r.m[2]->frame.banner.empty());
    r.m[1]->local.activateHeld = true;
    r.step(4200);
    REQUIRE(r.revivedLog.size() == 1);
    CHECK(r.revivedLog[0].slot == 0);
    CHECK(r.revivedLog[0].by == "Alex");
    CHECK(!r.m[0]->svc->localDowned());
    CHECK(anyNote(*r.m[1], "back on their feet"));   // the host told the friend directly (it is the host's own reviver note here)
    // now guest 2 comes over and gets guest 1 up, through the host
    r.m[1]->local.activateHeld = false;
    r.m[2]->local.at = outdoors(120, 0);
    r.kill(1);
    r.step(900);
    r.m[2]->local.activateHeld = true;
    r.step(4400);
    REQUIRE(r.revivedLog.size() == 2);
    CHECK(r.revivedLog[1].slot == 1);
    CHECK(r.revivedLog[1].by == "Bo");
    CHECK(!r.m[1]->svc->localDowned());
    CHECK(anyNote(*r.m[2], "back on their feet"));
}

TEST(revive_flow, letting_go_stops_it_and_nothing_happens) {
    Rig r(2);
    r.m[0]->local.at = outdoors(100, 0);
    r.m[1]->local.at = outdoors(0, 0);
    settle(r);
    r.kill(1);
    r.step(600);
    r.m[0]->local.activateHeld = true;
    r.step(2000);
    r.m[0]->local.activateHeld = false;
    r.step(2500);
    CHECK(r.revivedLog.empty());
    CHECK(r.m[0]->svc->authority().reviverOf(1) == kNoSlot);
    CHECK(r.m[0]->svc->stats().holdsCancelled == 1);
    // and it can be started over
    r.m[0]->local.activateHeld = true;
    r.step(3700);
    CHECK(r.revivedLog.size() == 1);
}

TEST(revive_flow, the_host_judges_a_guests_claim_from_its_own_view) {
    Rig r(3);
    r.m[0]->local.at = outdoors(0, 0);
    r.m[1]->local.at = outdoors(0, 0);
    r.m[2]->local.at = outdoors(5000, 0);   // 70 m from the downed guest
    settle(r);
    r.kill(1);
    r.step(600);
    // a modified guest sends begin and done without being near (its own reach check is not the host's)
    Machine& liar = *r.m[2];
    r.send(2, 0, encodeRevive({RevivePhase::Begin, 1}));
    r.step(3600);
    r.send(2, 0, encodeRevive({RevivePhase::Done, 1}));
    r.step(300);
    CHECK(r.revivedLog.empty());
    CHECK(r.m[0]->svc->authority().isDowned(1));
    CHECK(r.m[0]->svc->lastVerdict() != Verdict::Ok);
    (void)liar;
}

TEST(revive_flow, a_claim_that_skips_the_hold_time_is_refused) {
    Rig r(3);
    r.m[0]->local.at = outdoors(0, 0);
    r.m[1]->local.at = outdoors(0, 0);
    r.m[2]->local.at = outdoors(50, 0);
    settle(r);
    r.kill(1);
    r.step(600);
    r.send(2, 0, encodeRevive({RevivePhase::Begin, 1}));
    r.step(200);
    r.send(2, 0, encodeRevive({RevivePhase::Done, 1}));
    r.step(300);
    CHECK(r.revivedLog.empty());
    CHECK(r.m[0]->svc->stats().revivesRefused >= 1);
    // a done from somebody who never began
    r.send(0, 0, encodeRevive({RevivePhase::Done, 1}));
    r.step(300);
    CHECK(r.revivedLog.empty());
}

TEST(revive_flow, nobody_comes_the_policy_respawn_happens_as_before) {
    Rig r(2);
    r.m[0]->local.at = outdoors(5000, 0);
    r.m[1]->local.at = outdoors(0, 0);
    settle(r);
    r.kill(1);
    r.step(30000);
    CHECK(r.m[1]->svc->localDowned());
    CHECK_MSG(has(r.m[1]->frame.banner, "0:3") || has(r.m[1]->frame.banner, "0:2"), r.m[1]->frame.banner);   // the timer has run down to the half
    CHECK(r.respawnedLog.empty());
    r.step(31000);
    REQUIRE(r.respawnedLog.size() == 1);
    CHECK(r.respawnedLog[0] == 1);
    CHECK(!r.m[1]->svc->localDowned());
    CHECK(r.m[1]->frame.banner.empty());
    CHECK(!r.m[0]->svc->authority().isDowned(1));
    CHECK(r.revivedLog.empty());
}

TEST(revive_flow, a_revive_after_the_window_is_refused) {
    Rig r(2);
    r.m[0]->local.at = outdoors(50, 0);
    r.m[1]->local.at = outdoors(0, 0);
    r.windowMs = 5000;
    settle(r);
    r.kill(1);
    r.step(6000);   // the fake rules respawned him at 5 s
    CHECK(r.respawnedLog.size() == 1);
    r.m[0]->local.activateHeld = true;
    r.step(4000);
    CHECK(r.revivedLog.empty());
}

TEST(revive_flow, a_collar_death_waits_for_the_checkpoint_not_for_a_friend) {
    Rig r(2);
    r.m[0]->local.at = outdoors(50, 0);
    r.m[1]->local.at = outdoors(0, 0);
    settle(r);
    r.kill(1, true);
    r.step(600);
    CHECK(!has(r.m[0]->frame.banner, "Hold the activate key"));
    CHECK(has(r.m[1]->frame.banner, "checkpoint"));
    r.m[0]->local.activateHeld = true;
    r.step(4000);
    CHECK(r.revivedLog.empty());
    CHECK(r.m[1]->svc->localDowned());
}

TEST(revive_flow, with_the_respawn_policy_there_is_no_window_and_no_prompt) {
    Rig r(2);
    r.modeRespawn = true;
    r.m[0]->local.at = outdoors(50, 0);
    r.m[1]->local.at = outdoors(0, 0);
    settle(r);
    r.kill(1);
    r.step(200);
    CHECK(!has(r.m[0]->frame.banner, "Hold the activate key"));
}

TEST(revive_flow, the_revive_switch_turns_the_hold_off_and_leaves_the_markers) {
    ServiceConfig c;
    c.revive.enabled = false;
    Rig r(2, c);
    r.m[0]->local.at = outdoors(50, 0);
    r.m[1]->local.at = outdoors(0, 0);
    settle(r);
    r.kill(1);
    r.step(600);
    r.m[0]->local.activateHeld = true;
    r.step(4000);
    CHECK(r.revivedLog.empty());
    CHECK(r.m[0]->svc->stats().holdsBegun == 0);
    CHECK(r.m[0]->frame.banner.empty());
    CHECK(r.m[0]->frame.tags.size() == 1);
    // a modified guest asking anyway is refused by the host
    r.send(1, 0, encodeRevive({RevivePhase::Begin, 1}));
    r.step(200);
    CHECK(r.m[0]->svc->lastVerdict() == Verdict::Disabled);
}

TEST(revive_flow, one_friend_at_a_time_and_a_downed_friend_cannot_help) {
    Rig r(4);
    r.m[0]->local.at = outdoors(50, 0);
    r.m[1]->local.at = outdoors(0, 0);
    r.m[2]->local.at = outdoors(0, 50);
    r.m[3]->local.at = outdoors(0, 90);
    settle(r);
    r.kill(1);
    r.kill(3);   // 3 is down and cannot help
    r.step(900);
    r.m[0]->local.activateHeld = true;
    r.m[2]->local.activateHeld = true;
    r.m[3]->local.activateHeld = true;
    r.step(500);
    // the first to begin on the host holds him; the other sees the board say he is busy and stands down
    const Slot who = r.m[0]->svc->authority().reviverOf(1);
    CHECK(who == 0 || who == 2);
    r.step(5000);
    CHECK(r.revivedLog.size() >= 1);
    for (const auto& rv : r.revivedLog) CHECK(rv.by != "Cy");
    CHECK(r.m[3]->svc->stats().holdsBegun == 0);
}

TEST(revive_flow, the_hosts_places_fall_back_to_beacons_when_no_puppet_is_loaded) {
    Rig r(3);
    for (auto& m : r.m) m->autoPuppets = false;
    r.m[0]->local.at = outdoors(0, 0);
    r.m[1]->local.at = outdoors(0, 0);
    r.m[2]->local.at = outdoors(60, 0);
    settle(r);
    r.kill(1);
    r.step(900);
    r.m[2]->local.activateHeld = true;
    r.step(4500);
    REQUIRE(r.revivedLog.size() == 1);
    CHECK(r.revivedLog[0].slot == 1);
}

TEST(revive_flow, a_friend_in_another_place_is_not_offered) {
    Rig r(2);
    r.m[0]->local.at = indoors(0x777, 0, 0);
    r.m[1]->local.at = outdoors(0, 0);
    settle(r);
    r.kill(1);
    r.step(900);
    CHECK(!has(r.m[0]->frame.banner, "Hold the activate key"));
    r.m[0]->local.activateHeld = true;
    r.step(4000);
    CHECK(r.revivedLog.empty());
}

TEST(revive_flow, a_menu_or_a_dialogue_blocks_the_hold) {
    Rig r(2);
    r.m[0]->local.at = outdoors(50, 0);
    r.m[1]->local.at = outdoors(0, 0);
    settle(r);
    r.kill(1);
    r.step(600);
    r.m[0]->local.activateHeld = true;
    r.m[0]->local.uiBlocked = true;
    r.step(4000);
    CHECK(r.revivedLog.empty());
    CHECK(r.m[0]->frame.tags.empty());   // and no tags over a menu
}

TEST(revive_flow, the_status_json_has_the_counters) {
    Rig r(2);
    r.m[0]->local.at = outdoors(50, 0);
    r.m[1]->local.at = outdoors(0, 0);
    settle(r);
    const std::string s = r.m[0]->svc->statusJson(r.now);
    CHECK(has(s, "\"members\":1"));
    CHECK(has(s, "\"tablesSent\":"));
    CHECK(has(s, "\"lastVerdict\":\"ok\""));
}

}  // namespace
