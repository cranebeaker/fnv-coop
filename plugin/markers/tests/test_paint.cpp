// plugin/markers painter tests (game-free): what lands where on the window for one HudFrame (tags, compass, Pip-Boy map, list, banner).
// The FNV_MUT twins 1450..1464 of plugin/markers must each fail this file (plugin\markers\run_tests.cmd).
#include <cmath>

#include "minitest.h"
#include "mk_paint.h"

using namespace nvmk;

namespace {

bool approx(float a, float b, float eps = 0.05f) { return std::fabs(a - b) <= eps; }

Rect rect(float x, float y, float w, float h) {
    Rect r;
    r.valid = true;
    r.x = x; r.y = y; r.w = w; r.h = h;
    return r;
}
TileProbe tile(float x, float y, float w, float h, bool visible = true) {
    TileProbe t;
    t.r = rect(x, y, w, h);
    t.visible = visible;
    return t;
}
Place at(float x, float y, float z = 0) {
    Place p;
    p.valid = true;
    p.world = 0x3C;
    p.cell = 7;
    p.x = x; p.y = y; p.z = z;
    return p;
}

// a camera at the origin looking north, 16:9 (the same construction as test_project.cpp)
CameraMatrix northCamera() {
    CameraMatrix c;
    c.valid = true;
    c.m[0] = 1.0f / 0.8f;                    // across: x / tan(half width)
    c.m[6] = 1.0f / 0.45f;                   // down: z / tan(half height)
    c.m[13] = 1.0f;                          // depth: y
    return c;
}

SceneInput scene(float w = 1600, float h = 900) {
    SceneInput s;
    s.pxW = w; s.pxH = h;
    s.hud = rect(0, 0, 1600, 900);
    s.cam = northCamera();
    return s;
}

HudTag tag(const char* text, Place p, float alpha = 1.0f) {
    HudTag t;
    t.slot = 1; t.text = text; t.alpha = alpha; t.at = p;
    return t;
}
const PText* findText(const PaintList& l, const std::string& s) {
    for (const PText& t : l.texts)
        if (t.s == s) return &t;
    return nullptr;
}
HudCompassMark mark(float x, bool downed = false) {
    HudCompassMark m;
    m.slot = 1; m.glyph = downed ? "[A!]" : "[A]"; m.x = x; m.downed = downed;
    return m;
}
HudMapMark worldMark(const char* label, Place p, bool downed = false) {
    HudMapMark m;
    m.slot = 1; m.label = label; m.world = true; m.at = p; m.downed = downed;
    return m;
}
SceneInput pipboy() {
    SceneInput s = scene();
    s.ctx.pipboyMapOpen = true;
    s.ctx.menuOpen = true;
    s.ctx.engineOk = true;
    s.ctx.bounds.valid = true;
    s.ctx.bounds.nwX = -10; s.ctx.bounds.nwY = 10; s.ctx.bounds.seX = 10; s.ctx.bounds.seY = -10;   // 21 x 21 cells
    s.ctx.viewer = at(0, 0);
    s.worldMap = tile(100, 120, 1000, 600);
    return s;
}

}  // namespace

// ------------------------------------------------------------------------------------------------------------------------------ the HUD's screen
TEST(paint, the_huds_units_follow_the_window_like_the_engines_do) {
    Rect r = deriveHudScreen(1920, 1080);
    CHECK(r.valid && approx(r.h, 960.0f) && approx(r.w, 1920.0f / 1080.0f * 960.0f, 0.1f));
    r = deriveHudScreen(1280, 960);
    CHECK(approx(r.w, 1280.0f) && approx(r.h, 960.0f));                             // 4:3
    r = deriveHudScreen(800, 1000);                                                 // portrait
    CHECK(approx(r.w, 1280.0f) && approx(r.h, 1000.0f / 800.0f * 1280.0f, 0.1f));
    CHECK(!deriveHudScreen(0, 100).valid);
}

TEST(paint, without_a_readable_hud_root_the_derived_screen_scales_the_compass) {
    HudFrame f;
    f.compass.push_back(mark(0.0f));
    SceneInput s = scene(1920, 1080);
    s.hud = Rect{};                                                                 // the root tile could not be read: 1920 x 1080 px = 1706.7 x 960 units
    s.compassTile = tile(666.7f, 830, 373.3f, 48);                                  // a compass in the middle of the derived units
    const PaintList l = paintFrame(f, HudLayout{}, s);
    REQUIRE(l.texts.size() == 1);
    CHECK(approx(l.texts[0].x, 960.0f, 0.5f));
    CHECK(approx(l.texts[0].y, (830.0f + 14.0f) * 1.125f, 0.2f));
}

// ------------------------------------------------------------------------------------------------------------------------------ name tags
TEST(paint, a_tag_hangs_over_the_friends_head) {
    HudFrame f;
    f.tags.push_back(tag("Alex", at(0, 1000)));
    const PaintList l = paintFrame(f, HudLayout{}, scene());
    REQUIRE(l.texts.size() == 1);
    CHECK(l.tags == 1);
    const PText& t = l.texts[0];
    CHECK(t.s == "Alex");
    CHECK(approx(t.x, 800.0f));
    // head point: 135 units up at 1000 ahead -> 0.3 of the half view above the middle -> y 315; lifted 14 HUD units (1 unit = 1 px here) and its own height
    CHECK(approx(t.y, 315.0f - 14.0f - t.h, 0.2f));
    CHECK(approx(t.a, 1.0f));
}

TEST(paint, a_tag_fades_with_the_boards_alpha_and_a_nearly_gone_one_is_skipped) {
    HudFrame f;
    f.tags.push_back(tag("Alex", at(0, 1000), 0.4f));
    PaintList l = paintFrame(f, HudLayout{}, scene());
    REQUIRE(l.texts.size() == 1);
    CHECK(approx(l.texts[0].a, 0.4f, 0.001f));
    f.tags[0].alpha = 0.01f;
    CHECK(paintFrame(f, HudLayout{}, scene()).texts.empty());
}

TEST(paint, a_downed_friend_has_another_colour) {
    HudFrame f;
    f.tags.push_back(tag("Alex", at(0, 1000)));
    f.tags.push_back(tag("Bo (down)", at(100, 1000)));
    const PaintList l = paintFrame(f, HudLayout{}, scene());
    REQUIRE(l.texts.size() == 2);
    CHECK(l.texts[0].rgb != l.texts[1].rgb);
}

TEST(paint, a_friend_behind_or_to_the_side_gets_no_tag) {
    HudFrame f;
    f.tags.push_back(tag("Behind", at(0, -1000)));
    f.tags.push_back(tag("Side", at(6000, 1000)));
    f.tags.push_back(tag("Invalid", Place{}));
    CHECK(paintFrame(f, HudLayout{}, scene()).texts.empty());
    SceneInput s = scene();
    s.cam = CameraMatrix{};                                                         // no camera read: no tags
    f.tags.clear();
    f.tags.push_back(tag("Alex", at(0, 1000)));
    CHECK(paintFrame(f, HudLayout{}, s).texts.empty());
}

TEST(paint, the_head_height_comes_from_the_layout) {
    HudFrame f;
    f.tags.push_back(tag("Alex", at(0, 1000)));
    HudLayout lay;
    lay.tagHeadUnits = 0.0f;
    const PaintList l = paintFrame(f, lay, scene());
    REQUIRE(l.texts.size() == 1);
    CHECK(approx(l.texts[0].y, 450.0f - 14.0f - l.texts[0].h, 0.2f));             // the feet are on the horizon line
}

TEST(paint, nothing_of_the_hud_is_drawn_over_a_menu_or_with_the_hud_hidden) {
    HudFrame f;
    f.tags.push_back(tag("Alex", at(0, 1000)));
    f.banner = "You are down.";
    SceneInput s = scene();
    s.ctx.menuOpen = true;
    CHECK(paintFrame(f, HudLayout{}, s).empty());
    s = scene();
    s.hudShown = false;
    CHECK(paintFrame(f, HudLayout{}, s).empty());
    CHECK(!paintFrame(f, HudLayout{}, scene()).empty());
}

// ------------------------------------------------------------------------------------------------------------------------------ compass
TEST(paint, a_compass_glyph_sits_over_the_game_compass_tile) {
    HudFrame f;
    f.compass.push_back(mark(0.0f));
    SceneInput s = scene();
    s.compassTile = tile(560, 820, 400, 48);
    const PaintList l = paintFrame(f, HudLayout{}, s);
    REQUIRE(l.texts.size() == 1);
    CHECK(l.compass == 1);
    CHECK(approx(l.texts[0].x, 760.0f) && approx(l.texts[0].y, 834.0f) && approx(l.texts[0].h, 20.0f));
    CHECK(l.rects.empty());                                                         // the game's own compass is the strip: no line of ours
}

TEST(paint, the_compass_follows_the_windows_pixel_size) {
    HudFrame f;
    f.compass.push_back(mark(0.0f));
    SceneInput s = scene(3200, 1800);                                               // twice the pixels, the same HUD units
    s.compassTile = tile(560, 820, 400, 48);
    const PaintList l = paintFrame(f, HudLayout{}, s);
    REQUIRE(l.texts.size() == 1);
    CHECK(approx(l.texts[0].x, 1520.0f) && approx(l.texts[0].y, 1668.0f) && approx(l.texts[0].h, 40.0f));
}

TEST(paint, a_pinned_glyph_stays_inside_the_compass_and_a_downed_one_is_marked) {
    HudFrame f;
    f.compass.push_back(mark(-1.0f));
    f.compass.push_back(mark(1.0f, true));
    SceneInput s = scene();
    s.compassTile = tile(560, 820, 400, 48);
    const PaintList l = paintFrame(f, HudLayout{}, s);
    REQUIRE(l.texts.size() == 2);
    CHECK(approx(l.texts[0].x, 578.0f) && approx(l.texts[1].x, 942.0f));
    CHECK(l.texts[0].rgb != l.texts[1].rgb);
}

TEST(paint, no_compass_tile_means_a_strip_of_our_own) {
    HudFrame f;
    f.compass.push_back(mark(1.0f));
    const PaintList l = paintFrame(f, HudLayout{}, scene());
    REQUIRE(l.texts.size() == 1);
    REQUIRE(l.rects.size() == 1);
    CHECK(approx(l.texts[0].x, 800.0f + 0.15f * 1600.0f));                          // the default strip: 30 per cent wide, centred
    CHECK(approx(l.rects[0].w, 0.30f * 1600.0f));
}

TEST(paint, a_compass_the_hud_hides_gets_no_glyphs) {
    HudFrame f;
    f.compass.push_back(mark(0.0f));
    SceneInput s = scene();
    s.compassTile = tile(560, 820, 400, 48, false);
    CHECK(paintFrame(f, HudLayout{}, s).empty());
}

// ------------------------------------------------------------------------------------------------------------------------------ banner
TEST(paint, the_banner_is_at_banner_y_with_a_progress_bar_while_a_revive_is_held) {
    HudFrame f;
    f.banner = "Reviving Alex... 50%";
    f.bannerProgress = 0.5f;
    HudLayout lay;
    lay.bannerY = 400;
    const PaintList l = paintFrame(f, lay, scene());
    const PText* b = findText(l, f.banner);
    REQUIRE(b != nullptr);
    CHECK(approx(b->x, 800.0f) && approx(b->y, 400.0f, 0.1f));
    REQUIRE(l.rects.size() == 2);
    CHECK(approx(l.rects[1].w, 0.5f * l.rects[0].w));                               // the fill is half the bar
    CHECK(l.rects[1].y > b->y + b->h);                                              // under the text
    f.bannerProgress = -1.0f;
    CHECK(paintFrame(f, lay, scene()).rects.empty());
}

TEST(paint, a_downed_local_player_gets_a_banner_of_another_colour) {
    HudFrame f;
    f.banner = "You are down.";
    const PaintList up = paintFrame(f, HudLayout{}, scene());
    f.local = true;
    const PaintList down = paintFrame(f, HudLayout{}, scene());
    REQUIRE(up.texts.size() == 1);
    REQUIRE(down.texts.size() == 1);
    CHECK(up.texts[0].rgb != down.texts[0].rgb);
}

// ------------------------------------------------------------------------------------------------------------------------------ Pip-Boy map
TEST(paint, a_marker_goes_through_the_world_map_tiles_rectangle) {
    HudFrame f;
    f.map.push_back(worldMark("Alex, 340 m - Mojave", at(0, 0)));
    f.list.push_back("Alex, 340 m - Mojave");
    const PaintList l = paintFrame(f, HudLayout{}, pipboy());
    CHECK(l.mapPlaced == 1);
    CHECK(!l.list);                                                                 // everybody sits on the map: no list
    REQUIRE(l.rects.size() == 2);
    const float cx = l.rects[1].x + l.rects[1].w * 0.5f, cy = l.rects[1].y + l.rects[1].h * 0.5f;
    CHECK(approx(cx, 100.0f + 1000.0f * (10.0f / 21.0f), 0.1f));                    // the middle cell column of 21
    CHECK(approx(cy, 120.0f + 600.0f * (11.0f / 21.0f), 0.1f));                     // north-west corner is cell 10, so y = 0 is 11 cells down
    const PText* name = findText(l, "Alex");
    REQUIRE(name != nullptr);                                                       // only the name next to the marker
    CHECK(name->x > cx);
}

TEST(paint, the_map_follows_a_scrolled_and_scaled_tile_and_the_windows_pixels) {
    HudFrame f;
    f.map.push_back(worldMark("Alex, 1 m", at(0, 0)));
    f.list.push_back("Alex, 1 m");
    SceneInput s = pipboy();
    s.pxW = 3200; s.pxH = 1800;
    s.worldMap = tile(-400, -90, 2000, 1200);
    const PaintList l = paintFrame(f, HudLayout{}, s);
    REQUIRE(l.rects.size() == 2);
    CHECK(approx(l.rects[1].x + l.rects[1].w * 0.5f, 2.0f * (-400.0f + 2000.0f * (10.0f / 21.0f)), 0.2f));
    CHECK(approx(l.rects[1].y + l.rects[1].h * 0.5f, 2.0f * (-90.0f + 1200.0f * (11.0f / 21.0f)), 0.2f));
}

TEST(paint, a_marker_outside_the_viewport_is_not_drawn_and_the_list_takes_over) {
    HudFrame f;
    f.map.push_back(worldMark("Alex, 1 m", at(0, 0)));
    f.list.push_back("Alex, 1 m");
    SceneInput s = pipboy();
    s.worldMap.viewport = rect(900, 120, 200, 600);                                 // the frame shows only the east edge
    const PaintList l = paintFrame(f, HudLayout{}, s);
    CHECK(l.mapPlaced == 0);
    CHECK(l.rects.empty());
    CHECK(l.list);
    CHECK(findText(l, "Alex, 1 m") != nullptr);
}

TEST(paint, the_list_is_the_fallback_when_no_map_tile_can_be_read) {
    HudFrame f;
    f.map.push_back(worldMark("Alex, 1 m", at(0, 0)));
    f.list.push_back("Alex, 1 m");
    f.list.push_back("Bo, 5 km - Vault 21");
    SceneInput s = pipboy();
    s.worldMap = TileProbe{};
    PaintList l = paintFrame(f, HudLayout{}, s);
    CHECK(l.list && l.rects.empty());
    CHECK(l.texts.size() == 2);
    HudLayout lay;
    lay.mapMode = "list";
    s = pipboy();
    l = paintFrame(f, lay, s);
    CHECK(l.list && l.rects.empty());                                               // list-only mode never puts markers on the map
}

TEST(paint, a_friend_who_is_not_on_the_map_keeps_the_list_up) {
    HudFrame f;
    f.map.push_back(worldMark("Alex, 1 m", at(0, 0)));
    f.list.push_back("Alex, 1 m");
    f.list.push_back("Bo, 5 km - Vault 21");                                        // inside an interior: no marker for him
    const PaintList l = paintFrame(f, HudLayout{}, pipboy());
    CHECK(l.mapPlaced == 1);
    CHECK(l.list);
}

TEST(paint, the_manual_modes_use_the_rectangle_from_the_ini_and_always_list) {
    HudFrame f;
    f.map.push_back(worldMark("Alex, 1 m", at(0, 0)));
    f.list.push_back("Alex, 1 m");
    HudLayout lay;
    lay.mapMode = "world";
    SceneInput s = pipboy();
    s.worldMap = TileProbe{};
    const PaintList l = paintFrame(f, lay, s);
    CHECK(l.mapPlaced == 1);
    CHECK(l.list);
    REQUIRE(l.rects.size() == 2);
    CHECK(approx(l.rects[1].x + l.rects[1].w * 0.5f, (0.2f + 0.6f * (10.0f / 21.0f)) * 1600.0f, 0.2f));
}

TEST(paint, a_local_map_marker_is_north_up_around_the_player) {
    HudFrame f;
    HudMapMark m;
    m.slot = 1; m.label = "Alex, 20 m"; m.local = true; m.at = at(0, 700);
    f.map.push_back(m);
    f.list.push_back("Alex, 20 m");
    SceneInput s = pipboy();
    s.worldMap = TileProbe{};
    s.localMap = tile(300, 100, 800, 700);
    HudLayout lay;
    lay.localMapHalfRangeUnits = 1400.0f;
    const PaintList l = paintFrame(f, lay, s);
    REQUIRE(l.rects.size() == 2);
    CHECK(approx(l.rects[1].x + l.rects[1].w * 0.5f, 700.0f, 0.2f));                // straight above the middle: same x
    CHECK(approx(l.rects[1].y + l.rects[1].h * 0.5f, 100.0f + 700.0f * 0.25f, 0.2f));   // a quarter of the way down from the top: north of the middle
}

TEST(paint, the_map_is_painted_in_the_pipboy_only) {
    HudFrame f;
    f.map.push_back(worldMark("Alex, 1 m", at(0, 0)));
    f.list.push_back("Alex, 1 m");
    SceneInput s = pipboy();
    s.ctx.pipboyMapOpen = false;
    s.ctx.menuOpen = false;
    CHECK(paintFrame(f, HudLayout{}, s).empty());
    s.ctx.menuOpen = true;
    CHECK(paintFrame(f, HudLayout{}, s).empty());
}

TEST(paint, a_pipboy_map_shows_no_tags_or_banner) {
    HudFrame f;
    f.tags.push_back(tag("Alex", at(0, 1000)));
    f.banner = "You are down.";
    CHECK(paintFrame(f, HudLayout{}, pipboy()).empty());
}

TEST(paint, an_empty_frame_or_a_window_of_no_size_paints_nothing) {
    CHECK(paintFrame(HudFrame{}, HudLayout{}, scene()).empty());
    HudFrame f;
    f.banner = "x";
    CHECK(paintFrame(f, HudLayout{}, scene(0, 0)).empty());
    CHECK(firstField("Alex, 340 m - x") == "Alex");
    CHECK(firstField("Alex") == "Alex");
}
