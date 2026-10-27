// Game-free tests for the bot input driver (T6a). Built once (FNVCOOP_RUNTIME_MUT); run once for real and once per FNVCOOP_MUT=<n> (1..56); every twin MUST fail.
// Twins 49..56 (bot.vats queue / execute, listed in bot_engine.h): 49 accept never armed, 50 execute with an empty queue, 51 tutorial not
// closed, 52 target cycle skipped, 53 tutorial reported as vats, 54 gate forces every slot, 55 VATSMenu vtable check skipped, 56 hit-chance gate skipped.
// BOTS_EXE=<reference FalloutNV.exe> additionally compares the pinned prologue with the real image (run_tests.cmd sets it when the file exists).
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <fstream>
#include <iterator>
#include <map>
#include <string>
#include <vector>

#include "../bot_rpc.h"
#include "../bots_addrs.h"
#include "fake_game.h"

using namespace nvbots;
using nvbots::Addr32;

static int g_fail = 0, g_checks = 0;
#define CHECK(c)                                                       \
    do {                                                               \
        ++g_checks;                                                    \
        if (!(c)) {                                                    \
            ++g_fail;                                                  \
            std::printf("  FAIL %s:%d  %s\n", __FILE__, __LINE__, #c); \
        }                                                              \
    } while (0)
#define TEST(name) static void name()
static void run(const char* n, void (*f)()) {
    int before = g_fail;
    f();
    std::printf("[%s] %s\n", g_fail == before ? " ok " : "FAIL", n);
}

// ---- helpers: play the game's side of the contract ----
static int edges(ForcedInput& in, int c, int state, int frames) {  // how many of the next `frames` frames see the edge
    int n = 0;
    for (int i = 0; i < frames; ++i) {
        const int a = in.filter(c, state, 0);
        const int b = in.filter(c, state, 0);  // a second query in the same frame must agree
        CHECK(a == b);
        n += a;
        in.advanceFrame();
    }
    return n;
}

TEST(held_overrides_real_input_until_released) {
    ForcedInput in;
    CHECK(in.filter(kForward, kHeld, 0) == 0);
    in.hold(kForward);
    CHECK(in.filter(kForward, kHeld, 0) == 1);
    CHECK(in.filter(kBackward, kHeld, 0) == 0);
    in.advanceFrame();
    CHECK(in.filter(kForward, kHeld, 0) == 1);
    in.release(kForward);
    CHECK(in.filter(kForward, kHeld, 0) == 0);
    CHECK(in.filter(kForward, kHeld, 1) == 1);  // real key still passes through when we are not forcing
}

TEST(press_edge_reaches_the_game_exactly_once) {
    ForcedInput in;
    in.hold(kCrouch);
    // the game does not ask during the frame we issued it; the edge is served in the first frame it asks, once
    in.advanceFrame();
    CHECK(edges(in, kCrouch, kPressed, 5) == 1);
    CHECK(in.isHeld(kCrouch));
}

TEST(press_edge_same_frame_is_consistent) {
    ForcedInput in;
    in.hold(kJump);
    CHECK(in.filter(kJump, kPressed, 0) == 1);
    CHECK(in.filter(kJump, kPressed, 0) == 1);
    in.advanceFrame();
    CHECK(in.filter(kJump, kPressed, 0) == 0);
}

TEST(release_edge_once) {
    ForcedInput in;
    in.hold(kActivate);
    in.advanceFrame();
    edges(in, kActivate, kPressed, 2);
    in.release(kActivate);
    CHECK(edges(in, kActivate, kReleased, 4) == 1);
    CHECK(!in.isHeld(kActivate));
}

TEST(tap_holds_n_frames_then_lets_go) {
    ForcedInput in;
    in.tap(kAttack, 3);
    for (int i = 0; i < 2; ++i) {
        CHECK(in.isHeld(kAttack));
        in.advanceFrame();
    }
    CHECK(in.isHeld(kAttack));
    in.advanceFrame();
    CHECK(!in.isHeld(kAttack));
    for (int i = 0; i < 20; ++i) in.advanceFrame();
    CHECK(!in.isHeld(kAttack));
}

TEST(timed_hold_expires) {
    ForcedInput in;
    in.hold(kRun, 2);
    in.advanceFrame();
    in.advanceFrame();
    CHECK(!in.isHeld(kRun));
}

TEST(release_all_clears_everything) {
    ForcedInput in;
    in.hold(kForward);
    in.hold(kAttack);
    in.hold(kJump);
    CHECK(in.heldCount() == 3);
    in.releaseAll();
    CHECK(in.heldCount() == 0);
    CHECK(in.filter(kForward, kHeld, 0) == 0);
}

TEST(out_of_range_controls_are_never_forced) {
    ForcedInput in;
    in.hold(kCount);
    in.hold(-1);
    in.hold(0x40);
    CHECK(in.heldCount() == 0);
    CHECK(in.filter(kCount, kHeld, 0) == 0);
    CHECK(in.filter(0x1D, kPressed, 0) == 0);
    CHECK(in.filter(-3, kHeld, 1) == 1);
}

TEST(axes_map_to_the_right_controls) {
    ForcedInput in;
    in.setMove(1, 0, false);
    CHECK(in.isHeld(kForward) && !in.isHeld(kBackward));
    in.setMove(-1, 0, false);
    CHECK(in.isHeld(kBackward) && !in.isHeld(kForward));
    in.setMove(0, 1, false);
    CHECK(in.isHeld(kRight) && !in.isHeld(kLeft));
    in.setMove(0, -1, false);
    CHECK(in.isHeld(kLeft) && !in.isHeld(kRight));
    in.setMove(0.2, -0.2, false);  // inside the dead zone
    CHECK(in.heldCount() == 0);
    in.setMove(1, 1, true);
    CHECK(in.isHeld(kForward) && in.isHeld(kRight) && in.isHeld(kRun));
    in.stopMove();
    CHECK(in.heldCount() == 0);
}

// ---- RPC layer over a mock engine ----
struct MockEngine : IBotEngine {
    uint64_t now = 1000;
    std::vector<std::string> lines;
    std::map<std::string, std::vector<double>> refs;
    bool isHooked = true, refuse = false;
    uint64_t nowMs() override { return now; }
    bool exec(const std::string& l) override { lines.push_back(l); return !refuse; }
    bool pos(const std::string& r, double* x, double* y, double* z) override {
        auto it = refs.find(r);
        if (it == refs.end()) return false;
        *x = it->second[0]; *y = it->second[1]; *z = it->second[2];
        return true;
    }
    bool hooked() override { return isHooked; }
};
struct Rig {
    MockEngine e;
    ForcedInput in;
    BotRpc rpc{e, in};
    std::string out;
    bool call(const char* m, const char* p = "") { return rpc.handle(m, p, out); }
};

TEST(rpc_move_with_deadline_stops_itself) {
    Rig r;
    CHECK(r.call("bot.move", "{\"forward\":1,\"strafe\":0,\"run\":true,\"ms\":500}"));
    CHECK(r.in.isHeld(kForward) && r.in.isHeld(kRun));
    r.e.now += 400; r.rpc.tick();
    CHECK(r.in.isHeld(kForward));
    r.e.now += 200; r.rpc.tick();
    CHECK(!r.in.isHeld(kForward) && !r.in.isHeld(kRun));
}

TEST(rpc_stop_releases_every_control) {
    Rig r;
    r.call("bot.move", "{\"forward\":1}");
    r.call("bot.input", "{\"hold\":[\"Jump\",\"Attack\"]}");
    CHECK(r.in.heldCount() == 3);
    CHECK(r.call("bot.stop"));
    CHECK(r.in.heldCount() == 0);
}

TEST(rpc_look_at_ref_uses_heading_and_pitch_rules) {
    Rig r;
    r.e.refs[""] = {0, 0, 0};
    r.e.refs["FalloutNV.esm:100"] = {0, 100, 0};   // due +Y, 60 below eye level (eye 120, target 60)
    CHECK(r.call("bot.look", "{\"ref\":\"FalloutNV.esm:100\"}"));
    CHECK(r.e.lines.size() == 2);
    CHECK(r.e.lines[0] == "player.SetAngle Z 0.00");
    CHECK(r.e.lines[1].rfind("player.SetAngle X 30.9", 0) == 0);   // looking down is a positive X angle
    r.e.refs["east"] = {100, 0, 120 - 60};
    r.e.lines.clear();
    CHECK(r.call("bot.look", "{\"ref\":\"east\"}"));
    CHECK(r.e.lines[0] == "player.SetAngle Z 90.00");
    CHECK(std::fabs(std::atof(r.e.lines[1].c_str() + 18)) < 0.01);
}

TEST(rpc_look_errors_are_reported) {
    Rig r;
    r.e.refs[""] = {0, 0, 0};
    CHECK(!r.call("bot.look", "{\"ref\":\"nope\"}"));
    CHECK(r.out.find("not loaded") != std::string::npos);
    r.e.refuse = true;
    CHECK(!r.call("bot.look", "{\"yaw\":10,\"pitch\":0}"));
}

TEST(rpc_fire_pulses_the_attack_control_n_times) {
    Rig r;
    CHECK(r.call("bot.fire", "{\"shots\":3,\"gapMs\":300}"));
    CHECK(r.out.find("\"queued\":3") != std::string::npos);
    int presses = 0;
    for (int i = 0; i < 60 && (r.rpc.shotsLeft() > 0 || r.in.isHeld(kAttack)); ++i) {
        r.e.now += 50;
        r.rpc.tick();
        presses += r.in.filter(kAttack, kPressed, 0);
    }
    CHECK(presses == 3);
    CHECK(r.rpc.shotsLeft() == 0 && !r.in.isHeld(kAttack));
    CHECK(!r.call("bot.fire", "{\"shots\":0}"));
}

TEST(rpc_taps_and_generic_input) {
    Rig r;
    CHECK(r.call("bot.jump"));
    CHECK(r.in.isHeld(kJump));
    CHECK(r.call("bot.sneak") && r.in.isHeld(kCrouch));
    CHECK(r.call("bot.reload") && r.in.isHeld(kReadyItem));
    CHECK(r.call("bot.activate") && r.in.isHeld(kActivate));
    CHECK(!r.call("bot.input", "{\"hold\":[\"Bogus\"]}"));
    CHECK(!r.call("bot.input", "{\"hold\":[99]}"));
    CHECK(r.call("bot.input", "{\"hold\":[0,\"Vats\"],\"frames\":2}"));
    CHECK(r.in.isHeld(kForward) && r.in.isHeld(kVats));
}

TEST(rpc_status_and_unwired_methods) {
    Rig r;
    CHECK(r.call("bot.status"));
    CHECK(r.out.find("\"hooked\":true") != std::string::npos);
    r.e.isHooked = false;
    r.call("bot.status");
    CHECK(r.out.find("\"hooked\":false") != std::string::npos);
    CHECK(!r.call("bot.menu") && r.out.find("unsupported") != std::string::npos);
    CHECK(!r.call("bot.say", "{\"info\":\"x\"}"));
    CHECK(!r.call("bot.nope"));
    CHECK(!r.call("bot.move", "{bad json"));
}

// ---- engine glue: menus, dialogue, inventory, VATS, refs (synthetic game memory) ----
using nvbots_test::FakeGame;

struct GlueEngine : MockEngine {
    FakeGame* g = nullptr;
    bool helpers = true;   // env NVCOOP_TEST_HELPERS=1
    int pressed = -1, uiTries = 0;
    bool uiBox = true;   // the UI layer (ui.answer) is showing a box
    std::string moved;
    IEngine* engine() override { return g; }
    int alwaysRun() override { return g ? alwaysRunState(*g) : -1; }
    bool pressBox(int b, uint32_t* menu, std::string* err) override {
        ++uiTries;
        if (!uiBox) { *err = "no message box is being shown"; return false; }
        pressed = b;
        *menu = 900;
        return true;
    }
    bool moveTo(const std::string& r, std::string*) override { moved = r; return true; }
    bool testHelpers() override { return helpers; }
};
struct GlueRig {
    FakeGame g;
    GlueEngine e;
    ForcedInput in;
    BotRpc rpc{e, in};
    std::string out;
    GlueRig() { e.g = &g; }
    bool call(const char* m, const char* p = "") { return rpc.handle(m, p, out); }
};

TEST(menu_kinds_and_vats_priority) {
    GlueRig r;
    r.g.buildMenus();
    r.g.topId = 0x3F1;
    CHECK(r.call("bot.menu") && r.out.find("\"kind\":\"dialogue\"") != std::string::npos);
    r.g.topId = 0x3F0;
    CHECK(r.call("bot.menu") && r.out.find("\"kind\":\"container\"") != std::string::npos);
    r.g.topId = 0x421;
    CHECK(r.call("bot.menu") && r.out.find("\"kind\":\"terminal\"") != std::string::npos);
    r.g.topId = 0;
    CHECK(r.call("bot.menu") && r.out.find("\"kind\":\"none\"") != std::string::npos);
    r.g.topId = 0x3EC;
    r.g.set32(nvbots::eng::kVatsCamera + nvbots::eng::kVatsMode, 1);
    CHECK(r.call("bot.menu") && r.out.find("\"kind\":\"vats\"") != std::string::npos);
    r.g.topId = 0x420;
    CHECK(r.call("bot.menu") && r.out.find("\"kind\":\"vats\"") != std::string::npos && r.out.find("\"id\":1056") != std::string::npos);
    // 1059 = 0x423 is TutorialMenu, not VATS: a VATS camera mode never hides it (VATS input stops while it is open)
    r.g.topId = 0x423;
    CHECK(r.call("bot.menu") && r.out.find("\"kind\":\"tutorial\"") != std::string::npos && r.out.find("\"id\":1059") != std::string::npos);
    r.g.set32(nvbots::eng::kVatsCamera + nvbots::eng::kVatsMode, 0);
    CHECK(r.call("bot.menu") && r.out.find("\"kind\":\"tutorial\"") != std::string::npos);
    CHECK(std::string(menuKindOf(0x423)) == "tutorial" && std::string(menuKindOf(0x420)) == "vats");
    FakeGame bare;
    GlueRig r2;
    CHECK(!r2.call("bot.menu") && r2.out.find("not ready") != std::string::npos);   // manager pointer missing
}

TEST(dialogue_state_is_reported) {
    GlueRig r;
    r.g.buildMenus();
    r.g.topId = 0x3F1;
    r.g.buildDialogue(2, 2);
    CHECK(r.call("bot.menu") && r.out.find("\"dialogReady\":true") != std::string::npos);
    r.g.set32(FakeGame::kDlg + nvbots::eng::kDlgState, 1);
    CHECK(r.call("bot.menu") && r.out.find("\"dialogReady\":false") != std::string::npos);
}

TEST(dialogue_list_and_pick_run_the_click_sequence) {
    GlueRig r;
    r.g.buildDialogue(3);
    r.g.formIds["FalloutNV.esm:1001"] = 0x1001;
    CHECK(r.call("bot.dialog"));
    CHECK(r.out.find("\"idx\":2") != std::string::npos);
    r.g.calls.clear();
    CHECK(r.call("bot.say", "{\"info\":\"FalloutNV.esm:1001\"}"));
    CHECK(r.out.find("\"index\":1") != std::string::npos && r.out.find("\"nextState\":3") != std::string::npos);
    // the order matters: select the row, clear the item flag, item click, list after-click, record the INFO, speak
    std::string seq;
    for (auto& c : r.g.calls) seq += c + ";";
    const size_t sel = seq.rfind("select 1;");
    const size_t clk = seq.find("click ");
    const size_t aft = seq.find("after;");
    const size_t spk = seq.find("speak 0;");
    CHECK(sel != std::string::npos && clk != std::string::npos && aft != std::string::npos && spk != std::string::npos);
    CHECK(sel < clk && clk < aft && aft < spk);
    CHECK(r.g.get32(nvbots::eng::kInfoSpoken) == FakeGame::kInfos + 0x100);
    CHECK(r.g.get32(FakeGame::kDlg + nvbots::eng::kDlgState) == 3);
    uint8_t flag = 9;
    r.g.rd8(FakeGame::kItems + 0x100 + nvbots::eng::kItemFlag, &flag);
    CHECK(flag == 0);
}

TEST(dialogue_pick_refuses_when_not_ready_gated_missing_or_special) {
    GlueRig r;
    r.g.formIds["FalloutNV.esm:1000"] = 0x1000;
    r.g.formIds["FalloutNV.esm:9999"] = 0x9999;
    r.g.buildDialogue(2, 1);   // line still playing
    CHECK(!r.call("bot.say", "{\"info\":\"FalloutNV.esm:1000\"}") && r.out.find("not ready") != std::string::npos);
    r.g.buildDialogue(2, 2);
    r.g.set32(FakeGame::kDlg + nvbots::eng::kDlgClickGate, r.g.tick - 100);   // clicked 100 ms ago
    CHECK(!r.call("bot.say", "{\"info\":\"FalloutNV.esm:1000\"}") && r.out.find("gate") != std::string::npos);
    r.g.set32(FakeGame::kDlg + nvbots::eng::kDlgClickGate, r.g.tick - 900);   // long ago: allowed, and the gate is cleared
    CHECK(r.call("bot.say", "{\"info\":\"FalloutNV.esm:1000\"}"));
    CHECK(r.g.get32(FakeGame::kDlg + nvbots::eng::kDlgClickGate) == 0);
    r.g.buildDialogue(2, 2);
    CHECK(!r.call("bot.say", "{\"info\":\"FalloutNV.esm:9999\"}") && r.out.find("not offered") != std::string::npos);
    CHECK(r.out.find("4096") != std::string::npos);   // lists what IS offered (0x1000)
    r.g.special[FakeGame::kInfos] = true;
    r.g.calls.clear();
    CHECK(!r.call("bot.say", "{\"info\":\"FalloutNV.esm:1000\"}") && r.out.find("special") != std::string::npos);
    for (auto& c : r.g.calls) CHECK(c.rfind("click", 0) != 0 && c.rfind("speak", 0) != 0);   // nothing was poked
    CHECK(!r.call("bot.say", "{\"info\":\"nope\"}"));
    GlueRig none;
    CHECK(!none.call("bot.say", "{\"info\":\"x\"}"));
}

TEST(dialogue_pick_allows_speech_checks_and_says_so) {
    GlueRig r;
    r.g.formIds["FalloutNV.esm:1000"] = 0x1000;
    r.g.formIds["FalloutNV.esm:1001"] = 0x1001;
    r.g.buildDialogue(2, 2);
    r.g.speechCheck[FakeGame::kInfos] = true;
    r.g.calls.clear();
    CHECK(r.call("bot.say", "{\"info\":\"FalloutNV.esm:1000\"}"));
    CHECK(r.out.find("\"speechCheck\":true") != std::string::npos && r.out.find("\"said\":true") != std::string::npos);
    bool spoke = false;
    for (auto& c : r.g.calls) spoke = spoke || c.rfind("speak", 0) == 0;
    CHECK(spoke);   // the engine's own speak path ran: it rolls the check and runs the INFO's results
    r.g.buildDialogue(2, 2);
    CHECK(r.call("bot.say", "{\"info\":\"FalloutNV.esm:1001\"}") && r.out.find("\"speechCheck\":false") != std::string::npos);
}

TEST(inventory_reports_totals_not_deltas) {
    GlueRig r;
    const uint32_t kStim = 0x50000000, kAmmo = 0x50000100;
    r.g.forms[""] = 0;
    r.g.buildInventory({{kStim, 2}, {kAmmo, -1}});   // entries are deltas against the base container
    r.g.totals[kStim] = 7;                           // the engine's own total: base 5 + delta 2
    r.g.totals[kAmmo] = 24;                          // base 25 + delta -1
    CHECK(r.call("bot.inventory"));
    CHECK(r.out.find(":50000000\",\"count\":7") != std::string::npos);
    CHECK(r.out.find(":50000100\",\"count\":24") != std::string::npos);
    CHECK(r.out.find("\"partial\":true") != std::string::npos);
    r.g.forms["FalloutNV.esm:BEEF"] = 0x50000200;
    r.g.totals[0x50000200] = 3;
    CHECK(r.call("bot.inventory", "{\"items\":[\"FalloutNV.esm:BEEF\"]}"));
    CHECK(r.out.find("\"count\":3") != std::string::npos && r.out.find("\"partial\":false") != std::string::npos);
    CHECK(!r.call("bot.inventory", "{\"items\":[\"Nope.esm:1\"]}"));
    CHECK(!r.call("bot.inventory", "{\"ref\":\"Nope.esm:1\"}") && r.out.find("not loaded") != std::string::npos);
}

TEST(inventory_empty_container) {
    GlueRig r;
    r.g.buildInventory({});
    CHECK(r.call("bot.inventory") && r.out.find("\"items\":[]") != std::string::npos);
}

// One game frame per tick: the job ticks (BotRpc::tick), then the fake engine plays the frame with the Vats press edge the job made.
// Returns the phases seen, in order, without repeats.
static std::vector<std::string> vatsDrive(GlueRig& r, int maxTicks = 700) {
    std::vector<std::string> seen;
    for (int i = 0; i < maxTicks && r.rpc.vats().running(); ++i) {
        r.e.now += 50;
        r.rpc.tick();
        const std::string ph = VatsJob::name(r.rpc.vats().phase());
        if (seen.empty() || seen.back() != ph) seen.push_back(ph);
        r.g.vatsFrame(r.in.filter(kVats, kPressed, 0) != 0);
    }
    return seen;
}
static bool hasPhase(const std::vector<std::string>& seen, const char* p) {
    for (const std::string& s : seen)
        if (s == p) return true;
    return false;
}
static bool reasonHas(GlueRig& r, const char* s) { return r.rpc.vats().reason().find(s) != std::string::npos; }

TEST(vats_job_closes_the_tutorial_cycles_queues_and_executes) {
    GlueRig r;
    r.g.forms["FalloutNV.esm:603"] = 0x60000003;
    r.g.buildVats({0x60000001, 0x60000002, 0x60000003}, 55.0f, 60.0f, /*tutorial*/ true);
    r.g.pickFrames = 2;   // the mouse pick ends Update twice: the accept must stay armed over frames
    CHECK(r.call("bot.vats", "{\"ref\":\"FalloutNV.esm:603\"}"));
    CHECK(!r.call("bot.vats", "{\"ref\":\"FalloutNV.esm:603\"}"));   // second start while running is refused
    r.call("bot.status");
    CHECK(r.out.find("\"phase\":\"enter\"") != std::string::npos);
    int otherTaps = 0;
    std::vector<std::string> seen;
    for (int i = 0; i < 700 && r.rpc.vats().running(); ++i) {
        r.e.now += 50;
        r.rpc.tick();
        const std::string ph = VatsJob::name(r.rpc.vats().phase());
        if (seen.empty() || seen.back() != ph) seen.push_back(ph);
        for (int c = 0; c < kCount; ++c)
            if (c != kVats && r.in.filter(c, kPressed, 0)) ++otherTaps;
        r.g.vatsFrame(r.in.filter(kVats, kPressed, 0) != 0);
    }
    CHECK(r.rpc.vats().phase() == VatsJob::kDone);
    if (r.rpc.vats().phase() != VatsJob::kDone) std::printf("  reason: %s\n", r.rpc.vats().reason().c_str());
    CHECK(hasPhase(seen, "tutorial") && hasPhase(seen, "zoom") && hasPhase(seen, "target") && hasPhase(seen, "queue") && hasPhase(seen, "playback"));
    CHECK(r.g.closeCalls >= 1 && !r.g.tutorialOpen);                              // the tutorial box was closed through 0x7E8D60
    CHECK(r.g.clicks == 2 && r.rpc.vats().cycles() == 2);                         // two next-target clicks: 1 -> 2 -> 3
    CHECK(r.g.queuedTargets.size() == 1 && r.g.queuedTargets[0] == 0x60000003);   // the engine queued exactly one attack, on the wanted target
    CHECK(r.g.arms == 1 && !r.g.gate.armed() && r.g.gate.forced() >= 3);          // armed once, kept over the two picked frames, disarmed
    CHECK(r.g.wrongForces == 0);
    CHECK(r.g.execCalls == 1 && r.g.execEmpty == 0);
    CHECK(otherTaps == 0);                                                        // no Right / Activate taps: VATS never reads them
    bool armBeforeExec = false, execSeen = false;
    for (const std::string& c : r.g.calls) {
        if (c == "arm" && !execSeen) armBeforeExec = true;
        if (c.rfind("vatsexec", 0) == 0) { execSeen = true; CHECK(c == "vatsexec 1"); }
    }
    CHECK(armBeforeExec && execSeen);
    r.call("bot.status");
    CHECK(r.out.find("\"phase\":\"done\"") != std::string::npos);
}

TEST(vats_job_without_tutorial_on_the_default_target) {
    GlueRig r;
    r.g.forms["T:1"] = 0x60000001;
    r.g.buildVats({0x60000001, 0x60000002});
    CHECK(r.call("bot.vats", "{\"ref\":\"T:1\"}"));
    const std::vector<std::string> seen = vatsDrive(r);
    CHECK(r.rpc.vats().phase() == VatsJob::kDone);
    CHECK(r.g.closeCalls == 0 && r.g.clicks == 0 && r.g.execCalls == 1);
    CHECK(r.g.queuedTargets.size() == 1 && r.g.queuedTargets[0] == 0x60000001);
    CHECK(hasPhase(seen, "zoom") && hasPhase(seen, "playback"));
}

TEST(vats_job_closes_a_tutorial_box_that_opens_late) {
    // In game 2026-10-21: the VATS tutorial box (1059) opened after the job had passed its tutorial step, VATSMenu::Update stopped and the
    // job read a 0 hit chance. A box that opens in a later step must send the job back to the tutorial step.
    GlueRig r;
    r.g.forms["T:1"] = 0x60000001;
    r.g.buildVats({0x60000001, 0x60000002});
    CHECK(r.call("bot.vats", "{\"ref\":\"T:1\"}"));
    bool opened = false;
    std::vector<std::string> seen;
    for (int i = 0; i < 700 && r.rpc.vats().running(); ++i) {
        r.e.now += 50;
        r.rpc.tick();
        const std::string ph = VatsJob::name(r.rpc.vats().phase());
        if (seen.empty() || seen.back() != ph) seen.push_back(ph);
        if (!opened && (ph == "target" || ph == "queue")) {   // the box appears after the zoom
            r.g.tutorialOpen = true;
            r.g.topId = 0x423;
            opened = true;
        }
        r.g.vatsFrame(r.in.filter(kVats, kPressed, 0) != 0);
    }
    CHECK(opened);
    CHECK(r.rpc.vats().phase() == VatsJob::kDone);
    if (r.rpc.vats().phase() != VatsJob::kDone) std::printf("  reason: %s\n", r.rpc.vats().reason().c_str());
    CHECK(r.g.closeCalls >= 1 && !r.g.tutorialOpen);
    CHECK(r.g.queuedTargets.size() == 1 && r.g.execCalls == 1);
}

TEST(vats_job_keeps_an_attack_queued_before_a_late_tutorial_box) {
    // PR #27 review: the accept was armed, the engine queued the attack, then the tutorial box opened before the job saw the queue.
    // After the box the job must execute that attack, not queue a second one.
    GlueRig r;
    r.g.forms["T:1"] = 0x60000001;
    r.g.buildVats({0x60000001, 0x60000002});
    CHECK(r.call("bot.vats", "{\"ref\":\"T:1\"}"));
    bool opened = false;
    for (int i = 0; i < 700 && r.rpc.vats().running(); ++i) {
        r.e.now += 50;
        r.rpc.tick();
        const bool vatsTap = r.in.filter(kVats, kPressed, 0) != 0;
        const size_t before = r.g.queuedTargets.size();
        r.g.vatsFrame(vatsTap);
        if (!opened && r.g.queuedTargets.size() > before) {   // the frame that queued: the box opens before the job's next tick
            r.g.tutorialOpen = true;
            r.g.topId = 0x423;
            opened = true;
        }
    }
    CHECK(opened);
    CHECK(r.rpc.vats().phase() == VatsJob::kDone);
    if (r.rpc.vats().phase() != VatsJob::kDone) std::printf("  reason: %s\n", r.rpc.vats().reason().c_str());
    CHECK(r.g.queuedTargets.size() == 1);   // one attack, not two
    CHECK(r.g.execCalls == 1 && r.g.closeCalls >= 1);
}

TEST(vats_job_refuses_zero_hit_chance_and_zero_ap) {
    {   // hit chance 0: refused before the accept is armed
        GlueRig r;
        r.g.forms["T:1"] = 0x60000001;
        r.g.buildVats({0x60000001}, 0.0f, 60.0f);
        CHECK(r.call("bot.vats", "{\"ref\":\"T:1\"}"));
        vatsDrive(r);
        CHECK(r.rpc.vats().phase() == VatsJob::kFailed && reasonHas(r, "attack_not_queued") && reasonHas(r, "hit chance 0.0"));
        CHECK(r.g.arms == 0 && r.g.execCalls == 0);
    }
    {   // no AP left
        GlueRig r;
        r.g.forms["T:1"] = 0x60000001;
        r.g.buildVats({0x60000001}, 55.0f, 0.0f);
        CHECK(r.call("bot.vats", "{\"ref\":\"T:1\"}"));
        vatsDrive(r);
        CHECK(r.rpc.vats().phase() == VatsJob::kFailed && reasonHas(r, "no AP left"));
        CHECK(r.g.arms == 0 && r.g.execCalls == 0);
    }
    {   // AP left but less than the cost: the engine queues nothing; the job times out, says why, and disarms
        GlueRig r;
        r.g.forms["T:1"] = 0x60000001;
        r.g.buildVats({0x60000001}, 40.0f, 10.0f);
        CHECK(r.call("bot.vats", "{\"ref\":\"T:1\"}"));
        vatsDrive(r);
        CHECK(r.rpc.vats().phase() == VatsJob::kFailed && reasonHas(r, "attack_not_queued (hit chance 40.0, AP 10.0"));
        CHECK(r.g.arms == 1 && !r.g.gate.armed() && r.g.execCalls == 0);
    }
}

TEST(vats_job_never_executes_an_empty_queue) {
    GlueRig r;
    r.g.forms["T:1"] = 0x60000001;
    r.g.buildVats({0x60000001});
    r.g.undoAfterQueue = true;   // the queued attack is gone before the execute step
    CHECK(r.call("bot.vats", "{\"ref\":\"T:1\"}"));
    vatsDrive(r);
    CHECK(r.rpc.vats().phase() == VatsJob::kFailed && reasonHas(r, "queue_empty_at_execute"));
    CHECK(r.g.execCalls == 0 && r.g.execEmpty == 0);
}

TEST(vats_job_fails_clearly) {
    {   // never enters
        GlueRig r;
        r.g.buildMenus();
        r.g.forms["T:1"] = 0x60000001;
        CHECK(r.call("bot.vats", "{\"ref\":\"T:1\"}"));
        vatsDrive(r);
        CHECK(r.rpc.vats().phase() == VatsJob::kFailed && r.rpc.vats().reason().find("vats_not_entered") == 0);
    }
    {   // target never becomes selectable
        GlueRig r;
        r.g.forms["T:1"] = 0x60000001;
        r.g.buildVats({0x60000008, 0x60000009});
        CHECK(r.call("bot.vats", "{\"ref\":\"T:1\"}"));
        vatsDrive(r);
        CHECK(r.rpc.vats().phase() == VatsJob::kFailed);
        CHECK(r.rpc.vats().reason().find("target_not_selectable") == 0);
        CHECK(r.g.clicks == VatsJob::kMaxCycles && r.g.arms == 0 && r.g.execCalls == 0);
    }
    {   // the tutorial box does not close
        GlueRig r;
        r.g.forms["T:1"] = 0x60000001;
        r.g.buildVats({0x60000001}, 55.0f, 60.0f, true);
        r.g.tutorialCloses = false;
        CHECK(r.call("bot.vats", "{\"ref\":\"T:1\"}"));
        vatsDrive(r);
        CHECK(r.rpc.vats().phase() == VatsJob::kFailed && r.rpc.vats().reason().find("tutorial_box_stays_open") == 0);
        CHECK(r.g.closeCalls >= 1 && r.g.arms == 0 && r.g.execCalls == 0);
    }
    {   // the VATS menu pointer holds another object: no click, no arm
        GlueRig r;
        r.g.forms["T:1"] = 0x60000001;
        r.g.buildVats({0x60000002, 0x60000001}, 55.0f, 60.0f, false, 0x107566C);
        CHECK(r.call("bot.vats", "{\"ref\":\"T:1\"}"));
        vatsDrive(r);
        CHECK(r.rpc.vats().phase() == VatsJob::kFailed && r.rpc.vats().reason().find("vats_menu_wrong_type") == 0);
        CHECK(r.g.clicks == 0 && r.g.arms == 0 && r.g.execCalls == 0);
    }
    {   // the attack-button detour is not installed
        GlueRig r;
        r.g.forms["T:1"] = 0x60000001;
        r.g.buildVats({0x60000001});
        r.g.acceptHook = false;
        CHECK(r.call("bot.vats", "{\"ref\":\"T:1\"}"));
        vatsDrive(r);
        CHECK(r.rpc.vats().phase() == VatsJob::kFailed && r.rpc.vats().reason().find("accept_hook_missing") == 0);
        CHECK(r.g.execCalls == 0);
    }
    {   // limb choice is explicitly unsupported, an unloaded target and an already-active VATS are refused
        GlueRig r;
        r.g.buildMenus();
        r.g.forms["T:1"] = 0x60000001;
        CHECK(!r.call("bot.vats", "{\"ref\":\"T:1\",\"limb\":\"head\"}") && r.out.find("limb") != std::string::npos);
        CHECK(!r.call("bot.vats", "{\"ref\":\"Nope:1\"}"));
        r.g.set32(nvbots::eng::kVatsCamera + nvbots::eng::kVatsMode, 1);
        CHECK(!r.call("bot.vats", "{\"ref\":\"T:1\"}") && r.out.find("already active") != std::string::npos);
        CHECK(r.call("test.vats.hit", "{\"ref\":\"T:1\"}") == false);   // same entry point, same refusal
    }
}

TEST(vats_accept_gate_forces_only_the_slot_1_press_edge) {
    VatsAcceptGate g;
    CHECK(!g.force(1, 0, 0));   // not armed: the original answers
    g.arm(2);
    CHECK(!g.force(2, 0, 2) && !g.force(3, 0, 2));   // special-attack slots R / F are never forced
    CHECK(!g.force(1, 1, 2));                        // nor the held query
    CHECK(g.force(1, 0, 2) && g.force(1, 0, 2));     // the press edge of slot 1, on every frame until the queue grows
    CHECK(g.forced() == 2 && g.armed());
    CHECK(!g.force(1, 0, 3) && !g.armed());          // the queue grew: the gate disarms itself
    CHECK(!g.force(1, 0, 3));
    g.arm(0);
    g.disarm();
    CHECK(!g.force(1, 0, 0));
}

TEST(vats_pins_agree) {
    CHECK(addr::VatsMenuSlotHandleClick.addr == addr::VatsMenuVtbl + addr::kMenuSlotHandleClick);
    CHECK(addr::VatsMenuSlotHandleClick.value == addr::VatsMenuHandleClick.addr);
    CHECK(addr::VatsMenuVtbl == 0x10700D4 && addr::VatsMenuHandleClick.addr == 0x7EECB0 && addr::VatsMenuHandleClick.ret == 8);
    CHECK(addr::VatsMenuSlotGetId.addr == 0x1070108 && addr::VatsMenuSlotGetId.value == 0x720100);
    CHECK(addr::VatsAttackButton.target == 0x7EEB50 && addr::VatsAttackButton.retBytes == 0 && nvhooks::specIsValid(addr::VatsAttackButton));
    CHECK(addr::GetMenuByType.addr == 0xA09030 && addr::TutorialClose.addr == 0x7E8D60 && addr::VatsQueueCount.addr == 0x5AE380 &&
          addr::VatsExecute.addr == 0x705780);
    // the job's constants are the RE addresses
    CHECK(eng::kVatsSelTarget == 0x11F21CC && eng::kVatsMenuPtr == 0x11DB0D4 && eng::kVatsMenuPart == 0x100 && eng::kPartHitChance == 0x28 &&
          eng::kVatsMenuAp == 0xE0 && eng::kMenuIdVats == 0x420 && eng::kMenuIdTutorial == 0x423);
    int listed = 0;
    for (const addr::CallPin* c : addr::kAllCalls)
        listed += c == &addr::GetMenuByType || c == &addr::TutorialClose || c == &addr::VatsMenuHandleClick || c == &addr::VatsQueueCount || c == &addr::VatsExecute;
    CHECK(listed == 5);
    int data = 0;
    for (const addr::DataPin* d : addr::kAllData) data += d == &addr::VatsMenuSlotHandleClick || d == &addr::VatsMenuSlotGetId;
    CHECK(data == 2);
    bool hooked = false;
    for (const nvhooks::HookSpec* h : addr::kAllHooks) hooked = hooked || h == &addr::VatsAttackButton;
    CHECK(hooked);
}

TEST(consent_and_press_answer_the_right_button) {
    GlueRig r;
    CHECK(r.call("bot.consent", "{\"answer\":\"ready\"}") && r.e.pressed == 0);
    CHECK(r.call("bot.consent", "{\"answer\":\"notyet\"}") && r.e.pressed == 1);
    CHECK(!r.call("bot.consent", "{\"answer\":\"maybe\"}"));
    CHECK(r.call("bot.menu.press", "{\"button\":3}") && r.e.pressed == 3);
    CHECK(!r.call("bot.menu.press", "{\"button\":-1}"));
    CHECK(!r.call("bot.menu.press", "{}"));
    Rig plain;   // an engine with no answer path says so
    CHECK(!plain.call("bot.consent", "{\"answer\":\"ready\"}") && plain.out.find("unsupported") != std::string::npos);
}

// ---- bot.menu.press native path: the engine's own MessageMenu (1001) when the UI layer has no box ----
static bool anyMsgClick(const FakeGame& g) {
    for (const std::string& c : g.calls)
        if (c.rfind("msgclick", 0) == 0) return true;
    return false;
}
static std::string msgClick(Addr32 tile) { return "msgclick " + FakeGame::hex(FakeGame::kMsgMenu) + " 7 " + FakeGame::hex(tile); }
static std::string lastCall(const FakeGame& g) { return g.calls.empty() ? std::string() : g.calls.back(); }

TEST(native_message_box_is_pressed_when_the_ui_layer_has_none) {
    GlueRig r;
    r.e.uiBox = false;
    r.g.buildMessageBox({0, 1, 2});
    CHECK(r.call("bot.menu.press", "{\"button\":1}"));
    CHECK(r.out.find("\"path\":\"native\"") != std::string::npos && r.out.find("\"menu\":1001") != std::string::npos);
    CHECK(r.out.find("\"button\":1") != std::string::npos && r.out.find("\"value\":1") != std::string::npos && r.out.find("\"buttons\":3") != std::string::npos);
    CHECK(r.e.uiTries == 1);                                         // the UI layer was asked first
    CHECK(lastCall(r.g) == msgClick(FakeGame::msgTile(1)));          // HandleClick(7, tile of the 2nd listed button)
    r.g.calls.clear();
    CHECK(r.call("bot.menu.press", "{\"button\":0}"));
    CHECK(lastCall(r.g) == msgClick(FakeGame::msgTile(0)));          // never a SetField tile (menu+0x28..0x3C)
    for (const std::string& c : r.g.calls) CHECK(c.find(FakeGame::hex(FakeGame::kMsgDecoy)) == std::string::npos);
    r.g.calls.clear();
    CHECK(r.call("bot.menu.press", "{\"button\":2}") && lastCall(r.g) == msgClick(FakeGame::msgTile(2)));   // the last one, no wrap
    // an empty button text keeps its index in the engine value: the 2nd listed button reports value 2
    r.g.buildMessageBox({0, 2});
    r.g.calls.clear();
    CHECK(r.call("bot.menu.press", "{\"button\":1}") && r.out.find("\"value\":2") != std::string::npos);
    CHECK(lastCall(r.g) == msgClick(FakeGame::msgTile(1)));
}

TEST(native_message_press_refuses_with_a_reason) {
    GlueRig r;
    r.e.uiBox = false;
    r.g.calls.clear();
    // a button the box does not list
    r.g.buildMessageBox({0, 1});
    CHECK(!r.call("bot.menu.press", "{\"button\":2}") && r.out.find("is not shown (the box has 2 buttons)") != std::string::npos);
    CHECK(r.out.find("ui layer: no message box is being shown") != std::string::npos);
    // menu 1001 with another vtable (here the base Menu one)
    r.g.buildMessageBox({0, 1}, 0x1095484);
    CHECK(!r.call("bot.menu.press", "{\"button\":0}") && r.out.find("not a MessageMenu") != std::string::npos);
    // a listed button without a tile
    r.g.buildMessageBox({0, 1});
    r.g.set32(FakeGame::kMsgItems, 0);
    CHECK(!r.call("bot.menu.press", "{\"button\":0}") && r.out.find("has no tile") != std::string::npos);
    // no current message: the box is closing
    r.g.buildMessageBox({0, 1});
    r.g.set32(nvbots::eng::kMsgQueueIdx, 1);
    CHECK(!r.call("bot.menu.press", "{\"button\":0}") && r.out.find("closing") != std::string::npos);
    // not on the game main thread
    r.g.buildMessageBox({0, 1});
    r.g.mainThread = false;
    CHECK(!r.call("bot.menu.press", "{\"button\":0}") && r.out.find("main thread") != std::string::npos);
    r.g.mainThread = true;
    // the message menu is not the top menu (a stale MessageMenu object is still there): the UI layer's error comes back unchanged
    r.g.buildMessageBox({0, 1});
    r.g.topId = 0x3EC;
    CHECK(!r.call("bot.menu.press", "{\"button\":0}") && r.out == "no message box is being shown");
    CHECK(!anyMsgClick(r.g));                                        // no refusal above touched the engine
    // the live glue refusing the call is reported, not hidden
    r.g.buildMessageBox({0, 1});
    r.g.clickOk = false;
    CHECK(!r.call("bot.menu.press", "{\"button\":0}") && r.out.find("refused the HandleClick") != std::string::npos);
}

TEST(ui_path_wins_and_consent_is_unchanged) {
    GlueRig r;
    r.g.buildMessageBox({0, 1});   // both boxes exist: the UI layer's box answers
    CHECK(r.call("bot.menu.press", "{\"button\":1}") && r.out.find("\"path\":\"ui\"") != std::string::npos && r.e.pressed == 1);
    CHECK(r.out.find("\"menu\":900") != std::string::npos);
    CHECK(!anyMsgClick(r.g));
    CHECK(r.call("bot.consent", "{\"answer\":\"notyet\"}") && r.e.pressed == 1 && r.out.find("path") == std::string::npos);
    r.e.uiBox = false;              // consent never presses the engine's box
    CHECK(!r.call("bot.consent", "{\"answer\":\"ready\"}") && r.out == "no message box is being shown");
    CHECK(!anyMsgClick(r.g));
}

TEST(native_press_pins_agree) {
    CHECK(addr::MsgMenuSlotHandleClick.addr == addr::MessageMenuVtbl + addr::kMenuSlotHandleClick);
    CHECK(addr::MsgMenuSlotHandleClick.value == addr::MsgMenuHandleClick.addr);
    CHECK(addr::MsgMenuHandleClick.ret == 8);
    bool listed = false;
    for (const addr::CallPin* c : addr::kAllCalls) listed = listed || c == &addr::MsgMenuHandleClick;
    CHECK(listed);
}

TEST(fasttravel_is_a_move_and_says_it_is_not_genuine) {
    GlueRig r;
    CHECK(r.call("test.fasttravel", "{\"marker\":\"FalloutNV.esm:ABC\"}"));
    CHECK(r.e.moved == "FalloutNV.esm:ABC" && r.out.find("\"genuine\":false") != std::string::npos);
}


TEST(genuine_fast_travel_queues_the_engine_travel_object) {
    GlueRig r;
    r.g.buildMenus();
    r.g.forms["FalloutNV.esm:ABC"] = 0x3C000000;
    const char* kReq = "{\"marker\":\"FalloutNV.esm:ABC\"}";
    CHECK(r.call("bot.fasttravel", kReq));
    CHECK(r.out.find("\"genuine\":true") != std::string::npos && r.out.find("\"queued\":true") != std::string::npos);
    CHECK(r.g.get32(r.g.travelObj + nvbots::addr::kTravelObjMarker) == 0x3C000000);   // marker handed over at obj+0x30
    CHECK(r.g.get32(r.g.player + nvbots::addr::kPlayerPending) == r.g.travelObj);     // and the object is the player's pending one
    CHECK(r.e.moved.empty());                                                          // no MoveTo teleport involved
    // the engine's own may-travel check runs before anything is allocated or written
    GlueRig no;
    no.g.buildMenus();
    no.g.forms["FalloutNV.esm:ABC"] = 0x3C000000;
    no.g.canTravel = false;
    CHECK(!no.call("bot.fasttravel", kReq) && no.out.find("refuses") != std::string::npos);
    for (auto& c : no.g.calls) CHECK(c != "newtravel" && c.rfind("pending", 0) != 0);
    // busy menus and VATS
    GlueRig busy;
    busy.g.buildMenus();
    busy.g.forms["FalloutNV.esm:ABC"] = 0x3C000000;
    busy.g.topId = 0x3F1;
    CHECK(!busy.call("bot.fasttravel", kReq) && busy.out.find("busy") != std::string::npos);
    busy.g.topId = 0;
    busy.g.set32(nvbots::eng::kVatsCamera + nvbots::eng::kVatsMode, 1);
    CHECK(!busy.call("bot.fasttravel", kReq) && busy.out.find("busy") != std::string::npos);
    busy.g.set32(nvbots::eng::kVatsCamera + nvbots::eng::kVatsMode, 0);
    busy.g.set32(busy.g.player + nvbots::addr::kPlayerPending, 0x1234);   // a travel is already pending
    CHECK(!busy.call("bot.fasttravel", kReq) && busy.out.find("pending") != std::string::npos);
    CHECK(!r.call("bot.fasttravel", "{\"marker\":\"Nope:1\"}") && r.out.find("not loaded") != std::string::npos);
    CHECK(!r.call("bot.fasttravel", "{}"));
    Rig plain;
    CHECK(!plain.call("bot.fasttravel", kReq) && plain.out.find("unsupported") != std::string::npos);
}

TEST(drop_log_correlates_by_base_and_time_window) {
    Rig r;
    CHECK(!r.call("bot.drops") && r.out.find("unsupported") != std::string::npos);   // no OnDrop handler registered
    r.rpc.setDropsLive(true);
    CHECK(r.call("bot.drops") && r.out.find("\"seq\":0") != std::string::npos);
    r.e.now = 10200;
    r.rpc.drops().record("FalloutNV.esm:A1", "FalloutNV.esm:B1", "player", 10000);
    r.rpc.drops().record("FalloutNV.esm:A2", "FalloutNV.esm:B2", "player", 10100);
    CHECK(r.call("bot.drops", "{\"since\":0,\"base\":\"FalloutNV.esm:B2\"}"));
    CHECK(r.out.find("A2") != std::string::npos && r.out.find("A1") == std::string::npos);   // the base form selects
    CHECK(r.call("bot.drops", "{\"since\":1}") && r.out.find("A2") != std::string::npos && r.out.find("A1") == std::string::npos);   // since is exclusive
    CHECK(r.call("bot.drops", "{\"since\":0}") && r.out.find("A1") != std::string::npos && r.out.find("A2") != std::string::npos);
    r.e.now = 20000;   // both are old now
    CHECK(r.call("bot.drops", "{\"since\":0}") && r.out.find("A1") == std::string::npos && r.out.find("A2") == std::string::npos);
    CHECK(r.call("bot.drops", "{\"since\":0,\"windowMs\":60000}") && r.out.find("A1") != std::string::npos);
}

TEST(drop_log_base_matches_the_engine_form_name) {
    // In game the engine names the base "falloutnv.esm:015169". Bot.drop sends "FalloutNV.esm:015169" (2026-10-21).
    Rig r;
    r.rpc.setDropsLive(true);
    r.e.now = 10100;
    r.rpc.drops().record("FF001313", "falloutnv.esm:015169", "falloutnv.esm:000014", 10000);
    CHECK(r.call("bot.drops", "{\"since\":0,\"base\":\"FalloutNV.esm:015169\"}") && r.out.find("FF001313") != std::string::npos);
    CHECK(r.call("bot.drops", "{\"since\":0,\"base\":\"FALLOUTNV.ESM:15169\"}") && r.out.find("FF001313") != std::string::npos);
    CHECK(r.call("bot.drops", "{\"since\":0,\"base\":\"FalloutNV.esm:115169\"}") && r.out.find("FF001313") == std::string::npos);
    CHECK(r.call("bot.drops", "{\"since\":0,\"base\":\"DeadMoney.esm:015169\"}") && r.out.find("FF001313") == std::string::npos);
    CHECK(sameFormName("falloutnv.esm:015169", "FalloutNV.esm:15169"));
    CHECK(!sameFormName("falloutnv.esm:015169", "falloutnv.esm:zz"));
    CHECK(!sameFormName("falloutnv.esm:015169", "15169"));   // a bare id never matches a plugin name: no load order here
}

TEST(refs_of_lists_enabled_refs_of_a_base) {
    GlueRig r;
    const uint32_t kBase = 0x1234;
    for (int i = 0; i < 3; ++i) {
        const Addr32 ref = 0x38000000 + i * 0x100, bf = 0x39000000;
        r.g.set32(ref + 0x20, bf); r.g.set32(bf + nvbots::eng::kFormId, kBase); r.g.set32(ref + 0x08, i == 1 ? 0x800 : 0);
        const Addr32 node = FakeGame::kCell + 0xAC + i * 0x10;
        r.g.set32(node, ref); r.g.set32(node + 4, i < 2 ? node + 0x10 : 0);
    }
    r.g.set32(r.g.player + 0x40, FakeGame::kCell);
    r.g.formIds["FalloutNV.esm:1234"] = kBase;
    CHECK(r.call("dev.refsOf", "{\"base\":\"FalloutNV.esm:1234\"}"));
    CHECK(r.out.find("\"count\":2") != std::string::npos);
    CHECK(r.out.find("38000000") != std::string::npos && r.out.find("38000200") != std::string::npos && r.out.find("38000100") == std::string::npos);
}

TEST(count_refs_by_base_in_a_cell) {
    GlueRig r;
    const uint32_t kBase = 0x1234, kOther = 0x4321;
    auto mk = [&](int i, uint32_t baseId, uint32_t flags) {
        const Addr32 ref = 0x38000000 + i * 0x100, bf = 0x39000000 + i * 0x100;
        r.g.set32(ref + 0x20, bf); r.g.set32(bf + nvbots::eng::kFormId, baseId); r.g.set32(ref + 0x08, flags);
        return ref;
    };
    const Addr32 refs[5] = {mk(0, kBase, 0), mk(1, kBase, 0), mk(2, kOther, 0), mk(3, kBase, 0x800), mk(4, kBase, 0)};   // [3] disabled
    for (int i = 0; i < 5; ++i) {
        const Addr32 node = FakeGame::kCell + 0xAC + (i ? i * 0x10 : 0);
        r.g.set32(node, refs[i]);
        r.g.set32(node + 4, i < 4 ? FakeGame::kCell + 0xAC + (i + 1) * 0x10 : 0);
    }
    r.g.set32(r.g.player + 0x40, FakeGame::kCell);
    r.g.formIds["FalloutNV.esm:1234"] = kBase;
    r.g.forms["FalloutNV.esm:C1"] = FakeGame::kCell;
    CHECK(r.call("dev.countRefs", "{\"base\":\"FalloutNV.esm:1234\"}") && r.out.find("\"count\":3") != std::string::npos);
    CHECK(r.call("dev.countRefs", "{\"base\":\"FalloutNV.esm:1234\",\"cell\":\"FalloutNV.esm:C1\"}") && r.out.find("\"count\":3") != std::string::npos);
    CHECK(!r.call("dev.countRefs", "{\"base\":\"Nope:1\"}"));
    CHECK(!r.call("dev.countRefs", "{\"base\":\"FalloutNV.esm:1234\",\"cell\":\"Nope:1\"}"));
}

TEST(terminal_pick_clicks_the_requested_entry_and_records_it) {
    GlueRig r;
    r.g.forms["FalloutNV.esm:700"] = 0x70000000;
    r.g.forms["FalloutNV.esm:701"] = 0x70000100;
    r.g.buildMenus();
    r.g.buildTerminal({"Read log", "Open vault door", "Exit"}, 0x70000000);
    r.g.topId = 0x421;
    CHECK(r.call("dev.terminal", "{\"ref\":\"FalloutNV.esm:700\",\"entry\":1}"));
    CHECK(r.out.find("Open vault door") != std::string::npos && r.out.find("\"index\":1") != std::string::npos);
    CHECK(r.g.calls.back() == "termclick " + FakeGame::hex(FakeGame::kTerm + 0x10 + 0x10));   // the tile of entry 1
    CHECK(r.call("dev.terminalState", "{\"ref\":\"FalloutNV.esm:700\"}"));
    CHECK(r.out.find("Open vault door") != std::string::npos);
    CHECK(r.call("dev.terminalState", "{\"ref\":\"FalloutNV.esm:701\"}") && r.out.find("\"used\":[]") != std::string::npos);
    // refusals: out of range, a different terminal, menu not open
    CHECK(!r.call("dev.terminal", "{\"ref\":\"FalloutNV.esm:700\",\"entry\":3}") && r.out.find("out of range") != std::string::npos);
    CHECK(!r.call("dev.terminal", "{\"ref\":\"FalloutNV.esm:701\",\"entry\":0}") && r.out.find("different") != std::string::npos);
    r.g.topId = 0x3EC;
    r.g.calls.clear();
    CHECK(!r.call("dev.terminal", "{\"ref\":\"FalloutNV.esm:700\",\"entry\":0}") && r.out.find("not open") != std::string::npos);
    for (auto& c : r.g.calls) CHECK(c.rfind("termclick", 0) != 0);
    // picking the last entry (index 2) must click the last tile, not wrap
    r.g.topId = 0x421;
    CHECK(r.call("dev.terminal", "{\"ref\":\"FalloutNV.esm:700\",\"entry\":2}"));
    CHECK(r.g.calls.back() == "termclick " + FakeGame::hex(FakeGame::kTerm + 0x10 + 0x20));
}

TEST(test_fasttravel_needs_the_test_helper_env) {
    GlueRig r;
    r.e.helpers = false;
    CHECK(!r.call("test.fasttravel", "{\"marker\":\"FalloutNV.esm:ABC\"}"));
    CHECK(r.out.find("NVCOOP_TEST_HELPERS=1") != std::string::npos && r.e.moved.empty());
    r.e.helpers = true;
    CHECK(r.call("test.fasttravel", "{\"marker\":\"FalloutNV.esm:ABC\"}") && r.e.moved == "FalloutNV.esm:ABC");
    // the genuine path is not a test helper: it is open on a normal game
    r.e.helpers = false;
    r.g.buildMenus();
    r.g.forms["FalloutNV.esm:ABC"] = 0x3C000000;
    CHECK(r.call("bot.fasttravel", "{\"marker\":\"FalloutNV.esm:ABC\"}"));
}

// ---- bot.menu.close ----
static int closeCalls(const FakeGame& g) { int n = 0; for (const std::string& c : g.calls) n += c == "closeallmenus"; return n; }

TEST(menu_close_leaves_the_container_menu_the_engine_way) {
    GlueRig r;
    r.g.buildMenus();
    r.g.topId = 0x3F0;
    CHECK(r.call("bot.menu.close", "{}"));   // the default kind is the container menu
    CHECK(r.out.find("\"closed\":true") != std::string::npos && r.out.find("\"kind\":\"container\"") != std::string::npos && r.out.find("\"menu\":1008") != std::string::npos);
    CHECK(closeCalls(r.g) == 1);
    CHECK(r.g.topId == 0);                                                       // the menu stack is empty afterwards
    CHECK(r.call("bot.menu") && r.out.find("\"kind\":\"none\"") != std::string::npos);
    // an explicit kind, and the terminal menu
    r.g.topId = 0x421;
    CHECK(r.call("bot.menu.close", "{\"kind\":\"terminal\"}") && r.out.find("\"kind\":\"terminal\"") != std::string::npos && closeCalls(r.g) == 2);
    // "any" closes what a player could cancel out of
    for (uint32_t id : {0x41D, 0x3F6, 0x41F, 0x3EA}) {
        r.g.topId = id;
        const int before = closeCalls(r.g);
        CHECK(r.call("bot.menu.close", "{\"kind\":\"any\"}") && closeCalls(r.g) == before + 1);
    }
}

TEST(menu_close_refuses_with_a_reason_and_touches_nothing) {
    GlueRig r;
    r.g.buildMenus();
    // nothing open
    r.g.topId = 0;
    CHECK(!r.call("bot.menu.close", "{}") && r.out == "no menu is open");
    CHECK(!r.call("bot.menu.close", "{\"kind\":\"any\"}") && r.out == "no menu is open");
    // another kind is on top: the container request leaves it open
    r.g.topId = 0x421;
    CHECK(!r.call("bot.menu.close", "{}") && r.out.find("is terminal") != std::string::npos && r.out.find("not container") != std::string::npos);
    r.g.topId = 0x3F1;   // a conversation is never closed as a container
    CHECK(!r.call("bot.menu.close", "{\"kind\":\"container\"}") && r.out.find("is dialogue") != std::string::npos);
    // "any" never closes the message box, the loading screen, the pause menu, VATS, the tutorial box or the HUD
    for (uint32_t id : {0x3E9, 0x3EF, 0x3F5, 0x420, 0x423, 0x3EC}) {
        r.g.topId = id;
        CHECK(!r.call("bot.menu.close", "{\"kind\":\"any\"}") && r.out.find("not closed from code") != std::string::npos);
    }
    CHECK(closeCalls(r.g) == 0);
    // wrong thread
    r.g.topId = 0x3F0;
    r.g.mainThread = false;
    CHECK(!r.call("bot.menu.close", "{}") && r.out.find("main thread") != std::string::npos && closeCalls(r.g) == 0);
    r.g.mainThread = true;
    // the live glue refuses (pins, fault): reported, not claimed
    r.g.closeOk = false;
    CHECK(!r.call("bot.menu.close", "{}") && r.out.find("refused the CloseAllMenusCore call") != std::string::npos);
    CHECK(r.g.topId == 0x3F0);
    // no interface manager yet
    r.g.closeOk = true;
    r.g.mem.erase(nvbots::eng::kInterfaceMgrPtr);
    CHECK(!r.call("bot.menu.close", "{}") && r.out.find("interface manager not ready") != std::string::npos);
    // without engine glue
    Rig plain;
    CHECK(!plain.call("bot.menu.close", "{}") && plain.out.find("unsupported") != std::string::npos);
}

TEST(menu_close_pin_is_the_closeallmenus_core) {
    CHECK(addr::CloseAllMenusCore.addr == 0x7037E0U && addr::CloseAllMenusCore.ret == 0);
    bool listed = false;
    for (const addr::CallPin* c : addr::kAllCalls) listed = listed || c == &addr::CloseAllMenusCore;
    CHECK(listed);
}

TEST(glue_methods_need_the_engine) {
    Rig r;
    for (const char* m : {"bot.menu", "bot.dialog", "bot.inventory", "bot.vats", "test.vats.hit", "dev.countRefs", "dev.refsOf", "bot.fasttravel", "dev.terminal"})
        CHECK(!r.call(m, "{\"ref\":\"x\",\"base\":\"x\"}") && r.out.find("unsupported") != std::string::npos);
    CHECK(!r.call("test.fasttravel", "{\"marker\":\"x\"}") && r.out.find("unsupported") != std::string::npos);
}

// ---- bot.move run = the speed (always-run) ----
// The engine side of the contract, one frame of PlayerCharacter::vf2F8 (static RE, bots_addrs.h kRunPins). Literal addresses on purpose.
//   0x940C7D: the AlwaysRun (10) press edge flips player+0x651.
//   0x941A2B: keyboard mode and always-run on -> run unless Run (9) is pressed or held; otherwise run only while Run is pressed or held.
// Returns true when the player moves at run speed this frame.
struct RunEngine {
    FakeGame& g;
    ForcedInput& in;
    bool frame() {
        uint8_t on = 0, pad = 0, padOff = 0;
        if (in.filter(kAlwaysRun, kPressed, 0)) {
            g.rd8(g.player + 0x651, &on);
            g.set8(g.player + 0x651, on ? 0 : 1);
        }
        g.rd8(g.player + 0x651, &on);
        g.rd8(0x11D8A84, &pad);
        g.rd8(0x11D8C50, &padOff);
        const bool padMode = pad && !padOff;
        const bool runDown = in.filter(kRun, kPressed, 0) || in.filter(kRun, kHeld, 0);
        return (!padMode && on) ? !runDown : runDown;
    }
};

TEST(always_run_state_reads_the_engine_bytes) {
    FakeGame g;
    CHECK(alwaysRunState(g) == -1);          // nothing readable: unknown
    g.setAlwaysRun(true);
    CHECK(alwaysRunState(g) == 1);
    g.setAlwaysRun(false);
    CHECK(alwaysRunState(g) == 0);
    g.setAlwaysRun(true, true, false);
    CHECK(alwaysRunState(g) == 0);           // gamepad mode: the engine ignores the toggle, Run alone picks the speed
    g.setAlwaysRun(true, true, true);
    CHECK(alwaysRunState(g) == 1);           // a pad answers but bDisable360Controller=1: keyboard mode
    g.set8(g.player + 0x652, 1);
    g.setAlwaysRun(false);
    CHECK(alwaysRunState(g) == 0);           // the auto-move byte next to the toggle is not the toggle
}

TEST(move_run_is_the_speed_whatever_the_always_run_toggle) {
    // {always-run, pad connected, bDisable360Controller}; the first in-game run had always-run on (the New Vegas default)
    const bool cfg[4][3] = {{true, false, false}, {false, false, false}, {true, true, false}, {true, true, true}};
    for (const auto& c : cfg) {
        for (int want = 0; want < 2; ++want) {
            GlueRig r;
            r.g.setAlwaysRun(c[0], c[1], c[2]);
            RunEngine eng{r.g, r.in};
            CHECK(r.call("bot.move", want ? "{\"forward\":1,\"strafe\":0,\"run\":true}" : "{\"forward\":1,\"strafe\":0,\"run\":false}"));
            const bool engineOn = c[0] && !(c[1] && !c[2]);
            CHECK(r.out.find(engineOn ? "\"alwaysRun\":true" : "\"alwaysRun\":false") != std::string::npos);
            CHECK(r.out.find(want ? "\"run\":true" : "\"run\":false") != std::string::npos);
            int asked = 0;
            for (int f = 0; f < 10; ++f) {
                asked += eng.frame() == (want != 0);
                r.e.now += 16;
                r.rpc.tick();
            }
            CHECK(asked == 10);
            CHECK(r.in.isHeld(kForward));
        }
    }
}

TEST(move_without_run_leaves_the_speed_to_the_game) {
    // Review finding on PR #10: callers that omit `run` must keep the old behaviour (no Run key; the toggle picks the speed).
    for (int on = 0; on < 2; ++on) {
        GlueRig r;
        r.g.setAlwaysRun(on != 0);
        RunEngine eng{r.g, r.in};
        CHECK(r.call("bot.move", "{\"forward\":1,\"strafe\":0}"));
        int ran = 0;
        for (int f = 0; f < 10; ++f) {
            ran += eng.frame();
            r.e.now += 16;
            r.rpc.tick();
        }
        CHECK(!r.in.isHeld(kRun));
        CHECK(ran == (on ? 10 : 0));   // always-run on: runs; off: walks, exactly like a player who presses nothing
    }
}

TEST(move_follows_an_always_run_toggle_during_the_move) {
    GlueRig r;
    r.g.setAlwaysRun(true);
    RunEngine eng{r.g, r.in};
    auto frames = [&](int n, int* walked) {
        bool last = false;
        for (int f = 0; f < n; ++f) {
            last = eng.frame();
            *walked += !last;
            r.e.now += 16;
            r.rpc.tick();
        }
        return last;
    };
    CHECK(r.call("bot.move", "{\"forward\":1,\"strafe\":0,\"run\":true}"));
    int walked = 0;
    CHECK(frames(5, &walked) && walked == 0);
    CHECK(r.call("bot.input", "{\"tap\":[\"AlwaysRun\"]}"));   // someone flips the toggle while the bot runs
    walked = 0;
    CHECK(frames(12, &walked));
    CHECK(walked <= 1);                                         // at most the frame of the flip itself
    uint8_t v = 9;
    r.g.rd8(r.g.player + 0x651, &v);
    CHECK(v == 0);                                              // the edge reached the engine once
    CHECK(r.call("bot.input", "{\"tap\":[\"AlwaysRun\"]}"));   // and back on
    walked = 0;
    CHECK(frames(12, &walked));
    CHECK(walked <= 1);
    CHECK(r.call("bot.stop"));
    CHECK(!r.in.isHeld(kRun) && !r.in.isHeld(kForward));
    walked = 0;
    frames(3, &walked);
    CHECK(!r.in.isHeld(kRun));                                  // a stopped move no longer drives Run
}

TEST(move_with_unknown_always_run_keeps_the_key_meaning) {
    GlueRig r;   // no player bytes: alwaysRunState is -1 (in game: the always-run pins did not match this exe)
    CHECK(r.call("bot.move", "{\"forward\":1,\"run\":true}"));
    CHECK(r.in.isHeld(kRun) && r.out.find("\"alwaysRun\":null") != std::string::npos && r.out.find("\"runControl\":true") != std::string::npos);
    CHECK(r.call("bot.move", "{\"forward\":1,\"run\":false}"));
    CHECK(!r.in.isHeld(kRun) && r.out.find("\"runControl\":false") != std::string::npos);
    CHECK(r.call("bot.status") && r.out.find("\"alwaysRun\":null") != std::string::npos);
    r.g.setAlwaysRun(true);
    CHECK(r.call("bot.status") && r.out.find("\"alwaysRun\":true") != std::string::npos);
}

TEST(always_run_pins_hold_the_addresses_the_reader_uses) {
    auto u32at = [](const unsigned char* b) { uint32_t v; std::memcpy(&v, b, 4); return v; };
    CHECK(addr::AlwaysRunEdge.bytes[3] == kAlwaysRun);                          // push 0Ah: GetControlState(AlwaysRun, pressed)
    CHECK(u32at(addr::AlwaysRunFlip.bytes + 6) == eng::kPlayerAlwaysRun);       // movzx eax, byte [edx+651h]
    CHECK(u32at(addr::RunAlwaysRead.bytes + 3) == eng::kPlayerAlwaysRun);
    CHECK(addr::RunAlwaysRead.bytes[14] == kRun);                               // push 9: the always-run branch asks for Run
    CHECK(u32at(addr::PadModeConnected.bytes + 7) == eng::kPadConnected);
    CHECK(u32at(addr::PadModeSetting.bytes + 5) + 4 == eng::kDisable360Value);  // Setting object + 4 = its value byte
    // the run decision calls the pad-mode gate that alwaysRunState replicates (call rel32 at 0x941A2B lands on 0x4B71D0)
    CHECK(addr::RunPadGate.addr + 5 + u32at(addr::RunPadGate.bytes + 1) == addr::PadModeConnected.addr);
}

// ---- pins ----
struct FakeMem : nvhooks::MemoryOps {
    std::map<nvhooks::Addr, std::vector<uint8_t>> regions;
    nvhooks::Addr next = 0x20000000;
    uint8_t* at(nvhooks::Addr a, size_t n) {
        for (auto& [b, v] : regions)
            if (a >= b && a + n <= b + v.size()) return &v[a - b];
        return nullptr;
    }
    bool read(nvhooks::Addr a, void* o, size_t n) override { auto* p = at(a, n); if (!p) return false; std::memcpy(o, p, n); return true; }
    bool write(nvhooks::Addr a, const void* i, size_t n) override { auto* p = at(a, n); if (!p) return false; std::memcpy(p, i, n); return true; }
    nvhooks::Addr allocExec(size_t n) override { auto a = next; next += 0x1000; regions[a] = std::vector<uint8_t>(n, 0); return a; }
    void freeExec(nvhooks::Addr a, size_t) override { regions.erase(a); }
};

TEST(spec_is_valid_and_installs_and_restores_on_a_fake_image) {
    const nvhooks::HookSpec& s = addr::GetControlState;
    CHECK(nvhooks::specIsValid(s));
    CHECK(s.retBytes == 8);
    FakeMem m;
    m.regions[s.target] = std::vector<uint8_t>(64, 0xCC);
    std::memcpy(m.regions[s.target].data(), s.expected, 16);
    nvhooks::HookRegistry reg(m);
    nvhooks::Addr tramp = 0;
    CHECK(reg.acquire(s, 0x30000000, &tramp) == nvhooks::PatchStatus::Ok);
    CHECK(m.regions[s.target][0] == 0xE9);
    CHECK(reg.release(s) == nvhooks::PatchStatus::Restored);
    CHECK(std::memcmp(m.regions[s.target].data(), s.expected, 16) == 0);
    // a changed exe (different bytes) must be refused, nothing written
    m.regions[s.target][5] = 0x15;
    CHECK(reg.acquire(s, 0x30000000, &tramp) == nvhooks::PatchStatus::PrologueMismatch);
}

TEST(pins_match_real_exe_if_available) {
    const char* path = std::getenv("BOTS_EXE");
    if (!path || !*path) { std::printf("  (BOTS_EXE not set: real-image check skipped)\n"); return; }
    std::ifstream f(path, std::ios::binary);
    std::vector<unsigned char> d((std::istreambuf_iterator<char>(f)), std::istreambuf_iterator<char>());
    CHECK(d.size() > 0x1000);
    if (d.size() < 0x1000) return;
    auto u32 = [&](size_t o) { uint32_t v; std::memcpy(&v, &d[o], 4); return v; };
    auto u16 = [&](size_t o) { uint16_t v; std::memcpy(&v, &d[o], 2); return v; };
    size_t e = u32(0x3C), nsec = u16(e + 6), opt = u16(e + 20);
    for (const nvhooks::HookSpec* h : addr::kAllHooks) {
        bool found = false;
        for (size_t i = 0; i < nsec && !found; ++i) {
            size_t o = e + 24 + opt + i * 40;
            unsigned vs = u32(o + 8), va = u32(o + 12) + 0x400000, rs = u32(o + 16), rp = u32(o + 20);
            if (h->target >= va && h->target < va + (vs > rs ? vs : rs)) {
                found = true;
                CHECK(std::memcmp(&d[rp + (h->target - va)], h->expected, h->expectedLen) == 0);
                // the function must also end in its `ret` (C2 08 00 for retBytes 8, C3 after `mov esp,ebp; pop ebp` for 0) somewhere after
                // its prologue: confirms retBytes
                bool retOk = false;
                const size_t f = rp + (h->target - va);
                for (size_t k = 0; k < 0x1D0; ++k) {
                    if (h->retBytes == 8 && d[f + k] == 0xC2 && d[f + k + 1] == 0x08 && d[f + k + 2] == 0x00) { retOk = true; break; }
                    if (h->retBytes == 0 && d[f + k] == 0x8B && d[f + k + 1] == 0xE5 && d[f + k + 2] == 0x5D && d[f + k + 3] == 0xC3) { retOk = true; break; }
                }
                CHECK(retOk);
            }
        }
        CHECK(found);
    }
    for (const addr::CallPin* c : addr::kAllCalls) {
        bool found = false;
        for (size_t i = 0; i < nsec && !found; ++i) {
            size_t o = e + 24 + opt + i * 40;
            unsigned vs = u32(o + 8), va = u32(o + 12) + 0x400000, rs = u32(o + 16), rp = u32(o + 20);
            if (c->addr >= va && c->addr < va + (vs > rs ? vs : rs)) {
                found = true;
                CHECK(std::memcmp(&d[rp + (c->addr - va)], c->bytes, 16) == 0);
            }
        }
        CHECK(found);
    }
    for (const addr::DataPin* p : addr::kAllData) {   // vtable slots in .rdata
        bool found = false;
        for (size_t i = 0; i < nsec && !found; ++i) {
            size_t o = e + 24 + opt + i * 40;
            unsigned vs = u32(o + 8), va = u32(o + 12) + 0x400000, rs = u32(o + 16), rp = u32(o + 20);
            if (p->addr >= va && p->addr + 4 <= va + rs) {
                found = true;
                CHECK(u32(rp + (p->addr - va)) == p->value);
            }
            (void)vs;
        }
        CHECK(found);
    }
    int runPins = 0;
    for (const addr::CodePin* c : addr::kRunPins) {
        bool found = false;
        for (size_t i = 0; i < nsec && !found; ++i) {
            size_t o = e + 24 + opt + i * 40;
            unsigned vs = u32(o + 8), va = u32(o + 12) + 0x400000, rs = u32(o + 16), rp = u32(o + 20);
            if (c->addr >= va && c->addr + 16 <= va + rs) {
                found = true;
                const bool same = std::memcmp(&d[rp + (c->addr - va)], c->bytes, 16) == 0;
                CHECK(same);
                runPins += same;
            }
            (void)vs;
        }
        CHECK(found);
    }
    std::printf("  always-run pins matched in the reference exe: %d of %d\n", runPins, (int)(sizeof addr::kRunPins / sizeof addr::kRunPins[0]));
}

// A pinned entry that world_time has detoured (jmp rel32 at the entry, trampoline = the stolen prologue) still counts as the pinned function.
// Before: bots read "engine pin PendingTravelSet @0093BE30 does not match this exe" after a core reload and switched off bot.menu & co.
static void pins_accept_an_entry_our_own_hook_detoured() {
    const addr::CallPin& p = addr::PendingTravelSet;
    unsigned char hooked[16];
    std::memcpy(hooked, p.bytes, 16);
    hooked[0] = 0xE9; hooked[1] = 0x10; hooked[2] = 0x20; hooked[3] = 0x30; hooked[4] = 0x40;       // jmp detour
    unsigned char tramp[16] = {};
    std::memcpy(tramp, p.bytes, 6);                                                                    // stolen 55 8B EC 83 EC 08, then jmp back
    tramp[6] = 0xE9;
    CHECK(pinBytesOk(p.bytes, p.bytes, nullptr));                       // untouched entry
    CHECK(pinBytesOk(hooked, p.bytes, tramp));                          // our detour, original prologue in the trampoline
    CHECK(!pinBytesOk(hooked, p.bytes, nullptr));                       // a jmp nobody of ours owns (no trampoline): another mod's patch, refused
    unsigned char other[16];
    std::memcpy(other, tramp, 16);
    other[2] ^= 0xFF;
    CHECK(!pinBytesOk(hooked, p.bytes, other));                         // our detour over a DIFFERENT exe's function: the pin still fails
    unsigned char wrong[16];
    std::memcpy(wrong, p.bytes, 16);
    wrong[3] ^= 0xFF;
    CHECK(!pinBytesOk(wrong, p.bytes, tramp));                          // changed bytes without a jmp: refused whatever the trampoline holds
}

static void pins_accept_only_the_exact_known_rewrites() {
    const addr::KnownRewrite& tick = addr::kKnownRewrites[2];
    CHECK(tick.addr == addr::Tick.addr);
    CHECK(knownRewrite(tick.addr, tick.bytes) == &tick);                // the rewrite seen under TTW, at its own entry
    CHECK(knownRewrite(addr::ItemInfo.addr, tick.bytes) == nullptr);    // the same bytes at another entry: not that entry's rewrite
    unsigned char near[16];
    std::memcpy(near, tick.bytes, 16);
    near[2] = 0x18;                                                     // mov eax,[ecx+18h] at Tick: a different getter, refused
    CHECK(knownRewrite(tick.addr, near) == nullptr);
    CHECK(knownRewrite(addr::Tick.addr, addr::Tick.bytes) == nullptr);  // untouched code is the pin's job, not a rewrite
    CHECK(knownRewrite(addr::PendingTravelSet.addr, addr::PendingTravelSet.bytes) == nullptr);   // no rewrite known for this entry
    for (const addr::KnownRewrite& r : addr::kKnownRewrites) {          // every rewrite belongs to a call pin we check
        bool pinned = false;
        for (const addr::CallPin* p : addr::kAllCalls) pinned = pinned || p->addr == r.addr;
        CHECK(pinned);
    }
}

int main() {
    run("held_overrides_real_input_until_released", held_overrides_real_input_until_released);
    run("press_edge_reaches_the_game_exactly_once", press_edge_reaches_the_game_exactly_once);
    run("press_edge_same_frame_is_consistent", press_edge_same_frame_is_consistent);
    run("release_edge_once", release_edge_once);
    run("tap_holds_n_frames_then_lets_go", tap_holds_n_frames_then_lets_go);
    run("timed_hold_expires", timed_hold_expires);
    run("release_all_clears_everything", release_all_clears_everything);
    run("out_of_range_controls_are_never_forced", out_of_range_controls_are_never_forced);
    run("axes_map_to_the_right_controls", axes_map_to_the_right_controls);
    run("rpc_move_with_deadline_stops_itself", rpc_move_with_deadline_stops_itself);
    run("rpc_stop_releases_every_control", rpc_stop_releases_every_control);
    run("rpc_look_at_ref_uses_heading_and_pitch_rules", rpc_look_at_ref_uses_heading_and_pitch_rules);
    run("rpc_look_errors_are_reported", rpc_look_errors_are_reported);
    run("rpc_fire_pulses_the_attack_control_n_times", rpc_fire_pulses_the_attack_control_n_times);
    run("rpc_taps_and_generic_input", rpc_taps_and_generic_input);
    run("rpc_status_and_unwired_methods", rpc_status_and_unwired_methods);
    run("menu_kinds_and_vats_priority", menu_kinds_and_vats_priority);
    run("dialogue_state_is_reported", dialogue_state_is_reported);
    run("dialogue_list_and_pick_run_the_click_sequence", dialogue_list_and_pick_run_the_click_sequence);
    run("dialogue_pick_refuses_when_not_ready_gated_missing_or_special", dialogue_pick_refuses_when_not_ready_gated_missing_or_special);
    run("inventory_reports_totals_not_deltas", inventory_reports_totals_not_deltas);
    run("inventory_empty_container", inventory_empty_container);
    run("vats_job_closes_the_tutorial_cycles_queues_and_executes", vats_job_closes_the_tutorial_cycles_queues_and_executes);
    run("vats_job_without_tutorial_on_the_default_target", vats_job_without_tutorial_on_the_default_target);
    run("vats_job_closes_a_tutorial_box_that_opens_late", vats_job_closes_a_tutorial_box_that_opens_late);
    run("vats_job_keeps_an_attack_queued_before_a_late_tutorial_box", vats_job_keeps_an_attack_queued_before_a_late_tutorial_box);
    run("vats_job_refuses_zero_hit_chance_and_zero_ap", vats_job_refuses_zero_hit_chance_and_zero_ap);
    run("vats_job_never_executes_an_empty_queue", vats_job_never_executes_an_empty_queue);
    run("vats_job_fails_clearly", vats_job_fails_clearly);
    run("vats_accept_gate_forces_only_the_slot_1_press_edge", vats_accept_gate_forces_only_the_slot_1_press_edge);
    run("vats_pins_agree", vats_pins_agree);
    run("consent_and_press_answer_the_right_button", consent_and_press_answer_the_right_button);
    run("native_message_box_is_pressed_when_the_ui_layer_has_none", native_message_box_is_pressed_when_the_ui_layer_has_none);
    run("native_message_press_refuses_with_a_reason", native_message_press_refuses_with_a_reason);
    run("ui_path_wins_and_consent_is_unchanged", ui_path_wins_and_consent_is_unchanged);
    run("native_press_pins_agree", native_press_pins_agree);
    run("fasttravel_is_a_move_and_says_it_is_not_genuine", fasttravel_is_a_move_and_says_it_is_not_genuine);
    run("dialogue_pick_allows_speech_checks_and_says_so", dialogue_pick_allows_speech_checks_and_says_so);
    run("genuine_fast_travel_queues_the_engine_travel_object", genuine_fast_travel_queues_the_engine_travel_object);
    run("drop_log_correlates_by_base_and_time_window", drop_log_correlates_by_base_and_time_window);
    run("drop_log_base_matches_the_engine_form_name", drop_log_base_matches_the_engine_form_name);
    run("refs_of_lists_enabled_refs_of_a_base", refs_of_lists_enabled_refs_of_a_base);
    run("count_refs_by_base_in_a_cell", count_refs_by_base_in_a_cell);
    run("terminal_pick_clicks_the_requested_entry_and_records_it", terminal_pick_clicks_the_requested_entry_and_records_it);
    run("test_fasttravel_needs_the_test_helper_env", test_fasttravel_needs_the_test_helper_env);
    run("menu_close_leaves_the_container_menu_the_engine_way", menu_close_leaves_the_container_menu_the_engine_way);
    run("menu_close_refuses_with_a_reason_and_touches_nothing", menu_close_refuses_with_a_reason_and_touches_nothing);
    run("menu_close_pin_is_the_closeallmenus_core", menu_close_pin_is_the_closeallmenus_core);
    run("glue_methods_need_the_engine", glue_methods_need_the_engine);
    run("always_run_state_reads_the_engine_bytes", always_run_state_reads_the_engine_bytes);
    run("move_run_is_the_speed_whatever_the_always_run_toggle", move_run_is_the_speed_whatever_the_always_run_toggle);
    run("move_without_run_leaves_the_speed_to_the_game", move_without_run_leaves_the_speed_to_the_game);
    run("move_follows_an_always_run_toggle_during_the_move", move_follows_an_always_run_toggle_during_the_move);
    run("move_with_unknown_always_run_keeps_the_key_meaning", move_with_unknown_always_run_keeps_the_key_meaning);
    run("always_run_pins_hold_the_addresses_the_reader_uses", always_run_pins_hold_the_addresses_the_reader_uses);
    run("spec_is_valid_and_installs_and_restores_on_a_fake_image", spec_is_valid_and_installs_and_restores_on_a_fake_image);
    run("pins_accept_an_entry_our_own_hook_detoured", pins_accept_an_entry_our_own_hook_detoured);
    run("pins_accept_only_the_exact_known_rewrites", pins_accept_only_the_exact_known_rewrites);
    run("pins_match_real_exe_if_available", pins_match_real_exe_if_available);
    std::printf("%d checks, %d failed\n", g_checks, g_fail);
    return g_fail ? 1 : 0;
}
