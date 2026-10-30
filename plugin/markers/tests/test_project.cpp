// plugin/markers projection tests (game-free): world point -> window pixel through the scene camera's matrix and viewport.
// The FNV_MUT twins 1441..1447 of plugin/markers must each fail this file (plugin\markers\run_tests.cmd).
#include <cmath>

#include "minitest.h"
#include "mk_project.h"

using namespace nvmk;

namespace {

bool approx(float a, float b, float eps = 0.05f) { return std::fabs(a - b) <= eps; }

// A camera standing at `pos`, looking along the unit vector `fwd` with `up` as its up axis, half field of view tangents tx (across) and ty (down),
// built the way the engine's matrix works: row 0 / row 1 give the position inside the view times the depth, row 3 the depth.
CameraMatrix makeCamera(const float pos[3], const float fwd[3], const float up[3], float tx, float ty) {
    const float right[3] = {fwd[1] * up[2] - fwd[2] * up[1], fwd[2] * up[0] - fwd[0] * up[2], fwd[0] * up[1] - fwd[1] * up[0]};
    CameraMatrix c;
    c.valid = true;
    auto dot = [](const float* a, const float* b) { return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]; };
    for (int i = 0; i < 3; ++i) {
        c.m[0 + i] = right[i] / tx;
        c.m[4 + i] = up[i] / ty;
        c.m[8 + i] = fwd[i];
        c.m[12 + i] = fwd[i];
    }
    c.m[3] = -dot(right, pos) / tx;
    c.m[7] = -dot(up, pos) / ty;
    c.m[11] = -dot(fwd, pos);
    c.m[15] = -dot(fwd, pos);
    return c;
}
const float kOrigin[3] = {0, 0, 0};
const float kNorth[3] = {0, 1, 0};
const float kUp[3] = {0, 0, 1};
CameraMatrix northCamera() { return makeCamera(kOrigin, kNorth, kUp, 0.8f, 0.45f); }   // 16:9

}  // namespace

TEST(project, a_point_straight_ahead_is_the_middle_of_the_window) {
    float x = 0, y = 0, d = 0;
    REQUIRE(projectPoint(northCamera(), 0, 1000, 0, 1600, 900, 0.0f, x, y, d));
    CHECK(approx(x, 800.0f) && approx(y, 450.0f));
    CHECK(approx(d, 1000.0f));
}

TEST(project, right_is_right_and_up_is_up) {
    float x = 0, y = 0, d = 0;
    REQUIRE(projectPoint(northCamera(), 400, 1000, 0, 1600, 900, 0.0f, x, y, d));   // 0.4 of the view's 0.8 half width: half way to the right edge
    CHECK(approx(x, 800.0f + 400.0f) && approx(y, 450.0f));
    REQUIRE(projectPoint(northCamera(), -800, 1000, 0, 1600, 900, 0.0f, x, y, d));  // the left edge
    CHECK(approx(x, 0.0f));
    REQUIRE(projectPoint(northCamera(), 0, 1000, 225, 1600, 900, 0.0f, x, y, d));   // half way up
    CHECK(approx(x, 800.0f) && approx(y, 225.0f));
}

TEST(project, a_point_behind_the_camera_is_not_drawn) {
    float x = 0, y = 0, d = 0;
    CHECK(!projectPoint(northCamera(), 0, -1000, 0, 1600, 900, 0.0f, x, y, d));
    CHECK(!projectPoint(northCamera(), 0, 0.1f, 0, 1600, 900, 0.0f, x, y, d));      // on the camera plane
}

TEST(project, outside_the_view_is_dropped_but_a_little_off_the_edge_still_shows) {
    float x = 0, y = 0, d = 0;
    CHECK(!projectPoint(northCamera(), 5000, 1000, 0, 1600, 900, 0.05f, x, y, d));
    CHECK(!projectPoint(northCamera(), 0, 1000, 3000, 1600, 900, 0.05f, x, y, d));
    CHECK(projectPoint(northCamera(), 820, 1000, 0, 1600, 900, 0.05f, x, y, d));    // 2.5 per cent past the right edge, inside the 5 per cent margin
    CHECK(x > 1600.0f);
    CHECK(!projectPoint(northCamera(), 900, 1000, 0, 1600, 900, 0.05f, x, y, d));   // 12 per cent past
}

TEST(project, the_cameras_viewport_rectangle_places_the_view_inside_the_window) {
    CameraMatrix c = northCamera();
    c.portL = 0.0f; c.portR = 0.5f;                                                 // the left half of the window (split screen)
    float x = 0, y = 0, d = 0;
    REQUIRE(projectPoint(c, 0, 1000, 0, 1600, 900, 0.0f, x, y, d));
    CHECK(approx(x, 400.0f) && approx(y, 450.0f));
    REQUIRE(projectPoint(c, 800, 1000, 0, 1600, 900, 0.0f, x, y, d));
    CHECK(approx(x, 800.0f));
}

TEST(project, a_camera_turned_to_the_east_and_standing_elsewhere) {
    const float pos[3] = {1000, 2000, 100};
    const float east[3] = {1, 0, 0};
    const CameraMatrix c = makeCamera(pos, east, kUp, 0.8f, 0.45f);
    float x = 0, y = 0, d = 0;
    REQUIRE(projectPoint(c, 1500, 2000, 100, 1600, 900, 0.0f, x, y, d));
    CHECK(approx(x, 800.0f) && approx(y, 450.0f) && approx(d, 500.0f));
    REQUIRE(projectPoint(c, 1500, 1800, 100, 1600, 900, 0.0f, x, y, d));            // south of the line of sight, facing east: to the right
    CHECK(x > 800.0f);
}

TEST(project, bad_input_is_refused) {
    float x = 0, y = 0, d = 0;
    CHECK(!projectPoint(CameraMatrix{}, 0, 1000, 0, 1600, 900, 0.0f, x, y, d));
    CHECK(!projectPoint(northCamera(), std::nanf(""), 1000, 0, 1600, 900, 0.0f, x, y, d));
    CHECK(!projectPoint(northCamera(), 0, 1000, 0, 0, 900, 0.0f, x, y, d));
}

TEST(project, only_a_matrix_that_behaves_like_a_camera_is_used) {
    const CameraMatrix good = northCamera();
    CHECK(plausibleCamera(good, kOrigin, kNorth));
    const float south[3] = {0, -1, 0};
    CHECK(!plausibleCamera(good, kOrigin, south));                                  // the matrix looks north, the axis we were told is south
    CameraMatrix nan = good;
    nan.m[5] = std::nanf("");
    CHECK(!plausibleCamera(nan, kOrigin, kNorth));
    CHECK(!plausibleCamera(CameraMatrix{}, kOrigin, kNorth));
    CameraMatrix zero;
    zero.valid = true;
    CHECK(!plausibleCamera(zero, kOrigin, kNorth));
    CameraMatrix port = good;
    port.portR = port.portL;
    CHECK(!plausibleCamera(port, kOrigin, kNorth));
    const float stretched[3] = {0, 3, 0};
    CHECK(!plausibleCamera(good, kOrigin, stretched));
}

TEST(project, the_scene_camera_has_to_be_near_the_player) {
    const float cam[3] = {100, 100, 100};
    CHECK(cameraNearPlayer(cam, 100, 300, 100, 1500.0f));
    CHECK(!cameraNearPlayer(cam, 100, 5000, 100, 1500.0f));
    CHECK(!cameraNearPlayer(cam, 100, 100, 9000, 1500.0f));                         // far above: the height counts too
    CHECK(!cameraNearPlayer(cam, std::nanf(""), 0, 0, 1500.0f));
}
