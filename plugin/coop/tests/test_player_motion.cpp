// The local player's motion watch (player_motion.h). Game-free. Twins FNVCOOP_MUT=1700..1705: each must make this fail.
//
// The 2026-10-30 playtest said the guest's own player was "constantly running and rubber banding back"; the log had no position and no move of the local
// player in it. These tests pin the instrument that makes the next log conclusive: a set-back is a line (with the caller's context), a move the mod made
// itself is named as the mod's, a cell change is not a set-back, and three set-backs in a few seconds are one RUBBER BAND line.
#include <cstdio>
#include <string>
#include <vector>

#include "player_motion.h"

using namespace nvcoop::coop::motion;

static int g_checks = 0, g_failed = 0;
#define CHECK(c)                                                                  \
    do {                                                                          \
        ++g_checks;                                                               \
        if (!(c)) { ++g_failed; std::printf("  FAIL %s:%d  %s\n", __FILE__, __LINE__, #c); } \
    } while (0)

static Sample S(uint64_t t, float x, float y, float z = 7361, uint32_t cell = 0x103DF9, uint32_t world = 0) {
    Sample s;
    s.nowMs = t;
    s.x = x;
    s.y = y;
    s.z = z;
    s.cell = cell;
    s.world = world;
    return s;
}
static bool has(const std::vector<std::string>& v, const char* needle) {
    for (const auto& l : v)
        if (l.find(needle) != std::string::npos) return true;
    return false;
}

// A player who runs (and sprints) at 60 fps and at 15 fps is never reported.
static void t_running_is_silent() {
    MotionWatch w;
    float x = 1000;
    uint64_t t = 0;
    for (int i = 0; i < 600; ++i) {   // 10 s at 60 fps, sprint speed 350 u/s
        x += 350.0f / 60.0f;
        t += 16;
        CHECK(w.step(S(t, x, 1900), "").empty());
    }
    for (int i = 0; i < 40; ++i) {    // 15 fps: 67 ms per step, 23 units per step, 350 u/s
        x += 23.0f;
        t += 67;
        CHECK(w.step(S(t, x, 1900), "").empty());
    }
    CHECK(w.step(S(t + 16, x, 1900 - 400 * 0.016f), "").empty());   // a fall / a slope: the rise is not counted, nor a step of 6 units
}

// The set-back of the report: 250 units back in one frame, with the caller's context on the line.
static void t_a_set_back_is_a_line_with_its_context() {
    MotionWatch w;
    w.step(S(0, 1941, 1979), "");
    auto o = w.step(S(16, 1700, 1979), "controls 0x00 VCG01 55 puppet 48 u");
    CHECK(o.size() == 1);
    CHECK(has(o, "JUMPED, not by the mod's own moves") && has(o, "241 units in 16 ms"));
    CHECK(has(o, "from (1941,1979,7361) to (1700,1979,7361)"));
    CHECK(has(o, "controls 0x00 VCG01 55 puppet 48 u"));
}

// A move the mod announced (warp, respawn, join warp) is named as the mod's; one that comes much later is not.
static void t_a_mod_move_is_named_as_the_mods() {
    MotionWatch w;
    w.step(S(0, 100, 100), "");
    w.noteModMove(1000, "late join warp to the party's zone");
    auto o = w.step(S(1500, 5000, 100), "");
    CHECK(o.size() == 1 && has(o, "MOVED by the mod") && has(o, "late join warp to the party's zone") && !has(o, "JUMPED"));
    w.step(S(1516, 5000, 100), "");
    w.step(S(8984, 5000, 100), "");
    o = w.step(S(9000, 100, 100), "");                                  // 8 s later, nothing announced
    CHECK(has(o, "JUMPED"));
}

// A door or a travel: a line of its own, never a set-back.
static void t_a_cell_change_is_not_a_set_back() {
    MotionWatch w;
    w.step(S(0, 2400, 1530, 7360, 0x103DF9), "");
    auto o = w.step(S(16, -73100, 1270, 8700, 0xDAEBB), "");
    CHECK(o.size() == 1 && has(o, "changed cell 103DF9 -> DAEBB"));
    CHECK(!has(o, "JUMPED") && w.jumpsInWindow() == 0);
    CHECK(w.step(S(32, -73096, 1270, 8700, 0xDAEBB), "").empty());       // and the next step is measured in the new cell
}

// The symptom itself: three set-backs within a few seconds are one RUBBER BAND line, said once per half minute.
static void t_three_set_backs_are_a_rubber_band() {
    MotionWatch w;
    w.step(S(0, 1000, 1000), "");
    uint64_t t = 0;
    int summaries = 0;
    for (int k = 0; k < 3; ++k) {
        t += 600;
        w.step(S(t, 1000 + 20.0f * float(k) + 5, 1000), "");               // a little progress
        t += 16;
        auto o = w.step(S(t, 700, 1000), "ctx");                           // and pulled back
        if (has(o, "RUBBER BAND")) { ++summaries; CHECK(has(o, "3 times") && has(o, "[ctx]")); }
        t += 16;
        w.step(S(t, 1000, 1000), "");                                      // (the player's own next step back to where he was running is a jump too)
    }
    CHECK(summaries >= 1);
    // after the summary it stays quiet for tugRepeatMs
    int again = 0;
    for (int k = 0; k < 4; ++k) {
        t += 600;
        for (const auto& l : w.step(S(t, 300 + 100.0f * float(k), 1000), "")) again += l.find("RUBBER BAND") != std::string::npos;
    }
    CHECK(again == 0);
}

// Mod moves never add up to a rubber band (they are the design: a warp), and a flood is cut off at the line limit.
static void t_mod_moves_are_no_rubber_band_and_the_log_is_rate_limited() {
    MotionWatch w;
    w.step(S(0, 0, 0), "");
    uint64_t t = 0;
    for (int k = 0; k < 6; ++k) {
        t += 1000;
        w.noteModMove(t, "tether gather");
        auto o = w.step(S(t + 16, 400.0f * float(k + 1), 0), "");
        CHECK(!has(o, "RUBBER BAND"));
    }
    MotionWatch f;
    f.step(S(0, 0, 0), "");
    size_t lines = 0;
    for (int k = 0; k < 200; ++k) {
        const uint64_t tt = 100 * uint64_t(k + 1);
        lines += f.step(S(tt, (k % 2) ? 0.0f : 500.0f, 0), "").size();
    }
    CHECK(lines <= size_t(24) + 3);   // 24 per minute + the RUBBER BAND summaries (30 s apart, 20 s of test time)
}

int main() {
    t_running_is_silent();
    t_a_set_back_is_a_line_with_its_context();
    t_a_mod_move_is_named_as_the_mods();
    t_a_cell_change_is_not_a_set_back();
    t_three_set_backs_are_a_rubber_band();
    t_mod_moves_are_no_rubber_band_and_the_log_is_rate_limited();
    std::printf("player_motion: %d checks, %d failed\n", g_checks, g_failed);
    return g_failed ? 1 : 0;
}
