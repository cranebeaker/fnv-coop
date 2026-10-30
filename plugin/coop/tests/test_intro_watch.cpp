// The opening scene's watcher (intro_watch.h): the log lines for every stage change, the named wake-up steps, the "stalled" line that says what the scene waits
// for, and the one notice for a menu that waits in a window that is not in front. Game-free. Twins FNVCOOP_MUT=1600..1606: each must make this fail.
#include <cstdio>
#include <string>

#include "intro_watch.h"

using namespace nvcoop::coop::introwatch;

static int g_checks = 0, g_failed = 0;
#define CHECK(c)                                                                  \
    do {                                                                          \
        ++g_checks;                                                               \
        if (!(c)) { ++g_failed; std::printf("  FAIL %s:%d  %s\n", __FILE__, __LINE__, #c); } \
    } while (0)

static bool has(const Out& o, const char* needle) {
    for (const auto& l : o.log)
        if (l.find(needle) != std::string::npos) return true;
    return false;
}
static Sample S(uint64_t t, int v0, int v1, bool fg = true, int menu = 0) {
    Sample s;
    s.nowMs = t;
    s.vcg00 = v0;
    s.vcg01 = v1;
    s.foreground = fg;
    s.menu = menu;
    return s;
}

// The host's scene, as the 2026-10-30 log would have shown it: every stage is a line, the menu is named, the wake-up steps are named.
static void t_stage_changes_and_wake_up_are_logged() {
    IntroWatch w;
    Out o = w.step(S(1000, 95, 0));
    CHECK(has(o, "starts watching at VCG00 stage 95, VCG01 stage 0"));
    o = w.step(S(1500, 95, 0));
    CHECK(o.log.empty() && o.notice.empty());                       // nothing changed: nothing said
    o = w.step(S(4000, 100, 0));
    CHECK(has(o, "VCG00 stage 95 -> 100") && has(o, "wake-up:") && has(o, "VCG00 stage 100, the black screen ends"));
    o = w.step(S(9000, 100, 3));
    CHECK(has(o, "VCG01 stage 0 -> 3") && has(o, "waited 5.0 s"));
    o = w.step(S(20000, 100, 10, true, 1051));
    CHECK(has(o, "menu 1051 (name) opened") && has(o, "VCG01 stage 3 -> 10"));
    o = w.step(S(30000, 100, 10, true, 0));
    CHECK(has(o, "menu 1051 (name) closed"));
    o = w.step(S(60000, 100, 45));
    CHECK(has(o, "VCG01 stage 45, Doc's scene stands the player up"));
    o = w.step(S(70000, 100, 55));
    CHECK(has(o, "VCG01 stage 55, the controls come back"));
    o = w.step(S(71000, 100, 56));
    CHECK(!has(o, "wake-up:"));                                     // each step once
}

// A stage that does not change: the line says what it waits for (a menu, or nothing), and a menu that waits in a window that is not in front gets one notice.
static void t_stall_says_what_it_waits_for() {
    IntroWatch w;
    w.step(S(0, 100, 10, false, 1051));
    Out o = w.step(S(10000, 100, 10, false, 1051));
    CHECK(!has(o, "STALLED"));                                       // not long enough yet
    o = w.step(S(21000, 100, 10, false, 1051));
    CHECK(has(o, "STALLED") && has(o, "menu 1051 (name) is open and waits for the player") && has(o, "not in front"));
    CHECK(o.notice.find("Doc is waiting for you here (name)") == 0);
    o = w.step(S(22000, 100, 10, false, 1051));
    CHECK(o.log.empty() && o.notice.empty());                        // said once
    o = w.step(S(82000, 100, 10, false, 1051));
    CHECK(has(o, "STALLED") && o.notice.empty());                    // repeated after a minute, the notice is not
    // no menu: the scene waits on its own timer / a line; the window in front gets no notice
    IntroWatch q;
    q.step(S(0, 100, 8));
    o = q.step(S(25000, 100, 8, true, 0));
    CHECK(has(o, "no menu is open") && o.notice.empty() && !has(o, "not in front"));
    // a menu that waits in the window that IS in front: the line, no notice (the player sees it)
    IntroWatch f;
    f.step(S(0, 100, 36, true, 1036));
    o = f.step(S(25000, 100, 36, true, 1036));
    CHECK(has(o, "menu 1036 (face (reflectron))") && o.notice.empty());
    // a scene that is over (the player walks) or has not begun is never "stalled"
    IntroWatch d;
    d.step(S(0, 100, 55));
    CHECK(!has(d.step(S(90000, 100, 55)), "STALLED"));
    IntroWatch n;
    n.step(S(0, -1, -1));
    CHECK(!has(n.step(S(90000, -1, -1)), "STALLED"));
}

// A new stage gives the next wait a fresh start, and a new stall a fresh notice.
static void t_a_new_stage_rearms_the_stall() {
    IntroWatch w;
    w.step(S(0, 100, 10, false, 1051));
    Out o = w.step(S(21000, 100, 10, false, 1051));
    CHECK(!o.notice.empty());
    w.step(S(30000, 100, 15, false, 0));
    o = w.step(S(60000, 100, 15, false, 0));
    CHECK(has(o, "STALLED") && has(o, "VCG01 stage 15") && o.notice.empty());   // no menu: no notice
    w.step(S(61000, 100, 36, false, 1036));
    o = w.step(S(90000, 100, 36, false, 1036));
    CHECK(has(o, "menu 1036") && o.notice.find("click this window") != std::string::npos);
}

int main() {
    t_stage_changes_and_wake_up_are_logged();
    t_stall_says_what_it_waits_for();
    t_a_new_stage_rearms_the_stall();
    std::printf("intro_watch: %d checks, %d failed\n", g_checks, g_failed);
    return g_failed ? 1 : 0;
}
