// plugin/markers layout tests (game-free): the compass strip over the game's own compass tile, the Pip-Boy map tiles.
// The FNV_MUT twins 1420..1434 of plugin/markers must each fail this file (plugin\markers\run_tests.cmd).
#include <cmath>

#include "minitest.h"
#include "mk_layout.h"

using namespace nvmk;

namespace {

bool approx(float a, float b, float eps = 0.001f) { return std::fabs(a - b) <= eps; }

Rect rect(float x, float y, float w, float h) {
    Rect r;
    r.valid = true;
    r.x = x; r.y = y; r.w = w; r.h = h;
    return r;
}
const Rect kScreen = rect(0, 0, 1600, 900);

TileProbe tile(float x, float y, float w, float h, bool visible = true) {
    TileProbe t;
    t.r = rect(x, y, w, h);
    t.visible = visible;
    return t;
}
// a compass like the game's: wide and flat, a little left of the middle, near the bottom of the screen
TileProbe gameCompass() { return tile(560, 820, 400, 48); }

}  // namespace

// ------------------------------------------------------------------------------------------------------------------------------ compass
TEST(layout, the_strip_lies_over_the_compass_tile) {
    const CompassLayout l = compassLayout(gameCompass(), kScreen, {});
    REQUIRE(l.fromTile);
    CHECK(!l.hidden);
    CHECK(approx(l.centerX, 760.0f / 1600.0f));                  // the middle of the tile, not its left edge
    CHECK(approx(l.width, 400.0f / 1600.0f));                    // its own width, not the default
    CHECK(approx(l.y, 820.0f + (48.0f - 20.0f) * 0.5f));         // glyphs centred on the tile's height
}

TEST(layout, a_mark_moves_with_the_compass_and_stays_inside_its_ends) {
    const CompassLayout l = compassLayout(gameCompass(), kScreen, {});
    CHECK(approx(compassScreenX(l, 0.0f, kScreen), 760.0f / 1600.0f));
    CHECK(approx(compassScreenX(l, -1.0f, kScreen), (560.0f + 18.0f) / 1600.0f));   // pinned: half a glyph inside the left end
    CHECK(approx(compassScreenX(l, 1.0f, kScreen), (960.0f - 18.0f) / 1600.0f));
    CHECK(approx(compassScreenX(l, 0.5f, kScreen), (760.0f + 0.5f * (200.0f - 18.0f)) / 1600.0f));
    CHECK(approx(compassScreenX(l, 7.0f, kScreen), compassScreenX(l, 1.0f, kScreen)));   // never beyond the end
}

TEST(layout, the_ini_keys_override_their_part_only) {
    CompassOverride ov;
    ov.hasX = true; ov.x = 0.25f;
    CompassLayout l = compassLayout(gameCompass(), kScreen, ov);
    CHECK(approx(l.centerX, 0.25f));
    CHECK(approx(l.width, 400.0f / 1600.0f));                    // the width still comes from the tile
    ov = {};
    ov.hasW = true; ov.w = 0.5f;
    ov.hasY = true; ov.y = 100;
    l = compassLayout(gameCompass(), kScreen, ov);
    CHECK(approx(l.width, 0.5f));
    CHECK(approx(l.y, 100.0f));
    CHECK(approx(l.centerX, 760.0f / 1600.0f));
}

TEST(layout, without_a_usable_tile_the_ini_strip_stays_as_it_was) {
    const CompassOverride none;
    TileProbe missing;
    CompassLayout l = compassLayout(missing, kScreen, none);
    CHECK(!l.fromTile);
    CHECK(approx(l.centerX, 0.5f) && approx(l.width, 0.30f) && approx(l.y, 36.0f) && approx(l.insetUnits, 0.0f));
    CHECK(approx(compassScreenX(l, 1.0f, kScreen), 0.5f + 0.15f));    // as the strip always drew
    l = compassLayout(gameCompass(), Rect{}, none);                    // the screen could not be read
    CHECK(!l.fromTile);
    CHECK(!plausibleCompass(tile(100, 100, 60, 300), kScreen));       // tall and narrow: a list, not a compass
    CHECK(!plausibleCompass(tile(100, 100, 120, 100), kScreen));      // squarish
    CHECK(!plausibleCompass(tile(1500, 820, 400, 48), kScreen));      // runs off the screen
    CHECK(!plausibleCompass(tile(100, 100, 40, 8), kScreen));         // too small
    CHECK(plausibleCompass(gameCompass(), kScreen));
}

TEST(layout, a_compass_the_hud_hides_gets_no_glyphs) {
    const CompassLayout l = compassLayout(tile(560, 820, 400, 48, false), kScreen, {});
    CHECK(l.hidden);
    CHECK(l.fromTile);
}

// ------------------------------------------------------------------------------------------------------------------------------ map
TEST(layout, the_visible_map_tile_is_the_one_markers_go_on) {
    TileProbe local = tile(100, 120, 900, 560, false), world = tile(100, 120, 1800, 1100);
    world.r = rect(100, 120, 1000, 600);
    MapPick p = chooseMap(local, world, kScreen);
    CHECK(p.kind == MapKind::World);
    local.visible = true;
    world.visible = false;
    p = chooseMap(local, world, kScreen);
    CHECK(p.kind == MapKind::Local);
    CHECK(approx(p.tile.r.w, 900.0f));
    local.visible = world.visible = true;                           // both claim to be shown: do not guess
    p = chooseMap(local, world, kScreen);
    CHECK(p.kind == MapKind::None);
    local.visible = world.visible = false;
    CHECK(chooseMap(local, world, kScreen).kind == MapKind::None);
    CHECK(chooseMap(TileProbe{}, TileProbe{}, kScreen).kind == MapKind::None);
}

TEST(layout, a_map_tile_that_cannot_carry_markers_means_the_list) {
    TileProbe world = tile(100, 120, 1000, 600);
    CHECK(chooseMap(TileProbe{}, world, kScreen).kind == MapKind::World);
    TileProbe small = tile(100, 120, 120, 90);
    CHECK(chooseMap(TileProbe{}, small, kScreen).kind == MapKind::None);   // a button, not a map
    TileProbe zoomed = world;
    zoomed.hasZoom = true;
    zoomed.zoom = 2.5f;                                                    // a zoom the projection does not cover
    CHECK(chooseMap(TileProbe{}, zoomed, kScreen).kind == MapKind::None);
    zoomed.zoom = 1.0f;
    CHECK(chooseMap(TileProbe{}, zoomed, kScreen).kind == MapKind::World);
    zoomed.zoom = 100.0f;                                                  // the game's own "100 per cent"
    CHECK(chooseMap(TileProbe{}, zoomed, kScreen).kind == MapKind::World);
    CHECK(chooseMap(TileProbe{}, world, Rect{}).kind == MapKind::None);    // no screen size
}

TEST(layout, a_marker_goes_through_the_tiles_current_rectangle) {
    TileProbe world = tile(100, 120, 1000, 600);
    float x = 0, y = 0;
    REQUIRE(mapScreenPoint(world, 0.0f, 0.0f, x, y));                      // north-west corner
    CHECK(approx(x, 100.0f) && approx(y, 120.0f));
    REQUIRE(mapScreenPoint(world, 1.0f, 1.0f, x, y));                      // south-east corner
    CHECK(approx(x, 1100.0f) && approx(y, 720.0f));
    REQUIRE(mapScreenPoint(world, 0.25f, 0.75f, x, y));                    // three quarters down: south is down
    CHECK(approx(x, 350.0f) && approx(y, 570.0f));
    world.r = rect(-400, -90, 2000, 1200);                                 // scrolled and zoomed in: the image moved and grew
    REQUIRE(mapScreenPoint(world, 0.5f, 0.5f, x, y));
    CHECK(approx(x, 600.0f) && approx(y, 510.0f));
    CHECK(!mapScreenPoint(world, std::nanf(""), 0.5f, x, y));
    CHECK(!mapScreenPoint(TileProbe{}, 0.5f, 0.5f, x, y));
}

TEST(layout, a_marker_scrolled_out_of_the_frame_is_not_drawn) {
    TileProbe world = tile(-400, -90, 2000, 1200);
    world.viewport = rect(100, 120, 1000, 600);
    float x = 0, y = 0;
    CHECK(mapScreenPoint(world, 0.5f, 0.5f, x, y));                        // (600, 510) inside the frame
    CHECK(!mapScreenPoint(world, 0.02f, 0.5f, x, y));                      // (-360, 510): scrolled away to the west
    CHECK(!mapScreenPoint(world, 0.5f, 0.99f, x, y));                      // below the frame
}

TEST(layout, the_list_stays_unless_every_friend_is_on_the_map) {
    CHECK(mapListNeeded(false, 2, 2));        // no tile to draw on: the list
    CHECK(mapListNeeded(true, 1, 2));         // one friend is off the frame: the list names him
    CHECK(!mapListNeeded(true, 2, 2));
    CHECK(!mapListNeeded(true, 0, 0));        // nobody to show
}
