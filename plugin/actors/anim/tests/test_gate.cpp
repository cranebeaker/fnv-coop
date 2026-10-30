// enemy-anim: the guest gate. The hooks module drops every mutating command that a console line (a temporary script) runs on a GUEST unless the line
// runs inside the sync layer's own scope, and it reports success for the dropped line. The playtest of 2026-10-30 (release 0.1.6, host and friend on one
// PC): the NPC copy on the friend stayed in the chair pose it was loaded in and slid around in it, while the host's NPC walked normally - every
// animation line of this layer (movement group, sneak, sit, stand-up, kill, corpse placement) was swallowed on the guest, and every unit test passed
// because no fake engine had a gate. These tests put the gate into the fake console.
// Mutation twin: FNV_MUT(88) in GuardedScript (the line runs bare).
#include "anim_helpers.h"
#include <cstring>
#include "script_npc_engine.h"

using namespace anim_test;

namespace {

constexpr uint32_t kChair = 0x00001A2Bu;

bool mutating(const std::string& line) {
    static const char* const kNames[] = {"PlayGroup", "SetForceSneak", "SetAlert", "Activate", "EvaluatePackage", "SetRestrained", "SetPos", "SetAngle", "KillActor", "Disable", "Enable"};
    for (const char* n : kNames)
        if (line.compare(0, std::strlen(n), n) == 0) return true;
    return false;
}

// A console on a guest: a mutating line only does something inside the scope; outside it, it is dropped and the console still answers true.
struct GuestConsole : INpcScript {
    bool guest = true;
    int scopesOpen = 0, scopesOpened = 0;
    std::vector<std::string> landed, swallowed;
    bool run(const std::string& l) {
        if (guest && mutating(l) && scopesOpen == 0) { swallowed.push_back(l); return true; }
        landed.push_back(l);
        return true;
    }
    bool Exec(NetId, const std::string& l) override { return run(l); }
    bool ExecOnRef(uint32_t, const std::string& l) override { return run(l); }
    GuardedScript::OpenScope opener() {
        return [this]() -> std::shared_ptr<void> {
            ++scopesOpen; ++scopesOpened;
            return std::shared_ptr<void>(this, [](GuestConsole* c) { --c->scopesOpen; });
        };
    }
};

struct GateAccess : INpcGameAccess {
    bool WorldReady() override { return true; }
    uint32_t WorldEpoch() override { return 1; }
    bool MenuOpen() override { return false; }
    bool ReadActor(NetId, NpcReads&) override { return false; }
    uint32_t Generation3D(NetId) override { return 1; }
    bool GetPose(NetId, CellId&, Vec3&, float&) override { return false; }
    Result SetWeaponOut(NetId, bool) override { return Result::Unsupported; }
    Result SetAiming(NetId, bool) override { return Result::Unsupported; }
    Result SetGroupRate(NetId, float) override { return Result::Unsupported; }
    Result KnockDown(NetId, float, float) override { return Result::Unsupported; }
    Result GetUp(NetId) override { return Result::Unsupported; }
    uint32_t RefIdOf(NetId) override { return 0x00ABCDEFu; }
};

}  // namespace

TEST(gate, every_line_the_layer_sends_lands_on_a_guest_when_it_runs_in_the_scope) {   // twin 88
    GuestConsole con;
    GuardedScript gs(con, con.opener());
    GateAccess ga;
    ScriptNpcAnimEngine e(gs, ga);
    CHECK(e.PlayGroup(kId, kGroupForward, true) == Result::Ok);
    CHECK(e.SetSneak(kId, true) == Result::Ok);
    CHECK(e.SitIn(kId, kChair) == Result::Ok);                       // the furniture line is run on the furniture reference
    CHECK(e.StandUp(kId, kChair, 0) == Result::Ok);
    CHECK(e.StandUp(kId, kChair, 1) == Result::Ok);                  // SetRestrained 0 + EvaluatePackage
    CHECK(e.Restrain(kId) == Result::Ok);
    CHECK(e.Kill(kId) == Result::Ok);
    CHECK(e.PlaceCorpse(kId, V(1, 2, 3), 0.5f) == Result::Ok);
    CHECK(con.swallowed.empty());                                    // nothing was dropped by the gate
    CHECK(con.landed.size() == 12);                                  // 1 + 1 + 1 + 1 + 2 + 1 + 1 + 4 lines
    CHECK(gs.lines() == 12 && gs.failed() == 0);
    CHECK(con.scopesOpen == 0 && con.scopesOpened == 12);            // one scope per line, each closed again
}

TEST(gate, the_same_lines_without_the_scope_are_swallowed_on_a_guest_and_run_on_the_host) {
    GuestConsole bare;                                               // what the layer did before 2026-10-30
    GateAccess ga;
    ScriptNpcAnimEngine e(bare, ga);
    CHECK(e.PlayGroup(kId, kGroupForward, true) == Result::Ok);      // the command answers Ok ...
    CHECK(e.StandUp(kId, kChair, 1) == Result::Ok);
    CHECK(bare.landed.empty() && bare.swallowed.size() == 3);        // ... and nothing happened: this is why the driver must read the engine back
    GuestConsole host;
    host.guest = false;
    ScriptNpcAnimEngine h(host, ga);
    CHECK(h.PlayGroup(kId, kGroupForward, true) == Result::Ok);
    CHECK(host.landed.size() == 1 && host.swallowed.empty());
}

TEST(gate, a_refused_line_is_counted_and_the_scope_is_still_closed) {
    struct Refusing : GuestConsole {
        bool Exec(NetId, const std::string&) override { return false; }
    } con;
    GuardedScript gs(con, con.opener());
    GateAccess ga;
    ScriptNpcAnimEngine e(gs, ga);
    CHECK(e.PlayGroup(kId, kGroupIdle, true) == Result::Refused);
    CHECK(gs.lines() == 1 && gs.failed() == 1 && con.scopesOpen == 0);
}
