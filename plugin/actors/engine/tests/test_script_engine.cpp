// ScriptActorEngine: the exact console text of every write, the axis mask, the cell move, failure accounting, and the AV name table.
// Twins: 1 (axis mask ignored), 2 (yaw not wrapped), 3 (cell-move offset sign), 4 (limb AV names), 5 (kill command), 6 (failures not counted).
#include <cmath>
#include <string>
#include <vector>

#include "actors/engine/script_engine.h"
#include "minitest.h"

using namespace nvcoop::actors;
using namespace nvcoop::actors::engine;

namespace {

class RecHost : public IEngineHost {
public:
    std::vector<std::string> lines;
    bool consoleOk = true;
    RefId anchor = 0x00ABCDEF;
    live::Vec3 anchorPos{100, 200, 300};
    bool direct = false;
    int directCalls = 0;
    bool worldReady() override { return true; }
    uint32_t worldEpoch() override { return 1; }
    RefId playerRef() override { return 0x14; }
    bool read(RefId, ActorRaw&) override { return false; }
    bool actorValue(RefId, uint8_t, float&) override { return false; }
    void loadedActors(std::vector<RefId>&, size_t) override {}
    bool cellLoaded(CellId) override { return true; }
    bool cellInfo(CellId, CellInfo&) override { return false; }
    bool toFormRef(uint32_t, fnvcoop::FormRef&) override { return false; }
    uint32_t fromFormRef(const fnvcoop::FormRef&) override { return 0; }
    RefId anchorInCell(CellId, live::Vec3& p) override { p = anchorPos; return anchor; }
    RefId placeAtMe(RefId, uint32_t) override { return 0x00FF0001; }
    bool console(const std::string& line, RefId on) override {
        char b[16];
        std::snprintf(b, sizeof b, "@%08X", on);
        lines.push_back(line + b);
        return consoleOk;
    }
    bool directPose(RefId, const live::Vec3&, uint8_t) override { ++directCalls; return direct; }
    bool attribution = true;
    std::vector<std::pair<RefId, RefId>> attributed;
    bool killWithAttribution(RefId victim, RefId killer) override {
        if (!attribution) return false;
        attributed.push_back({victim, killer});
        return true;
    }
};

}  // namespace

TEST(script_engine, setpos_writes_only_the_masked_axes) {
    RecHost h;
    ScriptActorEngine e(h);
    CHECK(e.setPos(0x100, live::Vec3{1.5f, -2.25f, 3}, 4));
    CHECK(h.lines.size() == 1 && h.lines[0] == "SetPos Z 3.0000@00000100");
    h.lines.clear();
    CHECK(e.setPos(0x100, live::Vec3{1.5f, -2.25f, 3}, 3));
    CHECK(h.lines.size() == 2 && h.lines[0] == "SetPos X 1.5000@00000100" && h.lines[1] == "SetPos Y -2.2500@00000100");
    h.lines.clear();
    CHECK(e.setPos(0x100, live::Vec3{1, 2, 3}, 7));
    CHECK(h.lines.size() == 3 && h.lines[2] == "SetPos Z 3.0000@00000100");
    h.lines.clear();
    CHECK(e.setPos(0x100, live::Vec3{1, 2, 3}, 0));
    CHECK(h.lines.empty());
    CHECK(e.linesRun() == 6);
}

TEST(script_engine, a_direct_pose_replaces_the_lines) {
    RecHost h;
    h.direct = true;
    ScriptActorEngine e(h);
    CHECK(e.setPos(0x100, live::Vec3{1, 2, 3}, 7));
    CHECK(h.directCalls == 1 && h.lines.empty() && e.linesRun() == 0);
}

TEST(script_engine, yaw_is_degrees_wrapped_to_the_half_turn) {
    const float pi = 3.14159265f;
    CHECK(std::fabs(lines::yawDegrees(pi / 2) - 90.0) < 0.001);
    CHECK(std::fabs(lines::yawDegrees(1.5f * pi) - (-90.0)) < 0.001);
    CHECK(std::fabs(lines::yawDegrees(-1.5f * pi) - 90.0) < 0.001);
    CHECK(std::fabs(lines::yawDegrees(2.5f * pi) - 90.0) < 0.001);
    CHECK(lines::setAngleZ(pi / 2) == "SetAngle Z 90.0000");
    CHECK(lines::setAngleZ(1.5f * pi) == "SetAngle Z -90.0000");
    RecHost h;
    ScriptActorEngine e(h);
    CHECK(e.setYaw(0x44, 1.5f * pi));
    CHECK(h.lines.size() == 1 && h.lines[0] == "SetAngle Z -90.0000@00000044");
}

TEST(script_engine, the_cell_move_is_an_offset_from_the_anchor_then_a_heading) {
    RecHost h;
    ScriptActorEngine e(h);
    CHECK(e.moveToCell(0x100, 7, live::Vec3{110, 190, 305}, 0));
    CHECK(h.lines.size() == 2);
    CHECK(h.lines[0] == "MoveTo 00ABCDEF 10.0000 -10.0000 5.0000@00000100");
    CHECK(h.lines[1] == "SetAngle Z 0.0000@00000100");
    h.lines.clear();
    h.anchor = 0;   // no reference in that cell: nothing is run
    CHECK(!e.moveToCell(0x100, 7, live::Vec3{1, 2, 3}, 0));
    CHECK(h.lines.empty());
}

TEST(script_engine, one_line_per_write_with_the_pinned_text) {
    RecHost h;
    ScriptActorEngine e(h);
    e.setRestrained(0x10, true); e.setRestrained(0x10, false); e.stopCombat(0x10); e.startCombat(0x10, 0x1234AB);
    e.evaluatePackage(0x10); e.setEssential(0x10, true); e.setEssential(0x10, false);
    e.killActor(0x10); e.resurrect(0x10); e.setDisabled(0x10, true); e.setDisabled(0x10, false); e.markForDelete(0x10);
    const std::vector<std::string> want = {
        "SetRestrained 1@00000010", "SetRestrained 0@00000010", "StopCombat@00000010", "StartCombat 001234AB@00000010",
        "EvaluatePackage@00000010", "SetActorRefEssential 1@00000010", "SetActorRefEssential 0@00000010",
        "KillActor@00000010", "ResurrectActor 0@00000010", "Disable@00000010", "Enable@00000010", "MarkForDelete@00000010"};
    CHECK(h.lines == want);
}

TEST(script_engine, a_kill_with_a_killer_goes_through_the_attribution_call_and_falls_back_to_the_plain_kill) {
    RecHost h;
    ScriptActorEngine e(h);
    CHECK(e.killActorBy(0x10, 0x2001));
    CHECK(h.attributed.size() == 1 && h.attributed[0].first == 0x10 && h.attributed[0].second == 0x2001 && h.lines.empty());
    h.attribution = false;                       // a host without attribution
    CHECK(e.killActorBy(0x11, 0x2001));
    CHECK(h.lines.size() == 1 && h.lines[0] == "KillActor@00000011");
    h.lines.clear();
    CHECK(e.killActorBy(0x12, 0) && h.lines.size() == 1 && h.attributed.size() == 1);   // nobody to credit: the plain kill
}

TEST(script_engine, actor_values_use_the_console_names_and_unknown_ones_run_nothing) {
    RecHost h;
    ScriptActorEngine e(h);
    CHECK(e.forceAv(0x10, kAvHealth, 85.4f));
    CHECK(e.forceAv(0x10, kAvHead, 0));
    CHECK(e.forceAv(0x10, kAvRightLeg, 12));
    CHECK(!e.forceAv(0x10, 99, 1));
    CHECK(h.lines.size() == 3);
    CHECK(h.lines[0] == "ForceActorValue Health 85@00000010");
    CHECK(h.lines[1] == "ForceActorValue PerceptionCondition 0@00000010");
    CHECK(h.lines[2] == "ForceActorValue RightMobilityCondition 12@00000010");
    for (uint8_t av : watchedAvs()) CHECK_MSG(avConsoleName(av) != nullptr, std::to_string(av));
}

TEST(script_engine, an_amount_is_a_whole_number_the_console_can_compile) {
    // game 2026-10-22: "ForceActorValue Health 30.00" prints "Unknown variable '30.00' for parameter Amount" and changes nothing.
    CHECK(lines::forceAv("Health", 30.0f) == "ForceActorValue Health 30");
    CHECK(lines::forceAv("Health", 29.6f) == "ForceActorValue Health 30");
    CHECK(lines::forceAv("Health", 0.0f) == "ForceActorValue Health 0");
    CHECK(lines::forceAv("Fatigue", 210.2f).find('.') == std::string::npos);
}

TEST(script_engine, refused_lines_are_counted) {
    RecHost h;
    h.consoleOk = false;
    ScriptActorEngine e(h);
    CHECK(!e.setRestrained(0x10, true));
    CHECK(!e.killActor(0x10));
    CHECK(e.linesRun() == 2 && e.linesFailed() == 2);
    h.consoleOk = true;
    CHECK(e.stopCombat(0x10));
    CHECK(e.linesRun() == 3 && e.linesFailed() == 2);
}
