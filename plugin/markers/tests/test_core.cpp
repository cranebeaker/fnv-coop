// plugin/markers core tests (game-free): the board, the text, the wire, the revive authority and the hold tracker.
// The FNV_MUT twins 1301..1386 of plugin/markers must each fail this file (plugin\markers\run_tests.cmd).
#include <cmath>
#include <string>

#include "minitest.h"
#include "mk_board.h"
#include "mk_wire.h"
#include "rv_authority.h"
#include "rv_hold.h"
#include "mk_rig.h"

using namespace nvmk;
using mk_test::indoors;
using mk_test::kOtherWorld;
using mk_test::kWorld;
using mk_test::outdoors;

namespace {

Member member(Slot s, const char* name, Place at, uint64_t now = 1000) {
    Member m;
    m.slot = s;
    m.name = name;
    m.at = at;
    (void)now;
    return m;
}
bool approx(float a, float b, float eps = 0.02f) { return std::fabs(a - b) <= eps; }

// one viewer at the origin facing north (+Y), the friend `dist` units away in `dir` degrees clockwise from north
MarkerView one(MarkerBoard& b, Slot self, const Place& viewer, float headingRad, const Place& viewerExt = {}) {
    const auto v = b.views(self, viewer, viewerExt, headingRad, 1000);
    return v.empty() ? MarkerView{} : v.front();
}
Place polar(float distUnits, float degClockwise, uint32_t world = kWorld) {
    const float r = degClockwise * kPi / 180.0f;
    return outdoors(std::sin(r) * distUnits, std::cos(r) * distUnits, 0, world);
}

// ------------------------------------------------------------------------------------------------------------------------------ name tag
TEST(board, tag_is_full_near_fades_in_between_and_is_gone_past_the_range) {
    MarkerBoard b;
    const Place me = outdoors(0, 0);
    b.set(member(1, "Alex", polar(700, 0)), 1000);
    MarkerView v = one(b, 0, me, 0);
    REQUIRE(v.tag);
    CHECK(approx(v.tagAlpha, 1.0f));
    CHECK(v.tagText == "Alex");
    b.set(member(1, "Alex", polar(4200, 0)), 1000);   // halfway between 1400 and 7000
    v = one(b, 0, me, 0);
    REQUIRE(v.tag);
    CHECK(approx(v.tagAlpha, 0.5f));
    CHECK(v.tagText.find("60 m") != std::string::npos);
    b.set(member(1, "Alex", polar(2800, 0)), 1000);   // a quarter of the way out
    v = one(b, 0, me, 0);
    CHECK(approx(v.tagAlpha, 0.75f));
    b.set(member(1, "Alex", polar(5600, 0)), 1000);   // three quarters
    v = one(b, 0, me, 0);
    CHECK(approx(v.tagAlpha, 0.25f));
    b.set(member(1, "Alex", polar(7000, 0)), 1000);
    CHECK(!one(b, 0, me, 0).tag);
    b.set(member(1, "Alex", polar(9000, 0)), 1000);
    v = one(b, 0, me, 0);
    CHECK(!v.tag);
    CHECK(approx(v.tagAlpha, 0.0f));
}

TEST(board, no_tag_for_a_friend_in_another_space_but_he_is_located) {
    MarkerBoard b;
    b.set(member(1, "Alex", indoors(0x100, 0, 0)), 1000);
    MarkerView v = one(b, 0, outdoors(0, 0), 0);
    CHECK(!v.tag);
    CHECK(!v.sameSpace);
    CHECK(v.where.find("inside Vault 21") != std::string::npos);
    b.set(member(1, "Zed", outdoors(10, 0, 0, kOtherWorld)), 1000);
    v = one(b, 0, outdoors(0, 0), 0);
    CHECK(!v.tag);
    CHECK(!v.where.empty());
}

TEST(board, the_puppets_exact_place_beats_the_tables) {
    MarkerBoard b;
    b.set(member(1, "Alex", polar(5000, 90)), 1000);   // the table says 5000 away
    MarkerBoard::Puppets p;
    p.at[1] = polar(700, 90);                          // the puppet stands 700 away
    const auto v = b.views(0, outdoors(0, 0), {}, 0, 1000, p);
    REQUIRE(v.size() == 1);
    CHECK(approx(v[0].distUnits, 700.0f, 1.0f));
    CHECK(approx(v[0].tagAlpha, 1.0f));
}

TEST(board, a_stale_friend_is_not_shown) {
    MarkerBoard b;
    b.set(member(1, "Alex", polar(500, 0)), 1000);
    CHECK(b.views(0, outdoors(0, 0), {}, 0, 1000 + 3999).size() == 1);
    CHECK(b.views(0, outdoors(0, 0), {}, 0, 1000 + 4001).empty());
}

TEST(board, the_off_switch_and_the_per_surface_switches) {
    MarkerConfig c;
    c.enabled = false;
    MarkerBoard b(c);
    b.set(member(1, "Alex", polar(500, 0)), 1000);
    CHECK(b.views(0, outdoors(0, 0), {}, 0, 1000).empty());
    c.enabled = true;
    c.tags = false;
    b.setConfig(c);
    MarkerView v = one(b, 0, outdoors(0, 0), 0);
    CHECK(!v.tag && v.compass && v.localMap);
    c.tags = true; c.compass = false; c.map = false;
    b.setConfig(c);
    v = one(b, 0, outdoors(0, 0), 0);
    CHECK(v.tag && !v.compass && !v.localMap && !v.worldMap);
}

TEST(board, a_player_never_sees_himself) {
    MarkerBoard b;
    b.set(member(0, "Host", polar(100, 0)), 1000);
    CHECK(b.views(0, outdoors(0, 0), {}, 0, 1000).empty());
    CHECK(b.views(1, outdoors(0, 0), {}, 0, 1000).size() == 1);
}

// ------------------------------------------------------------------------------------------------------------------------------ compass
TEST(compass, bearing_against_the_heading_left_and_right) {
    MarkerBoard b;
    const Place me = outdoors(0, 0);
    b.set(member(1, "Alex", polar(2000, 0)), 1000);
    MarkerView v = one(b, 0, me, 0);
    CHECK(v.compass && v.edge == 0 && approx(v.compassX, 0.0f));
    b.set(member(1, "Alex", polar(2000, 45)), 1000);   // north-east: right of centre
    v = one(b, 0, me, 0);
    CHECK(approx(v.compassX, 0.5f));
    b.set(member(1, "Alex", polar(2000, -45)), 1000);
    v = one(b, 0, me, 0);
    CHECK(approx(v.compassX, -0.5f));
    // facing east: a friend to the east is straight ahead
    b.set(member(1, "Alex", polar(2000, 90)), 1000);
    v = one(b, 0, me, kPi / 2);
    CHECK(approx(v.compassX, 0.0f));
}

TEST(compass, a_friend_beyond_the_strip_is_pinned_to_its_edge) {
    MarkerBoard b;
    const Place me = outdoors(0, 0);
    b.set(member(1, "Alex", polar(2000, 180)), 1000);   // behind
    MarkerView v = one(b, 0, me, 0);
    CHECK(v.compass && v.edge != 0);
    CHECK(approx(std::fabs(v.compassX), 1.0f, 0.0001f));
    CHECK(v.compassGlyph == "<A" || v.compassGlyph == "A>");
    b.set(member(1, "Alex", polar(2000, 120)), 1000);   // right-behind
    v = one(b, 0, me, 0);
    CHECK(v.edge == 1 && approx(v.compassX, 1.0f, 0.0001f) && v.compassGlyph == "A>");
    b.set(member(1, "Alex", polar(2000, -120)), 1000);
    v = one(b, 0, me, 0);
    CHECK(v.edge == -1 && approx(v.compassX, -1.0f, 0.0001f) && v.compassGlyph == "<A");
}

TEST(compass, a_downed_friend_looks_different) {
    MarkerBoard b;
    Member m = member(1, "alex", polar(2000, 0));
    m.downed = true;
    b.set(m, 1000);
    CHECK(one(b, 0, outdoors(0, 0), 0).compassGlyph == "[A!]");
    m.downed = false;
    b.set(m, 1000);
    CHECK(one(b, 0, outdoors(0, 0), 0).compassGlyph == "[A]");
}

TEST(compass, a_friend_indoors_points_at_the_door_he_went_in_by) {
    MarkerBoard b;
    Member m = member(1, "Alex", indoors(0x100, 5, 5));
    m.ext = outdoors(3000, 0);   // he went in 3000 units east of here
    b.set(m, 1000);
    const MarkerView v = one(b, 0, outdoors(0, 0), 0);
    CHECK(v.compass);
    CHECK(approx(v.compassX, 1.0f, 0.01f) || v.edge == 1);
    CHECK(v.worldMap);
    CHECK(!v.localMap);
    CHECK(approx(v.mapAt.x, 3000.0f, 1.0f));
    CHECK(v.where.find("inside Vault 21") != std::string::npos);
    CHECK(v.where.find("near sector 0,0") != std::string::npos);
    // and the friend is nowhere on the compass when the viewer is inside and he is outside (no door known)
    Member out = member(1, "Alex", polar(500, 0));
    b.set(out, 1000);
    const MarkerView w = one(b, 0, indoors(0x200), 0, polar(100, 0));
    CHECK(!w.compass);
}

// ------------------------------------------------------------------------------------------------------------------------------ map
TEST(map, local_map_is_the_viewers_space_and_world_map_the_worldspace_he_looks_at) {
    MarkerBoard b;
    b.set(member(1, "Alex", polar(500, 0)), 1000);
    b.set(member(2, "Bo", outdoors(100, 100, 0, kOtherWorld)), 1000);
    b.set(member(3, "Cy", indoors(0x300)), 1000);
    const auto v = b.views(0, outdoors(0, 0), {}, 0, 1000);
    REQUIRE(v.size() == 3);
    CHECK(v[0].localMap && v[0].worldMap);
    CHECK(!v[1].localMap && !v[1].worldMap);   // another worldspace
    CHECK(!v[2].localMap && !v[2].worldMap);   // an interior with no known door
}

TEST(map, a_viewer_indoors_sees_the_world_map_of_the_place_he_came_from) {
    MarkerBoard b;
    b.set(member(1, "Alex", polar(500, 0)), 1000);
    const Place inside = indoors(0x200);
    auto v = b.views(0, inside, outdoors(0, 0), 0, 1000);
    REQUIRE(v.size() == 1);
    CHECK(v[0].worldMap && !v[0].localMap);
    v = b.views(0, inside, Place{}, 0, 1000);
    CHECK(!v[0].worldMap);
}

TEST(map, projection_onto_the_world_map_and_the_local_map) {
    WorldMapBounds b;
    b.valid = true;
    b.nwX = -10; b.nwY = 20; b.seX = 10; b.seY = 0;   // 21 x 21 cells
    float u = -1, v = -1;
    REQUIRE(worldMapUV(outdoors(0, 10 * 4096.0f + 100), b, u, v));   // the middle column, row 10 of 20..0
    CHECK(approx(u, 10.0f / 21.0f + 0.5f / 21.0f - 0.5f / 21.0f, 0.001f));
    CHECK(approx(v, (21.0f - 10.0f - 100.0f / 4096.0f) / 21.0f, 0.001f));
    REQUIRE(worldMapUV(outdoors(-10 * 4096.0f + 1, 20 * 4096.0f + 4000), b, u, v));   // the north-west corner
    CHECK(u >= 0.0f && u < 0.01f && v >= 0.0f && v < 0.01f);
    REQUIRE(worldMapUV(outdoors(10 * 4096.0f + 4000, 1), b, u, v));                  // the south-east corner
    CHECK(u > 0.99f && v > 0.99f);
    CHECK(!worldMapUV(outdoors(-11 * 4096.0f, 5), b, u, v));                          // west of the map
    CHECK(!worldMapUV(outdoors(0, -10), b, u, v));                                    // south of it
    CHECK(!worldMapUV(indoors(3), b, u, v));
    b.valid = false;
    CHECK(!worldMapUV(outdoors(0, 0), b, u, v));

    REQUIRE(localMapUV(outdoors(0, 0), outdoors(0, 1000), 4000, u, v));               // 1000 north: above the middle
    CHECK(approx(u, 0.5f) && approx(v, 0.375f));
    REQUIRE(localMapUV(outdoors(0, 0), outdoors(-2000, 0), 4000, u, v));              // 2000 west: left of the middle
    CHECK(approx(u, 0.25f) && approx(v, 0.5f));
    CHECK(!localMapUV(outdoors(0, 0), outdoors(4001, 0), 4000, u, v));
    CHECK(!localMapUV(outdoors(0, 0), indoors(2), 4000, u, v));
}

// ------------------------------------------------------------------------------------------------------------------------------ far friends
TEST(board, a_far_friend_gets_a_place_and_a_sector_a_near_one_does_not) {
    MarkerBoard b;
    b.set(member(1, "Alex", outdoors(-9000, 20000)), 1000);   // sector -3,4
    MarkerView v = one(b, 0, outdoors(0, 0), 0);
    CHECK(v.where == "Mojave Wasteland, sector -3,4");
    CHECK(v.label.find("Alex") == 0);
    CHECK(v.label.find("Mojave Wasteland, sector -3,4") != std::string::npos);
    CHECK(v.label.find(" km") != std::string::npos || v.label.find(" m") != std::string::npos);
    b.set(member(1, "Alex", polar(3000, 0)), 1000);
    v = one(b, 0, outdoors(0, 0), 0);
    CHECK(v.where.empty());
}

TEST(text, sectors_use_floor_so_negative_cells_are_their_own) {
    CHECK(sectorCoord(0) == 0);
    CHECK(sectorCoord(4095) == 0);
    CHECK(sectorCoord(4096) == 1);
    CHECK(sectorCoord(-1) == -1);
    CHECK(sectorCoord(-4096) == -1);
    CHECK(sectorCoord(-4097) == -2);
    CHECK(describePlace(outdoors(5000, -100), {}) == "Mojave Wasteland, sector 1,-1");
    CHECK(describePlace(indoors(1), outdoors(-10, 9000)) == "inside Vault 21, near sector -1,2");
    CHECK(describePlace(indoors(1), {}) == "inside Vault 21");
    CHECK(describePlace(Place{}, {}) == "somewhere");
}

TEST(text, distances_and_clocks) {
    CHECK(distanceText(700) == "10 m");
    CHECK(distanceText(70000) == "1.0 km");
    CHECK(distanceText(-5) == "0 m");
    CHECK(clockText(42000) == "0:42");
    CHECK(clockText(41001) == "0:42");
    CHECK(clockText(60000) == "1:00");
    CHECK(clockText(0) == "0:00");
    CHECK(tagText("Alex", false, 500) == "Alex");
    CHECK(tagText("Alex", true, 2800) == "Alex (down)  40 m");
}

TEST(text, a_name_cannot_carry_format_or_markup_characters_into_the_hud) {
    CHECK(sanitizeText("Al%\"ex<b>&\\", 24) == "Alexb");
    CHECK(sanitizeText("  tab\there\x01  ", 24) == "tabhere");
    CHECK(sanitizeText("abcdefghij", 4) == "abcd");
    CHECK(sanitizeText("\xC3\xA9", 24).empty());
}

// ------------------------------------------------------------------------------------------------------------------------------ wire
TEST(wire, beacon_party_and_revive_round_trip) {
    Beacon b;
    b.name = "Alex";
    b.at = outdoors(120.4f, -88.0f, 7.0f);
    b.ext = outdoors(1.0f, 2.0f, 3.0f);
    Beacon got;
    REQUIRE(decodeBeacon(encodeBeacon(b), got));
    CHECK(got.name == "Alex" && got.at.valid && got.at.world == kWorld && got.at.x == 120 && got.at.y == -88 && got.at.name == "Mojave Wasteland");
    CHECK(got.ext.valid && got.ext.x == 1);
    Beacon vault;
    vault.name = "Bo";
    vault.at = indoors(0x1234, 5, 6);
    REQUIRE(decodeBeacon(encodeBeacon(vault), got));
    CHECK(got.at.interior() && got.at.cell == 0x1234 && !got.ext.valid);

    for (RevivePhase ph : {RevivePhase::Begin, RevivePhase::Cancel, RevivePhase::Done}) {
        ReviveMsg r{ph, 2}, back;
        REQUIRE(decodeRevive(encodeRevive(r), back));
        CHECK(back.phase == ph && back.target == 2);
    }

    PartyTable t;
    t.seq = 77;
    Member a = member(0, "Host", outdoors(1, 2, 3));
    Member d = member(2, "Bo", indoors(9, 1, 1));
    d.ext = outdoors(50, 60);
    d.downed = true;
    d.windowLeftMs = 41500;
    d.reviver = 1;
    d.holdLeftMs = 1200;
    Member c = member(3, "Cy", outdoors(0, 0));
    c.downed = true;
    c.checkpointOnly = true;
    t.members = {a, d, c};
    PartyTable pt;
    REQUIRE(decodeParty(encodeParty(t), pt));
    REQUIRE(pt.members.size() == 3);
    CHECK(pt.seq == 77);
    CHECK(!pt.members[0].downed && pt.members[0].reviver == kNoSlot);
    CHECK(pt.members[1].downed && pt.members[1].windowLeftMs == 41500 && pt.members[1].reviver == 1 && pt.members[1].holdLeftMs == 1200);
    CHECK(pt.members[1].at.interior() && pt.members[1].ext.valid && pt.members[1].ext.x == 50);
    CHECK(pt.members[2].checkpointOnly && !revivable(pt.members[2]));
}

TEST(wire, hostile_input_is_refused_or_cleaned) {
    Beacon b;
    CHECK(!decodeBeacon("not json", b));
    CHECK(!decodeBeacon("{\"op\":\"party\"}", b));
    CHECK(!decodeBeacon("{\"op\":\"beacon\",\"n\":\"A\"}", b));                                   // no place
    CHECK(!decodeBeacon("{\"op\":\"beacon\",\"n\":\"A\",\"x\":1e30,\"y\":0}", b));              // out of range
    CHECK(!decodeBeacon("{\"op\":\"beacon\",\"n\":\"A\",\"x\":\"1\",\"y\":0}", b));            // not a number
    REQUIRE(decodeBeacon("{\"op\":\"beacon\",\"n\":\"Al%\\\"ex<\",\"x\":1,\"y\":2}", b));
    CHECK(b.name == "Alex");
    ReviveMsg r;
    CHECK(!decodeRevive("{\"op\":\"revive\",\"ph\":\"explode\",\"t\":1}", r));
    CHECK(!decodeRevive("{\"op\":\"revive\",\"ph\":\"begin\",\"t\":200}", r));
    CHECK(!decodeRevive("{\"op\":\"revive\",\"ph\":\"begin\",\"t\":-1}", r));
    CHECK(!decodeRevive("{\"op\":\"revive\",\"ph\":\"begin\",\"t\":1.5}", r));
    CHECK(!decodeRevive("[1]", r));
    PartyTable t;
    CHECK(!decodeParty("{\"op\":\"party\",\"m\":[{\"s\":99,\"x\":0,\"y\":0}]}", t));
    CHECK(!decodeParty("{\"op\":\"party\",\"m\":[{\"s\":1,\"x\":0,\"y\":0,\"d\":1,\"dl\":-5}]}", t));
    CHECK(!decodeParty("{\"op\":\"party\",\"m\":3}", t));
    std::string many = "{\"op\":\"party\",\"m\":[";
    for (int i = 0; i < 9; ++i) many += std::string(i ? "," : "") + "{\"s\":1,\"x\":0,\"y\":0}";
    CHECK(!decodeParty(many + "]}", t));
}

TEST(wire, the_in_place_respawn_order) {
    float f = 0;
    std::string by;
    nvcoop::json::Value v;
    std::string err;
    REQUIRE(nvcoop::json::parse(encodeInPlaceRespawn(0.3f, "Al%ex"), v, err));
    CHECK(v.get("op").asString("") == "respawn");
    REQUIRE(readInPlaceRespawn(v, f, by));
    CHECK(approx(f, 0.3f) && by == "Alex");
    // a checkpoint respawn carries no in-place flag
    REQUIRE(nvcoop::json::parse("{\"op\":\"respawn\",\"checkpoint\":\"x\"}", v, err));
    CHECK(!readInPlaceRespawn(v, f, by));
    // a silly figure is held to 5 .. 100 percent
    REQUIRE(nvcoop::json::parse("{\"op\":\"respawn\",\"inPlace\":true,\"frac\":-4}", v, err));
    REQUIRE(readInPlaceRespawn(v, f, by));
    CHECK(approx(f, 0.05f));
    REQUIRE(nvcoop::json::parse("{\"op\":\"respawn\",\"inPlace\":true,\"frac\":50}", v, err));
    REQUIRE(readInPlaceRespawn(v, f, by));
    CHECK(approx(f, 1.0f));
    CHECK(inPlaceDamageLine(0.3f) == "DamageActorValue Health (GetBaseAV Health * 0.70)");
    CHECK(inPlaceDamageLine(1.0f) == "DamageActorValue Health (GetBaseAV Health * 0.00)");
    CHECK(inPlaceDamageLine(0.0f) == "DamageActorValue Health (GetBaseAV Health * 0.95)");
}

TEST(wire, which_side_takes_which_op) {
    CHECK(isHostOp("beacon") && isHostOp("revive") && !isHostOp("party") && !isHostOp("downed"));
    CHECK(isGuestOp("party") && !isGuestOp("beacon") && !isGuestOp("respawn"));
}

// ------------------------------------------------------------------------------------------------------------------------------ revive authority
struct Where {
    std::map<Slot, Place> at;
    PlaceFn fn() const { return [this](Slot s, Place& p) { auto it = at.find(s); if (it == at.end()) return false; p = it->second; return true; }; }
};

// players 1 (downed) and 2 (up), a metre apart, 60 s window, all synced at t = 1000
struct Fix {
    ReviveAuthority a;
    Where w;
    uint64_t t = 1000;
    Fix() {
        w.at[1] = outdoors(0, 0);
        w.at[2] = outdoors(70, 0);
        w.at[3] = outdoors(140, 0);
        sync();
    }
    void sync(bool d1 = true) {
        a.sync(1, d1, false, 60000, t);
        a.sync(2, false, false, 60000, t);
        a.sync(3, false, false, 60000, t);
    }
    Verdict req(Slot from, RevivePhase ph, Slot target) { return a.request(from, ph, target, t, w.fn()); }
};

TEST(revive, a_friend_who_holds_long_enough_gets_him_up) {
    Fix f;
    CHECK(f.req(2, RevivePhase::Begin, 1) == Verdict::Ok);
    CHECK(f.a.reviverOf(1) == 2);
    f.t += 3000;
    CHECK(f.req(2, RevivePhase::Done, 1) == Verdict::Ok);
    const auto r = f.a.takeRevived();
    REQUIRE(r.size() == 1);
    CHECK(r[0].target == 1 && r[0].reviver == 2);
    CHECK(!f.a.isDowned(1));
    CHECK(f.a.takeRevived().empty());
}

TEST(revive, the_checks_say_why) {
    Fix f;
    CHECK(f.req(1, RevivePhase::Begin, 1) == Verdict::SelfRevive);
    CHECK(f.req(2, RevivePhase::Begin, 3) == Verdict::TargetNotDown);     // 3 is up
    CHECK(f.req(2, RevivePhase::Begin, 6) == Verdict::UnknownPlayer);
    f.a.sync(2, true, false, 60000, f.t);                                  // the friend is down himself
    CHECK(f.req(2, RevivePhase::Begin, 1) == Verdict::ReviverDown);
    f.a.sync(2, false, false, 60000, f.t);
    f.w.at[2] = outdoors(2000, 0);
    CHECK(f.req(2, RevivePhase::Begin, 1) == Verdict::TooFar);
    f.w.at[2] = outdoors(0, 0, 0, kOtherWorld);
    CHECK(f.req(2, RevivePhase::Begin, 1) == Verdict::OtherSpace);
    f.w.at[2] = indoors(5);
    CHECK(f.req(2, RevivePhase::Begin, 1) == Verdict::OtherSpace);
    f.w.at.erase(2);
    CHECK(f.req(2, RevivePhase::Begin, 1) == Verdict::UnknownPlayer);     // the host does not know where he is
    f.w.at[2] = outdoors(70, 0);
    CHECK(f.req(2, RevivePhase::Begin, 1) == Verdict::Ok);
}

TEST(revive, the_off_switch) {
    Fix f;
    ReviveRules r;
    r.enabled = false;
    f.a.setRules(r);
    CHECK(f.req(2, RevivePhase::Begin, 1) == Verdict::Disabled);
    CHECK(f.a.reviverOf(1) == kNoSlot);
}

TEST(revive, one_downed_player_has_one_friend_helping) {
    Fix f;
    CHECK(f.req(2, RevivePhase::Begin, 1) == Verdict::Ok);
    CHECK(f.req(3, RevivePhase::Begin, 1) == Verdict::Busy);
    CHECK(f.a.reviverOf(1) == 2);
    f.t += 3000;
    CHECK(f.req(3, RevivePhase::Done, 1) == Verdict::NotHolding);
    CHECK(f.a.isDowned(1));
    CHECK(f.req(2, RevivePhase::Done, 1) == Verdict::Ok);
}

TEST(revive, a_done_needs_a_begin_and_the_time_in_between) {
    Fix f;
    CHECK(f.req(2, RevivePhase::Done, 1) == Verdict::NotHolding);
    CHECK(f.a.isDowned(1));
    CHECK(f.req(2, RevivePhase::Begin, 1) == Verdict::Ok);
    f.t += 1000;
    CHECK(f.req(2, RevivePhase::Done, 1) == Verdict::TooEarly);
    CHECK(f.a.isDowned(1));
    f.t += 1700;   // 2700 + the 400 slack is the 3000
    CHECK(f.req(2, RevivePhase::Done, 1) == Verdict::Ok);
}

TEST(revive, cancel_is_only_the_helpers_to_make) {
    Fix f;
    CHECK(f.req(2, RevivePhase::Begin, 1) == Verdict::Ok);
    CHECK(f.req(3, RevivePhase::Cancel, 1) == Verdict::NotHolding);
    CHECK(f.a.reviverOf(1) == 2);
    CHECK(f.req(2, RevivePhase::Cancel, 1) == Verdict::Ok);
    CHECK(f.a.reviverOf(1) == kNoSlot);
    CHECK(f.req(3, RevivePhase::Begin, 1) == Verdict::Ok);   // somebody else may now
}

TEST(revive, a_collar_death_and_a_policy_without_a_window_cannot_be_revived) {
    Fix f;
    f.a.sync(1, true, true, 60000, f.t);   // down until the party reaches a checkpoint
    CHECK(f.req(2, RevivePhase::Begin, 1) == Verdict::NotRevivable);
    f.a.sync(1, true, false, 0, f.t);      // death.mode respawn: no window
    CHECK(f.req(2, RevivePhase::Begin, 1) == Verdict::NotRevivable);
    CHECK(f.a.windowLeftMs(1, f.t) == 0);
}

TEST(revive, the_window_runs_from_the_first_time_he_is_seen_down_and_ends) {
    Fix f;
    CHECK(f.a.windowLeftMs(1, f.t) == 60000);
    f.t += 1000;
    f.sync();   // the same frame loop keeps syncing: the window must not restart
    CHECK(f.a.windowLeftMs(1, f.t) == 59000);
    f.t += 58500;
    CHECK(f.a.windowLeftMs(1, f.t) == 500);
    f.t += 600;
    CHECK(f.a.windowLeftMs(1, f.t) == 0);
    CHECK(f.req(2, RevivePhase::Begin, 1) == Verdict::WindowOver);
}

TEST(revive, a_hold_is_dropped_when_the_friend_goes_down_walks_off_or_never_finishes) {
    {   // goes down
        Fix f;
        CHECK(f.req(2, RevivePhase::Begin, 1) == Verdict::Ok);
        f.a.sync(2, true, false, 60000, f.t);
        f.a.tick(f.t, f.w.fn());
        CHECK(f.a.reviverOf(1) == kNoSlot);
    }
    {   // walks off
        Fix f;
        CHECK(f.req(2, RevivePhase::Begin, 1) == Verdict::Ok);
        f.w.at[2] = outdoors(5000, 0);
        f.a.tick(f.t, f.w.fn());
        CHECK(f.a.reviverOf(1) == kNoSlot);
    }
    {   // never finishes
        Fix f;
        CHECK(f.req(2, RevivePhase::Begin, 1) == Verdict::Ok);
        f.t += 7000;
        f.a.tick(f.t, f.w.fn());
        CHECK(f.a.reviverOf(1) == 2);
        f.t += 2000;
        f.a.tick(f.t, f.w.fn());
        CHECK(f.a.reviverOf(1) == kNoSlot);
    }
    {   // leaves the party
        Fix f;
        CHECK(f.req(2, RevivePhase::Begin, 1) == Verdict::Ok);
        f.a.forget(2);
        CHECK(f.a.reviverOf(1) == kNoSlot);
    }
}

TEST(revive, a_friend_helps_one_downed_player_at_a_time) {
    Fix f;
    f.a.sync(3, true, false, 60000, f.t);   // 3 is down too
    CHECK(f.req(2, RevivePhase::Begin, 1) == Verdict::Ok);
    CHECK(f.req(2, RevivePhase::Begin, 3) == Verdict::Ok);
    CHECK(f.a.reviverOf(3) == 2);
    CHECK(f.a.reviverOf(1) == kNoSlot);
}

TEST(revive, hold_progress_is_reported_for_the_downed_players_bar) {
    Fix f;
    CHECK(f.a.holdLeftMs(1, f.t) == 0);
    CHECK(f.req(2, RevivePhase::Begin, 1) == Verdict::Ok);
    CHECK(f.a.holdLeftMs(1, f.t) == 3000);
    CHECK(f.a.holdLeftMs(1, f.t + 1000) == 2000);
    CHECK(f.a.holdLeftMs(1, f.t + 9000) == 0);
}

// ------------------------------------------------------------------------------------------------------------------------------ hold tracker
HoldInput held(uint64_t now, float dist = 100, bool revivable = true) {
    HoldInput in;
    in.now = now;
    in.activateHeld = true;
    HoldCandidate c;
    c.slot = 1;
    c.distUnits = dist;
    c.revivable = revivable;
    in.candidates.push_back(c);
    return in;
}

TEST(hold, begin_then_done_after_the_hold_time_and_a_cooldown) {
    ReviveHold h;
    HoldOut o = h.step(held(1000));
    CHECK(o.ev == HoldEvent::Begin && o.target == 1);
    CHECK(h.active());
    CHECK(h.step(held(2000)).ev == HoldEvent::None);
    CHECK(approx(h.progress(2000), 1.0f / 3.0f, 0.01f));
    CHECK(h.step(held(3999)).ev == HoldEvent::None);
    o = h.step(held(4000));
    CHECK(o.ev == HoldEvent::Done && o.target == 1);
    CHECK(!h.active());
    // still holding with him still down: nothing for the cooldown, then a new hold
    CHECK(h.step(held(5000)).ev == HoldEvent::None);
    CHECK(h.step(held(6999)).ev == HoldEvent::None);
    CHECK(h.step(held(7000)).ev == HoldEvent::Begin);
}

TEST(hold, letting_go_cancels_and_a_new_press_starts_over) {
    ReviveHold h;
    CHECK(h.step(held(1000)).ev == HoldEvent::Begin);
    HoldInput up = held(2000);
    up.activateHeld = false;
    HoldOut o = h.step(up);
    CHECK(o.ev == HoldEvent::Cancel && o.target == 1);
    CHECK(h.step(up).ev == HoldEvent::None);
    CHECK(h.step(held(2500)).ev == HoldEvent::Begin);
    CHECK(h.step(held(5400)).ev == HoldEvent::None);   // 2900 ms since the new begin
    CHECK(h.step(held(5500)).ev == HoldEvent::Done);
}

TEST(hold, only_a_friend_in_reach_who_can_be_revived_counts) {
    ReviveHold h;
    CHECK(h.step(held(1000, 300)).ev == HoldEvent::None);       // 4 m away
    CHECK(h.candidate() == kNoSlot);
    CHECK(h.step(held(1000, 100, false)).ev == HoldEvent::None);   // up, or down for good
    HoldInput busy = held(1000);
    busy.candidates[0].busyWithOther = true;
    CHECK(h.step(busy).ev == HoldEvent::None);
    CHECK(h.step(held(1000)).ev == HoldEvent::Begin);
    CHECK(h.candidate() == 1);
}

TEST(hold, a_downed_player_a_menu_or_no_key_starts_nothing) {
    ReviveHold h;
    HoldInput in = held(1000);
    in.selfUp = false;
    CHECK(h.step(in).ev == HoldEvent::None);
    in = held(1000);
    in.blocked = true;
    CHECK(h.step(in).ev == HoldEvent::None);
    in = held(1000);
    in.activateHeld = false;
    CHECK(h.step(in).ev == HoldEvent::None);
    CHECK(h.candidate() == 1);   // the prompt still names him
}

TEST(hold, the_nearest_friend_is_chosen_and_a_switch_starts_over) {
    ReviveHold h;
    HoldInput in = held(1000, 120);
    HoldCandidate isFar;
    isFar.slot = 2;
    isFar.distUnits = 140;
    isFar.revivable = true;
    in.candidates.push_back(isFar);
    CHECK(h.step(in).ev == HoldEvent::Begin);
    CHECK(h.target() == 1);
    in.now = 1500;
    in.candidates[0].distUnits = 145;   // now 2 is nearer
    HoldOut o = h.step(in);
    CHECK(o.ev == HoldEvent::Cancel && o.target == 1);
    in.now = 1550;
    o = h.step(in);
    CHECK(o.ev == HoldEvent::Begin && o.target == 2);
}

TEST(hold, the_target_getting_up_or_walking_away_cancels) {
    ReviveHold h;
    CHECK(h.step(held(1000)).ev == HoldEvent::Begin);
    CHECK(h.step(held(1500, 100, false)).ev == HoldEvent::Cancel);   // somebody else got him up
    CHECK(h.step(held(1600)).ev == HoldEvent::Begin);
    CHECK(h.step(held(1700, 400)).ev == HoldEvent::Cancel);          // out of reach
}

}  // namespace
