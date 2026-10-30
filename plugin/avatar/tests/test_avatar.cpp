// Game-free tests of the player puppet driver (plugin/avatar), on a fake game that reproduces the engine behaviours the
// puppet traps come from (testing/fake_engine.h). Every trap in docs/tools/avatar.md has a test here, and every trap
// handler has a runtime-switchable mutation twin (avatar_faults.h): the whole suite is run once per twin and must fail.
//
//   avatar_tests              run the suite and every twin; prints "... ALL OK" when the suite passes and every twin is killed
//   avatar_tests --no-twins   suite only
//   avatar_tests --twin NAME  suite with one fault switched on (exit code 1 = killed, 0 = survived)
//   avatar_tests --list-twins
#include <cmath>
#include <cstdio>
#include <cstring>
#include <functional>
#include <string>
#include <vector>

#include "anim_planner.h"
#include "avatar_faults.h"
#include "avatar_groups.h"
#include "live_bridge.h"
#include "avatar_api.h"
#include "loco.h"
#include "puppet_hits.h"
#include "puppet_manager.h"
#include "script_engine.h"
#include "testing/fake_engine.h"

using namespace nvcoop::avatar;

static int g_fail = 0, g_checks = 0;
static bool g_quiet = false;
static const char* g_test = "";
#define CHECK(c)                                                                      \
    do {                                                                              \
        ++g_checks;                                                                   \
        if (!(c)) {                                                                   \
            ++g_fail;                                                                 \
            if (!g_quiet) std::printf("  FAIL [%s] %s:%d  %s\n", g_test, __FILE__, __LINE__, #c); \
        }                                                                             \
    } while (0)

// ---- rig ------------------------------------------------------------------------------------------------------------

static constexpr CellId kCellA = 0x10, kCellB = 0x20, kExtA = 0x100, kExtB = 0x101;

static Config MakeCfg() {
    Config c;
    c.localSlot = 0;
    c.templates = {{1, false, 0x1000}, {1, true, 0x1001}};
    c.defaultTemplate = 0x1002;
    return c;
}

static Appearance App(bool female = false) {
    Appearance a;
    a.race = 1;
    a.female = female;
    a.name = "Courier";
    return a;
}

static PuppetPose MkPose(CellId cell, float x, float y, float z = 0, float yaw = 0, uint8_t loco = kLocoIdle, Vec3 vel = Vec3{}) {
    PuppetPose p;
    p.valid = true;
    p.cell = cell;
    p.pos = Vec3{x, y, z};
    p.rot = Vec3{0, yaw, 0};
    p.vel = vel;
    p.anim.loco = loco;
    return p;
}

struct Rig {
    FakePuppetEngine eng;
    Config cfg;
    PuppetManager pm;
    uint32_t now = 1000;
    bool autoPose = true;
    std::map<Slot, PuppetPose> cur;

    std::vector<std::string> log;
    bool Logged(const char* needle) const {
        for (const auto& l : log)
            if (l.find(needle) != std::string::npos) return true;
        return false;
    }
    explicit Rig(const std::function<void(Config&)>& tweak = nullptr) : cfg(Make(tweak)), pm(eng, cfg) {
        pm.SetLog([this](const std::string& l) { log.push_back(l); });
        eng.loadedCells = {kCellA, kCellB, kExtA, kExtB};
        eng.exterior = {kExtA, kExtB};
    }
    static Config Make(const std::function<void(Config&)>& tweak) {
        Config c = MakeCfg();
        if (tweak) tweak(c);
        return c;
    }
    void Pose(Slot s, const PuppetPose& p) {
        cur[s] = p;
        pm.SetPose(s, p);
    }
    void Add(Slot s, const PuppetPose& p, bool female = false) {
        pm.AddPlayer(s, App(female));
        Pose(s, p);
    }
    void Step(uint32_t ms = 50) {
        eng.Frame();
        if (autoPose)
            for (auto& kv : cur) pm.SetPose(kv.first, kv.second);
        pm.Tick(now);
        now += ms;
    }
    void Run(uint32_t ms, uint32_t step = 50) {
        for (uint32_t t = 0; t < ms; t += step) Step(step);
    }
    bool UntilDriving(Slot s, int maxSteps = 100) {
        for (int i = 0; i < maxSteps && pm.PhaseOf(s) != Phase::Driving; ++i) Step();
        return pm.PhaseOf(s) == Phase::Driving;
    }
    // The puppet's fake ref; a blank, non-existing ref when there is none, so a broken twin fails checks instead of crashing.
    FakePuppetEngine::FakeRef* F(Slot s) {
        static FakePuppetEngine::FakeRef none;
        none = FakePuppetEngine::FakeRef{};
        auto* f = eng.Get(pm.RefOf(s));
        return f ? f : &none;
    }
    int Group(Slot s) { auto* f = F(s); return f ? f->group : -1; }
};

static std::vector<int> GroupSeq(const Rig& r) {
    std::vector<int> v;
    for (auto& g : r.eng.groupLog)
        if (v.empty() || v.back() != g.second) v.push_back(g.second);
    return v;
}

// ---- planner / loco unit tests --------------------------------------------------------------------------------------

struct PlanRig {
    AnimPlanner p;
    uint32_t t = 0;
    PlanOut Step(PlanInput in, uint32_t ms = 50) {
        in.nowMs = t;
        PlanOut o = p.Plan(in);
        t += ms;
        return o;
    }
    PlanOut Hold(PlanInput in, uint32_t ms) {
        PlanOut o;
        for (uint32_t i = 0; i < ms; i += 50) o = Step(in);
        return o;
    }
};
static PlanInput PIn(uint8_t loco, Vec3 vel = Vec3{}, float yaw = 0) {
    PlanInput i;
    i.anim.loco = loco;
    i.vel = vel;
    i.yaw = yaw;
    return i;
}

static void t_loco_basics() {
    g_test = "loco_basics";
    LocalRead r;
    r.vel = Vec3{0, 10, 0};
    CHECK(CaptureLoco(r) == kLocoIdle);
    r.vel = Vec3{0, 100, 0};
    CHECK(CaptureLoco(r) == kLocoWalkFwd);
    r.vel = Vec3{0, 300, 0};
    CHECK(CaptureLoco(r) == kLocoRunFwd);
    r.vel = Vec3{0, -100, 0};
    CHECK(CaptureLoco(r) == kLocoWalkBack);
    r.vel = Vec3{100, 0, 0};
    CHECK(CaptureLoco(r) == kLocoWalkRight);   // yaw 0 faces +Y, +X is to the right
    r.vel = Vec3{-100, 0, 0};
    CHECK(CaptureLoco(r) == kLocoWalkLeft);
    r.yaw = kPi * 0.5f;                       // facing +X: +X is forward
    r.vel = Vec3{100, 0, 0};
    CHECK(CaptureLoco(r) == kLocoWalkFwd);
    r.yaw = kPi;                              // facing -Y
    r.vel = Vec3{0, -100, 0};
    CHECK(CaptureLoco(r) == kLocoWalkFwd);
    r.yaw = 0;
    r.vel = Vec3{0, 100, 0};
    r.haveMoveFlags = true;
    r.moverRun = true;
    CHECK(CaptureLoco(r) == kLocoRunFwd);     // the mover flags decide walk vs run when known
    r.moverRun = false;
    r.moverWalk = true;
    CHECK(CaptureLoco(r) == kLocoWalkFwd);

    // direction hysteresis near the 45 degree edge
    float a47 = 47.0f * kPi / 180.0f;
    Vec3 v{std::sin(a47) * 100.f, std::cos(a47) * 100.f, 0};
    CHECK(DirectionOf(v, 0, -1) == 3);        // fresh: right
    CHECK(DirectionOf(v, 0, 0) == 0);         // was forward: stays forward
    float a60 = 60.0f * kPi / 180.0f;
    Vec3 v2{std::sin(a60) * 100.f, std::cos(a60) * 100.f, 0};
    CHECK(DirectionOf(v2, 0, 0) == 3);        // clearly right now
    CHECK(DirectionOf(Vec3{0, -100, 0}, 0, -1) == 1);
    CHECK(std::fabs(WrapRad(7.0f) - (7.0f - kTwoPi)) < 1e-5f);
    CHECK(WrapRad(-kPi) > 0);                 // (-pi, pi]
    CHECK(WrapRad(std::nanf("")) == 0);
    for (uint8_t l = 1; l <= kLocoMax; ++l) CHECK(LocoIsMoving(l) && GroupForLoco(l) != kGrpIdle && IsLocomotionGroup(GroupForLoco(l)));
    CHECK(GroupForLoco(kLocoIdle) == kGrpIdle);
    CHECK(LocoDir(kLocoRunLeft) == 2 && LocoDir(kLocoWalkBack) == 1 && LocoDir(kLocoIdle) == -1);
}

static void t_planner_groups() {
    g_test = "planner_groups";
    { PlanRig p; CHECK(p.Hold(PIn(kLocoIdle), 200).group == kGrpIdle); }
    { PlanRig p; CHECK(p.Hold(PIn(kLocoWalkFwd, Vec3{0, 120, 0}), 200).group == kGrpForward); }
    { PlanRig p; CHECK(p.Hold(PIn(kLocoRunFwd, Vec3{0, 300, 0}), 200).group == kGrpFastForward); }
    { PlanRig p; CHECK(p.Hold(PIn(kLocoWalkBack, Vec3{0, -100, 0}), 200).group == kGrpBackward); }
    { PlanRig p; CHECK(p.Hold(PIn(kLocoWalkLeft, Vec3{-100, 0, 0}), 200).group == kGrpLeft); }
    { PlanRig p; CHECK(p.Hold(PIn(kLocoRunRight, Vec3{300, 0, 0}), 200).group == kGrpFastRight); }
    { PlanRig p; CHECK(p.Hold(PIn(kLocoRunBack, Vec3{0, -300, 0}), 200).group == kGrpFastBackward); }
    { PlanRig p; CHECK(p.Hold(PIn(kLocoRunLeft, Vec3{-300, 0, 0}), 200).group == kGrpFastLeft); }

    // reload beats locomotion
    { PlanRig p; PlanInput i = PIn(kLocoIdle); i.anim.reload = true; CHECK(p.Hold(i, 100).group == kGrpReloadA); }
    // aim stances and pitch
    {
        PlanRig p;
        PlanInput i = PIn(kLocoIdle);
        i.anim.weaponDrawn = true;
        p.Hold(i, 600);
        CHECK(p.Hold(i, 100).group == kGrpAim);
        i.pitch = 0.5f;
        CHECK(p.Step(i).group == kGrpAimUp);
        i.pitch = -0.5f;
        CHECK(p.Step(i).group == kGrpAimDown);
        i.anim.aim = true;
        CHECK(p.Step(i).group == kGrpAimISDown);
        i.pitch = 0.5f;
        CHECK(p.Step(i).group == kGrpAimISUp);
        i.pitch = 0;
        CHECK(p.Step(i).group == kGrpAimIS);
    }
    // draw and holster play their transition, then settle
    {
        PlanRig p;
        PlanInput i = PIn(kLocoIdle);
        p.Hold(i, 200);
        i.anim.weaponDrawn = true;
        CHECK(p.Step(i).group == kGrpEquip);
        CHECK(p.Hold(i, 600).group == kGrpAim);
        i.anim.weaponDrawn = false;
        CHECK(p.Step(i).group == kGrpUnequip);
        CHECK(p.Hold(i, 600).group == kGrpIdle);
    }
    // sneak and the drawn flag are passed through
    { PlanRig p; PlanInput i = PIn(kLocoWalkFwd, Vec3{0, 100, 0}); i.anim.sneak = true; i.anim.weaponDrawn = true; PlanOut o = p.Step(i); CHECK(o.sneak && o.weaponDrawn); }
}

static void t_speed_matching_planner() {
    g_test = "speed_matching_planner";
    // idle code with real speed: corrected only after the debounce (no flicker), then walking in the right direction
    {
        PlanRig p;
        PlanInput i = PIn(kLocoIdle, Vec3{0, 150, 0});
        CHECK(p.Step(i).group == kGrpIdle);   // t=0: debounce starts
        CHECK(p.Step(i).group == kGrpIdle);   // 50
        CHECK(p.Step(i).group == kGrpIdle);   // 100
        PlanOut o;
        for (int k = 0; k < 4; ++k) o = p.Step(i);   // 150..300
        CHECK(o.group == kGrpForward && o.speedCorrected);
    }
    // direction of the correction comes from the velocity relative to the facing
    { PlanRig p; CHECK(p.Hold(PIn(kLocoIdle, Vec3{150, 0, 0}, 0), 400).group == kGrpRight); }
    { PlanRig p; CHECK(p.Hold(PIn(kLocoIdle, Vec3{150, 0, 0}, kPi * 0.5f), 400).group == kGrpForward); }
    { PlanRig p; CHECK(p.Hold(PIn(kLocoIdle, Vec3{0, -150, 0}, 0), 400).group == kGrpBackward); }
    // run code but walking speed: legs must not outrun the ground
    { PlanRig p; CHECK(p.Hold(PIn(kLocoRunFwd, Vec3{0, 110, 0}), 400).group == kGrpForward); }
    // walk code but running speed
    { PlanRig p; CHECK(p.Hold(PIn(kLocoWalkFwd, Vec3{0, 400, 0}), 400).group == kGrpFastForward); }
    // a moving code with no velocity (running into a wall) is kept as the owner shows it
    { PlanRig p; CHECK(p.Hold(PIn(kLocoRunFwd, Vec3{}), 600).group == kGrpFastForward); }
    // hysteresis: speeds between runExit and runEnter keep whatever it was doing
    {
        PlanRig p;
        PlanOut o;
        for (int k = 0; k < 20; ++k) o = p.Step(PIn(kLocoRunFwd, Vec3{0, k % 2 ? 200.f : 225.f, 0}));
        CHECK(o.group == kGrpFastForward);
        PlanRig q;
        for (int k = 0; k < 20; ++k) o = q.Step(PIn(kLocoWalkFwd, Vec3{0, k % 2 ? 200.f : 225.f, 0}));
        CHECK(o.group == kGrpForward);
    }
    // jump: start, loop, land, back to the ground pose; forward variants when moving
    {
        PlanRig p;
        PlanInput i = PIn(kLocoIdle);
        p.Hold(i, 200);
        i.anim.jump = true;
        CHECK(p.Step(i).group == kGrpJumpStart);
        CHECK(p.Hold(i, 400).group == kGrpJumpLoop);
        i.anim.jump = false;
        CHECK(p.Step(i).group == kGrpJumpLand);
        CHECK(p.Hold(i, 500).group == kGrpIdle);
        PlanRig q;
        PlanInput m = PIn(kLocoRunFwd, Vec3{0, 300, 0});
        q.Hold(m, 200);
        m.anim.jump = true;
        q.Step(m);
        CHECK(q.Hold(m, 400).group == kGrpJumpLoopForward);
        m.anim.jump = false;
        CHECK(q.Step(m).group == kGrpJumpLandForward);
    }
    // frozen: never a moving pose
    {
        PlanRig p;
        PlanInput i = PIn(kLocoRunFwd, Vec3{0, 300, 0});
        p.Hold(i, 300);
        i.frozen = true;
        CHECK(p.Step(i).group == kGrpIdle);
    }
}

// ---- trap tests on the manager --------------------------------------------------------------------------------------

// Trap: a restrained / just-spawned actor refuses commands until its 3D exists.
static void t_spawn_waits_for_3d() {
    g_test = "spawn_waits_for_3d";
    Rig r;
    r.eng.load3DFrames = 4;
    r.Add(1, MkPose(kCellA, 100, 200, 30, 0.5f));
    r.Run(1500);
    CHECK(r.pm.PhaseOf(1) == Phase::Driving);
    CHECK(r.eng.refusedNo3D == 0);
    CHECK(r.eng.violations.empty());
    CHECK(r.eng.spawnCalls == 1);
    auto* f = r.F(1);
    CHECK(f && f->restrained && f->nonCombatant && f->appearanceApplied);
    CHECK(f && std::fabs(f->pos.x - 100) < 0.01f && std::fabs(f->pos.z - 30) < 0.01f);
    CHECK(f && f->base == 0x1000);
    // restraint is the first command after the spawn, before any position or animation write
    size_t iRestrain = 0, iPos = 0, iGroup = 0;
    for (size_t i = 0; i < r.eng.trace.size(); ++i) {
        if (!iRestrain && r.eng.trace[i] == "Restrain") iRestrain = i + 1;
        if (!iPos && r.eng.trace[i] == "SetPos") iPos = i + 1;
        if (!iGroup && r.eng.trace[i].rfind("PlayGroup", 0) == 0) iGroup = i + 1;
    }
    CHECK(iRestrain && iPos && iGroup && iRestrain < iPos && iRestrain < iGroup);
}

// Trap: the actor's AI takes a loose actor over (walks home, fights, sits down) -> restraint must be re-asserted.
static void t_restraint_reasserted() {
    g_test = "restraint_reasserted";
    Rig r;
    r.Add(1, MkPose(kCellA, 100, 200));
    CHECK(r.UntilDriving(1));
    r.Run(300);
    RefId ref = r.pm.RefOf(1);
    r.eng.Unrestrain(ref);
    r.Run(600);
    CHECK(r.eng.IsRestrained(ref));
    CHECK(r.pm.stats().restrainReasserts == 1);
}

// Trap: animation is reset by menus (Pip-Boy, dialogue): re-assert at once when the menu closes, and jump to the latest pose.
static void t_anim_after_menu() {
    g_test = "anim_after_menu";
    Rig r;
    r.eng.reportsCurrentGroup = false;   // no readback: only the menu event can know
    r.Add(1, MkPose(kCellA, 100, 200, 0, 0, kLocoWalkFwd, Vec3{0, 120, 0}));
    r.Run(2000);
    CHECK(r.Group(1) == kGrpForward);
    r.eng.SetMenu(true);
    r.Step();
    unsigned w = r.eng.Writes();
    r.Run(300);
    CHECK(r.eng.Writes() == w);          // nothing is sent into a paused / menu state
    r.eng.SetMenu(false);                // closing the menu resets the animation silently
    CHECK(r.Group(1) == kGrpIdle);
    r.Step();
    CHECK(r.Group(1) == kGrpForward);    // replayed on the first Tick after the menu closed
}

// Trap: the menu left the puppet near, but not at, where it should be; it is moved to the latest pose immediately.
static void t_post_menu_snap() {
    g_test = "post_menu_snap";
    Rig r([](Config& c) { c.posHeartbeatMs = 1000000; });   // the periodic re-send must not hide a missing post-menu snap
    r.eng.menuDisplacement = Vec3{100, 0, 0};   // below the drift limit, so only the menu event can fix it at once
    r.Add(1, MkPose(kCellA, 1000, 2000));
    r.Run(2000);
    r.eng.SetMenu(true);
    r.Run(200);
    r.eng.SetMenu(false);
    CHECK(std::fabs(r.F(1)->pos.x - 1100) < 0.01f);
    r.Step();
    CHECK(std::fabs(r.F(1)->pos.x - 1000) < 0.01f);
}

// Trap: the engine forgets the group without any event and cannot say what plays: the heartbeat re-issues it.
static void t_anim_heartbeat() {
    g_test = "anim_heartbeat";
    Rig r;
    r.eng.reportsCurrentGroup = false;
    r.Add(1, MkPose(kCellA, 100, 200, 0, 0, kLocoWalkFwd, Vec3{0, 120, 0}));
    r.Run(1500);
    r.eng.SilentGroupReset(r.pm.RefOf(1));
    r.Run(r.cfg.animHeartbeatMs + 300);
    CHECK(r.Group(1) == kGrpForward);
    // with readback the mismatch is found much sooner
    Rig q;
    q.Add(1, MkPose(kCellA, 100, 200, 0, 0, kLocoWalkFwd, Vec3{0, 120, 0}));
    q.Run(1500);
    q.eng.SilentGroupReset(q.pm.RefOf(1));
    q.Run(q.cfg.animMismatchMs + 200);
    CHECK(q.Group(1) == kGrpForward);
    // groups that end on their own (equip, jump start, reload) are not "re-asserted" when the engine returns to idle
    Rig o;
    PuppetPose p = MkPose(kCellA, 100, 200);
    p.anim.reload = true;
    o.Add(1, p);
    o.Run(1000);
    unsigned before = o.eng.playGroupCalls;
    o.eng.SilentGroupReset(o.pm.RefOf(1));
    o.Run(800);
    CHECK(o.eng.playGroupCalls == before);
}

// Trap: slide. Owner code says idle but the velocity says walking: legs must follow the ground speed.
static void t_no_slide_manager() {
    g_test = "no_slide";
    Rig r;
    r.Add(1, MkPose(kCellA, 100, 200, 0, 0, kLocoIdle, Vec3{0, 150, 0}));
    r.Run(600);
    CHECK(r.Group(1) == kGrpForward);
    r.Pose(1, MkPose(kCellA, 100, 200, 0, 0, kLocoRunFwd, Vec3{0, 100, 0}));
    r.Run(600);
    CHECK(r.Group(1) == kGrpForward);        // run legs at walking speed would skate
    r.Pose(1, MkPose(kCellA, 100, 200, 0, 0, kLocoWalkFwd, Vec3{0, 400, 0}));
    r.Run(600);
    CHECK(r.Group(1) == kGrpFastForward);
    r.Pose(1, MkPose(kCellA, 100, 200, 0, 0, kLocoRunFwd, Vec3{}));
    r.Run(600);
    CHECK(r.Group(1) == kGrpFastForward);    // running on the spot against a wall stays as shown
}

// Trap: the remote stops sending (alt-tab, lag): a dry buffer must stop the legs at once, never run in place.
static void t_frozen_and_stale() {
    g_test = "frozen_and_stale";
    Rig r;
    r.Add(1, MkPose(kCellA, 100, 200, 0, 0, kLocoRunFwd, Vec3{0, 300, 0}));
    r.Run(600);
    CHECK(r.Group(1) == kGrpFastForward);
    PuppetPose fz = MkPose(kCellA, 100, 200, 0, 0, kLocoRunFwd, Vec3{0, 300, 0});
    fz.frozen = true;
    r.Pose(1, fz);
    r.Run(150);
    CHECK(r.Group(1) == kGrpIdle);
    CHECK(r.pm.stats().staleFrozen == 1);
    // and a pose that simply stops arriving
    Rig s;
    s.autoPose = false;
    s.Add(1, MkPose(kCellA, 100, 200, 0, 0, kLocoRunFwd, Vec3{0, 300, 0}));
    s.Run(1000);
    CHECK(s.Group(1) == kGrpFastForward);
    s.Run(s.cfg.staleMs + 500);
    CHECK(s.Group(1) == kGrpIdle);
}

static void t_jump_sequence() {
    g_test = "jump_sequence";
    Rig r;
    r.Add(1, MkPose(kCellA, 100, 200));
    r.Run(500);
    PuppetPose j = MkPose(kCellA, 100, 200, 40);
    j.anim.jump = true;
    r.Pose(1, j);
    r.Run(700);
    r.Pose(1, MkPose(kCellA, 100, 200, 0));
    r.Run(900);
    std::vector<int> seq = GroupSeq(r);
    std::vector<int> want = {kGrpIdle, kGrpJumpStart, kGrpJumpLoop, kGrpJumpLand, kGrpIdle};
    CHECK(seq == want);
}

// Trap: drawing a weapon the puppet does not hold yet (T-pose, fists out). Weapon is drawn only once equipped.
static void t_weapon_draw_after_equip() {
    g_test = "weapon_draw_after_equip";
    Rig r;
    r.eng.weaponItems = {0x600};
    PuppetPose p = MkPose(kCellA, 100, 200);
    p.anim.weaponDrawn = true;
    r.Add(1, p);
    r.pm.SetEquipment(1, {{0x600, true}});
    CHECK(r.UntilDriving(1));
    r.Step();
    CHECK(r.eng.drawOnCalls == 0);
    CHECK(r.Group(1) != kGrpAim && r.Group(1) != kGrpEquip);
    r.Run(3000);
    CHECK(r.F(1)->worn.count(0x600) == 1);
    CHECK(r.eng.drawOnCalls >= 1);
    CHECK(r.F(1)->drawn);
    CHECK(r.eng.violations.empty());
    CHECK(r.Group(1) == kGrpAim);
}

// Trap (VaultMP list): own the item first; extras off; slot replacement; weapon last.
static void t_equipment_converges() {
    g_test = "equipment_converges";
    Rig r;
    r.eng.defaultOutfit = {0x500};
    r.eng.slotOf = {{0x500, 1}, {0x510, 1}, {0x600, 2}};
    r.eng.weaponItems = {0x600};
    r.Add(1, MkPose(kCellA, 100, 200));
    r.pm.SetEquipment(1, {{0x600, true}, {0x510, false}});
    r.Run(6000);
    auto* f = r.F(1);
    CHECK(f && f->worn == (std::set<FormId>{0x510, 0x600}));
    CHECK(r.pm.EquipConverged(1));
    CHECK(r.eng.addCalls == 2);
    CHECK(r.pm.stats().equipGaveUp == 0);
    // armor is on before the weapon
    // change the wanted set later: converges again
    r.pm.SetEquipment(1, {{0x520, false}});
    r.Run(6000);
    f = r.F(1);
    CHECK(f && f->worn == (std::set<FormId>{0x520}));
    CHECK(r.pm.EquipConverged(1));
}

static void t_equipment_backoff_giveup() {
    g_test = "equipment_backoff_giveup";
    Rig r;
    r.eng.equipFailuresLeft = 100000;
    r.Add(1, MkPose(kCellA, 100, 200));
    r.pm.SetEquipment(1, {{0x510, false}});
    r.Run(15000);
    CHECK(r.pm.stats().equipFailures <= 8);
    CHECK(r.pm.stats().equipGaveUp == 1);
    CHECK(!r.pm.EquipConverged(1));
    // a changed wanted set clears the give-up
    r.eng.equipFailuresLeft = 0;
    r.pm.SetEquipment(1, {{0x511, false}});
    r.Run(4000);
    CHECK(r.F(1)->worn.count(0x511) == 1);
}

// Trap: the actor's AI puts the default outfit back on again and again: break the fight with RemoveItem.
static void t_equipment_flap_guard() {
    g_test = "equipment_flap_guard";
    Rig r;
    r.eng.defaultOutfit = {0x500};
    r.eng.aiReequipLeft[0x500] = 100000;
    r.Add(1, MkPose(kCellA, 100, 200));
    r.pm.SetEquipment(1, {{0x510, false}});
    r.Run(8000);
    auto* f = r.F(1);
    CHECK(f && f->worn == (std::set<FormId>{0x510}));
    CHECK(r.pm.stats().equipFlapRemovals >= 1);
    CHECK(r.pm.EquipConverged(1));
}

static void t_local_slot_and_limits() {
    g_test = "local_slot_and_limits";
    Rig r;
    CHECK(!r.pm.AddPlayer(0, App()));
    CHECK(r.pm.stats().rejectedLocalSlot == 1);
    r.Pose(0, MkPose(kCellA, 1, 1));
    r.Run(1000);
    CHECK(r.eng.spawnCalls == 0 && r.eng.LiveCount() == 0);
    CHECK(r.pm.AddPlayer(1, App()) && r.pm.AddPlayer(2, App()) && r.pm.AddPlayer(3, App()));
    CHECK(!r.pm.AddPlayer(4, App()));   // 4 players = 3 puppets
    CHECK(r.pm.Count() == 3);
}

// Trap: reconnect / double join must leave exactly one puppet per slot.
static void t_duplicate_slot() {
    g_test = "duplicate_slot";
    Rig r;
    r.Add(1, MkPose(kCellA, 100, 200));
    CHECK(r.UntilDriving(1));
    RefId first = r.pm.RefOf(1);
    r.Add(1, MkPose(kCellA, 300, 400));   // the player reconnected
    r.Run(1500);
    CHECK(r.eng.LiveCount() == 1);
    CHECK(r.pm.RefOf(1) != 0 && r.pm.RefOf(1) != first);
    CHECK(!r.pm.SlotOf(first).has_value());
    CHECK(r.pm.stats().replacedDuplicates == 1);
}

static void t_remove_player() {
    g_test = "remove_player";
    Rig r;
    r.Add(1, MkPose(kCellA, 100, 200));
    r.Add(2, MkPose(kCellA, 300, 200));
    r.Run(1500);
    RefId a = r.pm.RefOf(1);
    CHECK(r.eng.LiveCount() == 2);
    r.pm.RemovePlayer(1);
    CHECK(r.eng.LiveCount() == 1);
    CHECK(r.pm.RefOf(1) == 0 && !r.pm.SlotOf(a).has_value());
    CHECK(r.pm.RefOf(2) != 0);
    r.Run(1000);
    CHECK(r.eng.LiveCount() == 1);
}

// Trap: the ref is gone (cell unloaded) or its id was reused by something else: never command it, respawn instead.
static void t_stale_and_reused_refs() {
    g_test = "stale_and_reused_refs";
    {
        Rig r;
        r.Add(1, MkPose(kCellA, 100, 200));
        CHECK(r.UntilDriving(1));
        RefId ref = r.pm.RefOf(1);
        r.eng.KillRef(ref);
        r.Run(1500);
        CHECK(r.eng.violations.empty());
        CHECK(r.pm.RefOf(1) != 0 && r.pm.RefOf(1) != ref);
        CHECK(r.eng.LiveCount() == 1);
        CHECK(r.pm.stats().refLost == 1);
    }
    {
        Rig r;
        r.eng.reuseRefIds = true;
        r.Add(1, MkPose(kCellA, 100, 200));
        CHECK(r.UntilDriving(1));
        RefId ref = r.pm.RefOf(1);
        r.eng.KillRef(ref);
        r.eng.HijackId(ref);   // the engine hands the id to an unrelated object
        r.Run(1500);
        CHECK(r.eng.commandsOnForeign == 0);
        CHECK(r.eng.violations.empty());
        CHECK(r.pm.RefOf(1) != ref);
        CHECK(r.eng.Get(ref) && r.eng.Get(ref)->base == 0xDEAD && r.eng.Get(ref)->exists);   // untouched
    }
}

// Trap: the remote is in a cell this machine has not loaded: no puppet floating in the void; it comes back with the cell.
static void t_park_when_cell_unloaded() {
    g_test = "park_when_cell_unloaded";
    Rig r;
    r.Add(1, MkPose(kCellA, 100, 200));
    r.Run(1500);
    CHECK(r.eng.LiveCount() == 1);
    r.eng.loadedCells.erase(kCellA);
    r.Run(300);
    CHECK(r.eng.LiveCount() == 1);       // grace period: a one-frame hiccup does not delete it
    r.Run(500);
    CHECK(r.eng.LiveCount() == 0);
    CHECK(r.pm.stats().parked == 1);
    CHECK(r.eng.violations.empty());
    r.Run(1000);
    CHECK(r.eng.LiveCount() == 0 && r.eng.spawnCalls == 1);   // not respawned into an unloaded cell
    r.eng.loadedCells.insert(kCellA);
    r.Run(1500);
    CHECK(r.eng.LiveCount() == 1 && r.pm.stats().unparked == 1);
    CHECK(std::fabs(r.F(1)->pos.x - 100) < 0.01f);
}

// Trap: interior to interior (or any other space): SetPos would leave the actor in the old cell. MoveToCell, then the
// rebuilt 3D loses the restraint and the animation, which must come back.
static void t_cross_cell_move() {
    g_test = "cross_cell_move";
    Rig r;
    r.eng.cellOfPos = [](const Vec3& p) { return p.x >= 10000 ? kCellB : kCellA; };
    r.Add(1, MkPose(kCellA, 100, 200, 0, 0, kLocoWalkFwd, Vec3{0, 120, 0}));
    r.Run(1500);
    CHECK(r.F(1)->cell == kCellA);
    r.Pose(1, MkPose(kCellB, 20000, 300, 0, 1.0f, kLocoWalkFwd, Vec3{0, 120, 0}));
    r.Run(1500);
    CHECK(r.eng.violations.empty());
    CHECK(r.eng.moveToCellCalls == 1);
    CHECK(r.F(1)->cell == kCellB);
    CHECK(r.F(1)->restrained);
    CHECK(r.pm.PhaseOf(1) == Phase::Driving);
    CHECK(std::fabs(r.F(1)->pos.x - 20000) < 0.01f);
    CHECK(r.Group(1) == kGrpForward);
    CHECK(r.eng.refusedNo3D == 0);
    CHECK(r.pm.stats().teleports >= 1);
}

// Exterior neighbours share a worldspace: a plain SetPos is enough, no 3D rebuild, no flicker.
static void t_exterior_border() {
    g_test = "exterior_border";
    Rig r;
    r.Add(1, MkPose(kExtA, 100, 200, 0, 0, kLocoWalkFwd, Vec3{0, 120, 0}));
    r.Run(1500);
    r.Pose(1, MkPose(kExtB, 4200, 200, 0, 0, kLocoWalkFwd, Vec3{0, 120, 0}));
    r.Step();
    CHECK(r.pm.PhaseOf(1) == Phase::Driving);
    r.Run(500);
    CHECK(r.eng.moveToCellCalls == 0);
    CHECK(std::fabs(r.F(1)->pos.x - 4200) < 0.01f);
    CHECK(r.pm.PhaseOf(1) == Phase::Driving);
}

// Trap: cell reload rebuilds the 3D without a gap: the generation changes and the pose / animation must be re-sent.
static void t_rebuild_3d_reasserts() {
    g_test = "rebuild_3d_reasserts";
    Rig r;
    r.eng.reportsCurrentGroup = false;
    r.Add(1, MkPose(kCellA, 100, 200, 0, 0, kLocoWalkFwd, Vec3{0, 120, 0}));
    r.Run(1500);
    CHECK(r.Group(1) == kGrpForward);
    r.eng.Rebuild3DInstant(r.pm.RefOf(1));
    CHECK(r.Group(1) == kGrpIdle);
    r.Step();
    CHECK(r.Group(1) == kGrpForward);
}

// Trap: never saved. The save writer must find no puppet; they come back afterwards.
static void t_save_despawns() {
    g_test = "save_despawns";
    Rig r;
    r.Add(1, MkPose(kCellA, 100, 200));
    r.Add(2, MkPose(kCellA, 300, 200));
    r.Run(1500);
    CHECK(r.eng.LiveCount() == 2);
    r.pm.OnSaveBegin();
    r.Run(400);   // frames while the save is being written
    CHECK(r.eng.SaveImage().empty());
    CHECK(r.pm.stats().saveDespawns == 2);
    r.pm.OnSaveEnd();
    r.Run(1500);
    CHECK(r.eng.LiveCount() == 2);
    CHECK(r.pm.PhaseOf(1) == Phase::Driving && r.pm.PhaseOf(2) == Phase::Driving);
}

// Trap: purged on load. Orphans from an old save are deleted, our puppets are rebuilt, and nothing is sent into the old world.
static void t_load_purges_orphans() {
    g_test = "load_purges_orphans";
    Rig r;
    r.Add(1, MkPose(kCellA, 100, 200));
    r.Run(1500);
    std::vector<RefId> image = r.eng.SaveImage();   // a save that (wrongly) contains the puppet, e.g. after a crash
    CHECK(image.size() == 1);
    r.pm.OnLoadBegin();
    unsigned w = r.eng.Writes();
    r.Run(200);
    CHECK(r.eng.Writes() == w);                     // nothing is sent while the world is torn down
    CHECK(r.pm.RefOf(1) == 0);
    r.eng.Load({image[0], image[0]});               // the old save brings back two puppets
    r.pm.OnLoadEnd();
    CHECK(r.pm.PurgePending());
    r.Run(1500);
    CHECK(r.pm.stats().purged == 2);
    CHECK(r.eng.LiveCount() == 1);                  // only the fresh puppet
    CHECK(r.pm.PhaseOf(1) == Phase::Driving);
    CHECK(r.eng.violations.empty());

    // a load the game did not announce: the world epoch is the backstop
    Rig q;
    q.Add(1, MkPose(kCellA, 100, 200));
    q.Run(1500);
    q.eng.Load(q.eng.SaveImage());
    q.Run(1500);
    CHECK(q.eng.LiveCount() == 1);
    CHECK(q.pm.stats().purged == 1);
    CHECK(q.eng.violations.empty());
}

static void t_first_tick_purge() {
    g_test = "first_tick_purge";
    Rig r;
    r.eng.PlantOrphan(kCellA, Vec3{});
    r.eng.PlantOrphan(kCellA, Vec3{});
    r.Run(200);
    CHECK(r.eng.LiveCount() == 0);
    CHECK(r.pm.stats().purged == 2);
    // not while the game is not ready
    Rig q;
    q.eng.worldReady = false;
    q.eng.PlantOrphan(kCellA, Vec3{});
    q.Add(1, MkPose(kCellA, 1, 1));
    q.Run(500);
    CHECK(q.eng.LiveCount() == 1 && q.eng.Writes() == 0 && q.eng.spawnCalls == 0);
    q.eng.worldReady = true;
    q.Run(500);
    CHECK(q.eng.LiveCount() == 1 && q.pm.stats().purged == 1);   // the orphan is gone, the puppet is the one left
}

static void t_spawn_retry() {
    g_test = "spawn_retry";
    Rig r;
    r.eng.spawnFailuresLeft = 3;
    r.Add(1, MkPose(kCellA, 100, 200));
    r.Run(800);
    CHECK(r.eng.spawnCalls <= 3);              // backoff 500, 1000, 2000: not one attempt per frame
    r.Run(6000);
    CHECK(r.pm.PhaseOf(1) == Phase::Driving);
    CHECK(r.pm.stats().spawnFailures == 3 && r.eng.spawnCalls == 4);

    Rig q;
    q.eng.spawnFailuresLeft = 1000000;
    q.Add(1, MkPose(kCellA, 100, 200));
    q.Run(120000, 100);
    CHECK(q.pm.PhaseOf(1) == Phase::Failed);
    CHECK(q.eng.spawnCalls == q.cfg.maxSpawnAttempts);

    // no template at all
    Rig t([](Config& c) { c.templates.clear(); c.defaultTemplate = 0; });
    t.Add(1, MkPose(kCellA, 1, 1));
    t.Run(300);
    CHECK(t.pm.PhaseOf(1) == Phase::Failed && t.eng.spawnCalls == 0);

    // a spawned ref that never gets 3D is replaced
    Rig s;
    s.eng.load3DFrames = 100000;
    s.Add(1, MkPose(kCellA, 100, 200));
    s.Run(s.cfg.load3DTimeoutMs + 1500);
    CHECK(s.pm.stats().load3DTimeouts >= 1);
    CHECK(s.eng.LiveCount() <= 1);
}

static void t_template_choice() {
    g_test = "template_choice";
    Rig r;
    r.Add(1, MkPose(kCellA, 1, 1), false);
    r.Add(2, MkPose(kCellA, 2, 2), true);
    Appearance odd = App();
    odd.race = 99;
    r.pm.AddPlayer(3, odd);
    r.Pose(3, MkPose(kCellA, 3, 3));
    r.Run(2000);
    CHECK(r.F(1) && r.F(1)->base == 0x1000);
    CHECK(r.F(2) && r.F(2)->base == 0x1001);
    CHECK(r.F(3) && r.F(3)->base == 0x1002);   // fallback template
    CHECK(r.F(1)->appearance.name == "Courier");
}

static void t_no_pose_no_spawn() {
    g_test = "no_pose_no_spawn";
    Rig r;
    r.pm.AddPlayer(1, App());
    r.Run(1000);
    CHECK(r.eng.spawnCalls == 0);
    r.Pose(1, MkPose(kCellA, 5, 5));
    r.Run(1000);
    CHECK(r.eng.spawnCalls == 1);
}

// Trap: heading. Wrap to (-pi, pi], never send pitch, no SetAngle storm at the +-pi seam.
static void t_yaw_handling() {
    g_test = "yaw_handling";
    Rig r;
    r.Add(1, MkPose(kCellA, 100, 200, 0, 7.0f));
    r.Run(1000);
    CHECK(!r.eng.yawLog.empty());
    for (float y : r.eng.yawLog) CHECK(y > -kPi - 1e-4f && y <= kPi + 1e-4f);
    CHECK(!r.eng.yawLog.empty() && std::fabs(r.eng.yawLog.back() - (7.0f - kTwoPi)) < 1e-3f);

    Rig q;
    q.Add(1, MkPose(kCellA, 100, 200, 0, kPi - 0.001f));
    q.Run(500);
    unsigned base = q.eng.setYawCalls;
    for (int i = 0; i < 20; ++i) {
        q.Pose(1, MkPose(kCellA, 100, 200, 0, (i % 2) ? kPi - 0.001f : -kPi + 0.001f));   // same direction, across the seam
        q.Step();
    }
    CHECK(q.eng.setYawCalls - base <= 1);

    // pitch tilts a body: it is never sent as an angle
    Rig p;
    PuppetPose pitched = MkPose(kCellA, 100, 200, 0, 0.3f);
    pitched.rot.x = 0.9f;
    pitched.rot.z = 0.4f;
    p.Add(1, pitched);
    p.Run(1000);
    CHECK(!p.eng.yawLog.empty());
    for (float y : p.eng.yawLog) CHECK(std::fabs(y - 0.3f) < 1e-4f);
}

static void t_position_rate() {
    g_test = "position_rate";
    Rig r;
    r.Add(1, MkPose(kCellA, 100, 200));
    r.Run(3000);
    // unchanged pose: heartbeat only (about once a second), not once per frame
    unsigned c0 = r.eng.setPosCalls;
    r.Run(3000);
    CHECK(r.eng.setPosCalls - c0 >= 2 && r.eng.setPosCalls - c0 <= 4);
    // sub-epsilon wobble is ignored
    unsigned c1 = r.eng.setPosCalls;
    for (int i = 0; i < 20; ++i) {
        r.Pose(1, MkPose(kCellA, 100 + (i % 2) * 0.2f, 200));
        r.Step();
    }
    CHECK(r.eng.setPosCalls - c1 <= 1);
    // real movement is applied every frame
    unsigned c2 = r.eng.setPosCalls;
    for (int i = 0; i < 10; ++i) {
        r.Pose(1, MkPose(kCellA, 100 + 10.0f * (i + 1), 200));
        r.Step();
    }
    CHECK(r.eng.setPosCalls - c2 == 10);
    CHECK(std::fabs(r.F(1)->pos.x - 200) < 0.01f);
}

// A snap (teleport) is applied at once, even if tiny, and the animation is re-sent.
static void t_snap_and_drift() {
    g_test = "snap_and_drift";
    Rig r;
    r.eng.reportsCurrentGroup = false;
    r.Add(1, MkPose(kCellA, 100, 200, 0, 0, kLocoWalkFwd, Vec3{0, 120, 0}));
    r.Run(1500);
    PuppetPose s = MkPose(kCellA, 100.1f, 200, 0, 0, kLocoWalkFwd, Vec3{0, 120, 0});
    s.snapped = true;
    unsigned c = r.eng.setPosCalls;
    r.Pose(1, s);
    r.Step();
    CHECK(r.eng.setPosCalls == c + 1);
    CHECK(r.pm.stats().teleports >= 1);
    // physics (or the AI) pushed the puppet 2+ m away: noticed and corrected, counted
    r.Pose(1, MkPose(kCellA, 100, 200, 0, 0, kLocoWalkFwd, Vec3{0, 120, 0}));
    r.Run(200);
    r.F(1)->pos.x += 200;
    uint64_t d0 = r.pm.stats().drift;
    r.Step();
    CHECK(r.pm.stats().drift == d0 + 1);
    CHECK(std::fabs(r.F(1)->pos.x - 100) < 0.01f);
    CellId ac = 0;
    Vec3 ap;
    CHECK(r.pm.ActualPose(1, ac, ap) && ac == kCellA && std::fabs(ap.x - 100) < 0.01f);
}

// A body in the way (the real player standing in the puppet) pushes it hundreds of units. The place reported to the
// interpolation buffer must stay the one the driver put it at, or the buffer eases the puppet back from the pushed place.
// A small deviation is still reported as it is (the buffer corrects those at once).
static void t_physics_push_not_adopted() {
    g_test = "physics_push_not_adopted";
    Rig r;
    r.eng.reportsCurrentGroup = false;
    r.Add(1, MkPose(kCellA, 100, 200, 0, 0, kLocoWalkFwd, Vec3{0, 120, 0}));
    r.Run(1500);
    CellId ac = 0;
    Vec3 ap;
    r.F(1)->pos.x += 60;
    CHECK(r.pm.ActualPose(1, ac, ap) && std::fabs(ap.x - 160) < 0.01f);
    CHECK(r.pm.stats().pushesIgnored == 0);
    r.Step();
    r.F(1)->pos.x += 300;
    CHECK(r.pm.ActualPose(1, ac, ap) && std::fabs(ap.x - 100) < 0.01f && ac == kCellA);
    CHECK(r.pm.stats().pushesIgnored == 1);
    uint64_t d0 = r.pm.stats().drift;
    r.Step();
    CHECK(r.pm.stats().drift == d0 + 1);
    CHECK(std::fabs(r.F(1)->pos.x - 100) < 0.01f);
    CHECK(r.pm.StatusJson().find("\"pushesIgnored\":1") != std::string::npos);
}

// Two bodies in one place: the remote player's puppet is driven by SetPos every frame, and a real player standing in (or walking through) that
// place is shoved by the engine, hundreds of units in a second, because the driver puts the puppet back into the same spot again and again.
// The driver must leave room: the place it sends is moved out to bodyRoomUnits around the local player, and the true place is used again as
// soon as the two bodies are apart. The fake engine shoves the real player by the overlap on every write that lands inside it.
static float Flat(const Vec3& a, const Vec3& b) { return std::hypot(a.x - b.x, a.y - b.y); }

static void t_body_room() {
    g_test = "body_room";
    // (a) the remote runs straight through the local player
    {
        Rig r;
        r.eng.haveLocal = true;
        r.eng.localCell = kCellA;
        r.eng.localPos = Vec3{500, 500, 0};
        r.Add(1, MkPose(kCellA, 500, 100, 0, 0, kLocoRunFwd, Vec3{0, 300, 0}));
        r.Run(1500);
        CHECK(r.pm.PhaseOf(1) == Phase::Driving);
        float minD = 1e9f;
        for (int i = 0; i <= 60; ++i) {
            r.Pose(1, MkPose(kCellA, 500, 100.0f + 15.0f * i, 0, 0, kLocoRunFwd, Vec3{0, 300, 0}));
            r.Step();
            minD = std::min(minD, Flat(r.F(1)->pos, r.eng.localPos));
        }
        r.Run(500);
        CHECK(r.eng.localShoves == 0 && r.eng.localTravel == 0);
        CHECK(std::fabs(r.eng.localPos.x - 500) < 0.01f && std::fabs(r.eng.localPos.y - 500) < 0.01f);
        CHECK(minD >= 69.9f);
        CHECK(r.pm.stats().bodyYields > 0);
        CHECK(Flat(r.F(1)->pos, Vec3{500, 1000, 0}) < 0.6f);           // apart again: the true place is used
        CHECK(r.pm.StatusJson().find("\"bodyYields\":") != std::string::npos);
    }
    // (b) the remote stands exactly where the local player stands (also at spawn): it waits at the edge, the player is not moved
    {
        Rig r;
        r.eng.haveLocal = true;
        r.eng.localCell = kCellA;
        r.eng.localPos = Vec3{500, 500, 0};
        r.Add(1, MkPose(kCellA, 500, 500, 0));
        r.Run(2000);
        CHECK(r.eng.localShoves == 0);
        CHECK(std::fabs(Flat(r.F(1)->pos, r.eng.localPos) - 70.0f) < 0.6f);
    }
    // (c) the local player walks into a standing remote and through its place: it is the puppet that yields, the player is not shoved
    {
        Rig r;
        r.eng.haveLocal = true;
        r.eng.localCell = kCellA;
        r.eng.localPos = Vec3{500, 100, 0};
        r.Add(1, MkPose(kCellA, 500, 300, 0));
        r.Run(1500);
        float minD = 1e9f;
        for (int i = 0; i <= 40; ++i) {
            r.eng.localPos.y = 100.0f + 15.0f * i;
            r.Step();
            minD = std::min(minD, Flat(r.F(1)->pos, r.eng.localPos));
        }
        CHECK(r.eng.localShoves == 0);
        CHECK(minD >= 69.9f);
        r.eng.localPos = Vec3{500, 900, 0};
        r.Run(500);
        CHECK(Flat(r.F(1)->pos, Vec3{500, 300, 0}) < 0.6f);            // the player is past: the puppet is back at the true place
    }
    // (d) no room is made where there is no body: another cell, another floor, a local place the engine cannot read
    {
        Rig r;
        r.eng.haveLocal = true;
        r.eng.localCell = kCellB;
        r.eng.localPos = Vec3{500, 500, 0};
        r.Add(1, MkPose(kCellA, 500, 500, 0));
        r.Run(1500);
        CHECK(Flat(r.F(1)->pos, Vec3{500, 500, 0}) < 0.6f && r.pm.stats().bodyYields == 0);
        Rig r2;
        r2.eng.haveLocal = true;
        r2.eng.localCell = kCellA;
        r2.eng.localPos = Vec3{500, 500, 0};
        r2.Add(1, MkPose(kCellA, 500, 500, 300));
        r2.Run(1500);
        CHECK(Flat(r2.F(1)->pos, Vec3{500, 500, 0}) < 0.6f && r2.pm.stats().bodyYields == 0);
        Rig r3;
        r3.Add(1, MkPose(kCellA, 500, 500, 0));
        r3.Run(1500);
        CHECK(Flat(r3.F(1)->pos, Vec3{500, 500, 0}) < 0.6f && r3.pm.stats().bodyYields == 0);
    }
}

static void t_group_refused_retries() {
    g_test = "group_refused_retries";
    Rig r;
    r.eng.playGroupRefusalsLeft = 3;
    r.Add(1, MkPose(kCellA, 100, 200, 0, 0, kLocoWalkFwd, Vec3{0, 120, 0}));
    r.Run(1500);
    CHECK(r.pm.stats().groupRefused == 3);
    CHECK(r.Group(1) == kGrpForward);
}

// Engine call budget: no Tick may issue more than maxEngineCalls writes, and nobody starves.
static void t_call_budget() {
    g_test = "call_budget";
    Rig r([](Config& c) { c.maxEngineCalls = 4; });
    for (Slot s = 1; s <= 3; ++s) {
        r.Add(s, MkPose(kCellA, 100.f * s, 200, 0, 0.1f * s, kLocoRunFwd, Vec3{0, 300, 0}));
        r.pm.SetEquipment(s, {{0x510u + s, false}});
    }
    unsigned worst = 0;
    for (int i = 0; i < 160; ++i) {
        unsigned w = r.eng.Writes();
        r.Step();
        worst = std::max(worst, r.eng.Writes() - w);
    }
    CHECK(worst <= 4);
    CHECK(r.pm.stats().budgetDeferred > 0);
    for (Slot s = 1; s <= 3; ++s) {
        CHECK(r.pm.PhaseOf(s) == Phase::Driving);
        CHECK(r.pm.EquipConverged(s));
        CHECK(r.Group(s) == kGrpFastForward);
    }
}

// The ghost-ref map used by the acting-player design.
static void t_ref_map() {
    g_test = "ref_map";
    Rig r;
    CHECK(r.pm.RefOf(1) == 0);
    r.Add(1, MkPose(kCellA, 100, 200));
    r.Add(2, MkPose(kCellA, 300, 200));
    r.Run(1500);
    RefId a = r.pm.RefOf(1), b = r.pm.RefOf(2);
    CHECK(a && b && a != b);
    CHECK(r.pm.SlotOf(a).value_or(99) == 1 && r.pm.SlotOf(b).value_or(99) == 2);
    CHECK(!r.pm.SlotOf(0).has_value() && !r.pm.SlotOf(0xFFFFu).has_value());
    // after a despawn (save), the map has no entry for a ref that no longer exists
    r.pm.OnSaveBegin();
    CHECK(r.pm.RefOf(1) == 0 && r.pm.RefOf(2) == 0);
    CHECK(!r.pm.SlotOf(a).has_value() && !r.pm.SlotOf(b).has_value());
    r.pm.OnSaveEnd();
    r.Run(1500);
    CHECK(r.pm.RefOf(1) != 0);
    std::string js = r.pm.StatusJson();
    CHECK(js.find("\"slot\":1") != std::string::npos && js.find("\"phase\":\"Driving\"") != std::string::npos);
    // the keys tools/avatar/proof_puppet.py bounds (STATS_SPEC) must all be reported
    for (const char* k : {"spawnFailures", "refLost", "restrainReasserts", "groupRefused", "drift", "equipGaveUp", "load3DTimeouts"})
        CHECK(js.find(std::string("\"") + k + "\":") != std::string::npos);
}

// ---- script backend: exact text ---------------------------------------------------------------------------------------

struct FakeRunner : IScriptRunner {
    struct Line { RefId ctx; std::string text; };
    std::vector<Line> lines;
    bool ok = true;
    double queryValue = 0;
    RefId placed = 0xFF001234;
    bool Exec(RefId ctx, const std::string& line) override { lines.push_back({ctx, line}); return ok; }
    bool Query(RefId ctx, const std::string& line, double& out) override { lines.push_back({ctx, "?" + line}); out = queryValue; return ok; }
    RefId PlaceAtMe(FormId base, CellId cell, const Vec3& pos, float yawDeg) override {
        char b[160];
        std::snprintf(b, sizeof b, "PlaceAtMe %08X cell=%X %.1f,%.1f,%.1f yaw=%.2f", (unsigned)base, (unsigned)cell, pos.x, pos.y, pos.z, yawDeg);
        lines.push_back({0, b});
        return placed;
    }
    std::string Joined() const {
        std::string s;
        for (auto& l : lines) s += (s.empty() ? "" : " | ") + l.text;
        return s;
    }
};
struct FakeReads : IEngineReads {
    RefId anchor = 0x00ABCDEF;
    bool WorldReady() override { return true; }
    uint32_t WorldEpoch() override { return 1; }
    bool MenuOpen() override { return false; }
    bool CellLoaded(CellId) override { return true; }
    bool SameSpace(CellId, CellId) override { return false; }
    bool IsValid(RefId) override { return true; }
    bool Has3D(RefId) override { return true; }
    uint32_t Generation3D(RefId) override { return 1; }
    std::vector<RefId> FindTaggedPuppets() override { return {}; }
    bool GetPose(RefId, CellId&, Vec3&) override { return false; }
    int CurrentGroup(RefId) override { return -1; }
    bool GetEquipped(RefId, std::vector<FormId>&) override { return false; }
    int ItemCount(RefId, FormId) override { return 0; }
    RefId AnchorRef(CellId, Vec3& p) override { p = Vec3{}; return anchor; }
};

static void t_script_backend() {
    g_test = "script_backend";
    FakeRunner run;
    FakeReads rd;
    ScriptPuppetEngine e(run, rd);
    const RefId R = 0xFF000A01;
    CHECK(e.SetPos(R, Vec3{12.5f, -3.0004f, 100}));
    CHECK(run.Joined() == "SetPos X 12.500 | SetPos Y -3.000 | SetPos Z 100.000");
    CHECK(run.lines[0].ctx == R);
    run.lines.clear();
    CHECK(e.SetYaw(R, kPi * 0.5f));
    CHECK(run.Joined() == "SetAngle Z 90.000");
    run.lines.clear();
    CHECK(e.SetYaw(R, 7.0f));
    CHECK(run.lines.size() == 1 && run.lines[0].text.rfind("SetAngle Z ", 0) == 0);
    double deg = std::atof(run.lines[0].text.c_str() + 11);
    CHECK(deg > -180.0 && deg <= 180.0 && std::fabs(deg - (7.0 - 6.283185307) * 57.29578) < 0.01);
    run.lines.clear();
    CHECK(e.PlayGroup(R, kGrpFastForward, 1) && e.PlayGroup(R, kGrpIdle, 0));
    CHECK(run.Joined() == "PlayGroup FastForward 1 | PlayGroup Idle 0");
    run.lines.clear();
    CHECK(!e.PlayGroup(R, 9999, 1));        // never guess a group name
    CHECK(run.lines.empty());
    CHECK(e.SetRestrained(R, true) && e.SetRestrained(R, false) && e.SetSneak(R, true) && e.SetAlert(R, false));
    CHECK(run.Joined() == "SetRestrained 1 | SetRestrained 0 | SetForceSneak 1 | SetAlert 0");
    run.lines.clear();
    CHECK(e.MakeNonCombatant(R));
    CHECK(run.Joined() == "SetGhost 1 | SetIgnoreFriendlyHits 1");
    run.lines.clear();
    CHECK(e.AddItem(R, 0x000A1B2C, 1) && e.RemoveItem(R, 0x000A1B2C, 2) && e.EquipItem(R, 0x00001234) && e.UnequipItem(R, 0x00001234));
    CHECK(run.Joined() == "AddItem 000A1B2C 1 1 | RemoveItem 000A1B2C 2 1 | EquipItem 00001234 1 1 | UnequipItem 00001234 0 1");
    run.lines.clear();
    CHECK(!e.AddItem(R, 5, 0) && run.lines.empty());
    e.Delete(R);
    CHECK(run.Joined() == "Disable | MarkForDelete");
    run.lines.clear();
    CHECK(e.MoveToCell(R, 0x20, Vec3{1, 2, 3}, 0));
    CHECK(run.Joined() == "MoveToMarker 00ABCDEF 0 0 0");
    run.lines.clear();
    rd.anchor = 0;
    CHECK(!e.MoveToCell(R, 0x20, Vec3{}, 0) && run.lines.empty());
    RefId sp = e.Spawn(0x1000, 0x10, Vec3{1, 2, 3}, kPi);
    CHECK(sp == 0xFF001234 && run.Joined() == "PlaceAtMe 00001000 cell=10 1.0,2.0,3.0 yaw=180.00");
    run.lines.clear();
    run.queryValue = 1;
    CHECK(e.IsRestrained(R) && e.IsWeaponDrawn(R));
    run.queryValue = 0;
    CHECK(!e.IsRestrained(R));
    CHECK(run.lines[0].text == "?GetRestrained" && run.lines[1].text == "?IsWeaponOut");
    run.ok = false;
    CHECK(!e.SetPos(R, Vec3{}) && !e.PlayGroup(R, 0, 1) && !e.IsRestrained(R));
    CHECK(!e.SetWeaponDrawn(R, true));   // unsupported by design
    CHECK(FormatNumber(-0.0001) == "0.000" && FormatNumber(1e12) == "1000000000000.000" && FormatNumber(std::nan("")) == "0.000");
    CHECK(FormatForm(0x1A) == "0000001A");
}

// The manager works unchanged on top of the script backend (smoke: it produces only whitelisted command text).
static void t_manager_on_script_backend() {
    g_test = "manager_on_script_backend";
    struct Runner : FakeRunner {};
    struct Reads : FakeReads {
        int tick = 0;
        bool Has3D(RefId) override { return ++tick > 2; }
    };
    Runner run;
    Reads rd;
    ScriptPuppetEngine e(run, rd);
    Config cfg = MakeCfg();
    PuppetManager pm(e, cfg);
    pm.AddPlayer(1, App());
    PuppetPose p = MkPose(kCellA, 100, 200, 0, 0.2f, kLocoRunFwd, Vec3{0, 300, 0});
    for (uint32_t t = 1000; t < 3000; t += 50) {
        pm.SetPose(1, p);
        pm.Tick(t);
    }
    CHECK(pm.PhaseOf(1) == Phase::Driving);
    static const char* ok[] = {"SetPos ", "SetAngle ", "PlayGroup ", "SetRestrained ", "SetGhost ", "SetIgnoreFriendlyHits ", "SetForceSneak ",
                               "PlaceAtMe ", "?GetRestrained"};
    for (auto& l : run.lines) {
        bool good = false;
        for (const char* o : ok)
            if (l.text.rfind(o, 0) == 0) good = true;
        CHECK(good);
    }
    CHECK(run.Joined().find("PlayGroup FastForward 1") != std::string::npos);
}

// ---- live layer bridge -----------------------------------------------------------------------------------------------

static void t_live_bridge() {
    g_test = "live_bridge";
    namespace live = ::fnvcoop::live;
    live::InterpOut o;
    o.state = live::InterpState::Interpolating;
    o.cell = 0x77;
    o.pos = {1, 2, 3};
    o.rot = {0.1f, 0.2f, 0.3f};
    o.vel = {4, 5, 6};
    o.anim.locomotion = kLocoRunLeft;
    o.anim.sneak = true;
    o.anim.weaponDrawn = true;
    o.anim.jump = true;
    o.anim.reload = true;
    o.anim.aim = true;
    PuppetPose p = FromInterp(o);
    CHECK(p.valid && !p.frozen && !p.snapped && p.cell == 0x77);
    CHECK(p.pos.y == 2 && p.rot.y == 0.2f && p.vel.z == 6);
    CHECK(p.anim.loco == kLocoRunLeft && p.anim.sneak && p.anim.weaponDrawn && p.anim.jump && p.anim.reload && p.anim.aim);
    o.state = live::InterpState::Holding;
    CHECK(FromInterp(o).frozen);
    o.state = live::InterpState::Empty;
    CHECK(FromInterp(o).frozen && !FromInterp(o).valid);
    o.state = live::InterpState::Snapped;
    o.snapped = false;
    CHECK(FromInterp(o).snapped);
    o.state = live::InterpState::Interpolating;
    o.snapped = true;
    CHECK(FromInterp(o).snapped);
    // sender side: the locomotion code comes from the velocity / mover flags
    LocalRead rd;
    rd.haveMoveFlags = true;
    rd.moverRun = true;
    Anim a;
    a.weaponDrawn = true;
    live::ActorSnapshot s = MakeLocalSnapshot(7, 3, 11, 5000, 0x55, Vec3{1, 2, 3}, Vec3{0, 1.5f, 0}, Vec3{300, 0, 0}, a, &rd);
    CHECK(s.netId == 7 && s.ownerEpoch == 3 && s.seq == 11 && s.tick == 5000 && s.cell == 0x55);
    CHECK(s.anim.locomotion == kLocoRunFwd);
    CHECK(s.anim.weaponDrawn);
    // round trip of the anim flags
    Anim b;
    b.loco = 3; b.sneak = true; b.reload = true;
    Anim c = FromLive(ToLive(b));
    CHECK(c.loco == 3 && c.sneak && c.reload && !c.jump && !c.aim && !c.weaponDrawn);
    // the one-shot: group, counter and block bit survive both ways, and reach the puppet through the interpolation output
    b.action = 92;
    b.actionSeq = 200;
    b.block = true;
    Anim d = FromLive(ToLive(b));
    CHECK(d.action == 92 && d.actionSeq == 200 && d.block);
    live::ActorSnapshot es = MakeLocalSnapshot(7, 3, 11, 5000, 0x55, Vec3{}, Vec3{}, Vec3{}, b);
    CHECK(es.anim.action == 92 && es.anim.actionSeq == 200 && es.anim.block);
    live::InterpOut eo;
    eo.state = live::InterpState::Interpolating;
    eo.anim = es.anim;
    CHECK(FromInterp(eo).anim.action == 92 && FromInterp(eo).anim.actionSeq == 200 && FromInterp(eo).anim.block);
}

// ---- the owner's one-shots: attacks, throws, block, stagger on the puppet --------------------------------------------------
// The sync model (docs/sync-model.md section 2): a player's attacks are owner events played on the copy. The stream carries the group in
// AnimState::action (held for a while) and a counter in actionSeq that bumps when a one-shot starts; the puppet plays one bump once.

static int CountGroup(const Rig& r, int g) {
    int n = 0;
    for (auto& e : r.eng.groupLog)
        if (e.second == g) ++n;
    return n;
}

// The owner's pose with a one-shot in the stream (`group` 0 = none): the group and the counter.
static PuppetPose Act(PuppetPose p, int group, uint8_t seq, bool block = false) {
    p.anim.action = static_cast<uint8_t>(group);
    p.anim.actionSeq = seq;
    p.anim.block = block;
    return p;
}

// A rig whose puppet is driving and has read the owner's counter (`baseline`).
static void StartDriving(Rig& r, uint8_t baseline = 4) {
    r.Add(1, Act(MkPose(kCellA, 100, 200), 0, baseline));
    CHECK(r.UntilDriving(1));
    r.Run(300);
}

// Melee swing, unarmed punch, power attack, shot, throw, hit reactions: each shows once on the puppet, the group is named from the NPC layer's table,
// and the movement group comes back afterwards.
static void t_oneshot_plays_once() {
    g_test = "oneshot_plays_once";
    const int groups[] = {26 /*AttackLeft: the unarmed punch*/, 32 /*AttackRight: a swing*/, 38 /*Attack3*/, 92 /*AttackPower*/, 114 /*AttackThrow*/,
                          172 /*Recoil*/, 171 /*BlockHit*/, 223 /*Stagger*/};
    for (int g : groups) {
        Rig r;
        StartDriving(r);
        CHECK(CountGroup(r, g) == 0);
        r.Pose(1, Act(MkPose(kCellA, 100, 200), g, 5));
        r.Run(300);
        CHECK(r.Group(1) == g);                // the engine is playing it
        r.Run(500);                            // the owner keeps it in the stream for 600 ms: many Ticks see the same group
        CHECK(CountGroup(r, g) == 1);
        r.Pose(1, Act(MkPose(kCellA, 100, 200), 0, 5));
        r.Run(1500);
        CHECK(CountGroup(r, g) == 1);
        CHECK(r.Group(1) == kGrpIdle && r.eng.groupLog.back().second == kGrpIdle);   // asserted again once it ran its time
        CHECK(r.pm.stats().oneShots == 1);
    }
    // the same group twice in a row is two events
    Rig r;
    StartDriving(r);
    r.Pose(1, Act(MkPose(kCellA, 100, 200), 32, 5));
    r.Run(900);
    r.Pose(1, Act(MkPose(kCellA, 100, 200), 32, 6));
    r.Run(900);
    CHECK(CountGroup(r, 32) == 2);
    // a one-shot is started now (flag 1): the queued mode (0) never lets go of the puppet's idle slot
    bool immediate = false, queued = false;
    for (size_t i = 0; i < r.eng.groupLog.size(); ++i)
        if (r.eng.groupLog[i].second == 32) ((r.eng.groupFlags[i] & 1) ? immediate : queued) = true;
    CHECK(immediate && !queued);
}

// A duplicate of the snapshot, or an older one that arrives late, plays nothing.
static void t_oneshot_duplicates_and_reordering() {
    g_test = "oneshot_duplicates_and_reordering";
    Rig r;
    StartDriving(r);
    r.Pose(1, Act(MkPose(kCellA, 100, 200), 32, 5));
    r.Run(300);
    CHECK(CountGroup(r, 32) == 1);
    r.Run(600);                                                          // the same snapshot on every Tick
    CHECK(CountGroup(r, 32) == 1);
    r.Pose(1, Act(MkPose(kCellA, 100, 200), 0, 4));                      // an older snapshot overtaken on the wire ...
    r.Step();
    r.Pose(1, Act(MkPose(kCellA, 100, 200), 32, 5));                     // ... and the newest again
    r.Run(500);
    CHECK(CountGroup(r, 32) == 1);
    CHECK(r.pm.stats().oneShotsStale >= 1);
    // after the stale blip a real next attack still plays
    r.Pose(1, Act(MkPose(kCellA, 100, 200), 38, 6));
    r.Run(300);
    CHECK(CountGroup(r, 38) == 1);
    // an owner whose counter restarted below ours (the game was restarted): ignored for a second, then taken as the new baseline
    Rig q;
    StartDriving(q, 100);
    q.Pose(1, Act(MkPose(kCellA, 100, 200), 0, 1));
    q.Run(1500);
    q.Pose(1, Act(MkPose(kCellA, 100, 200), 32, 2));
    q.Run(300);
    CHECK(CountGroup(q, 32) == 1);
}

// A lost datagram: the counter jumps by more than one. The group the stream holds now plays, once; the one in between is gone with its datagram.
static void t_oneshot_lost_packet() {
    g_test = "oneshot_lost_packet";
    Rig r;
    StartDriving(r);
    r.Pose(1, Act(MkPose(kCellA, 100, 200), 38, 7));   // seq 5 and 6 never arrived
    r.Run(900);
    CHECK(CountGroup(r, 38) == 1);
    CHECK(CountGroup(r, 32) == 0);
    // the counter wraps at 256: 255 -> 0 is newer
    Rig w;
    StartDriving(w, 254);
    int plays = 0;
    for (uint8_t seq : {uint8_t(255), uint8_t(0), uint8_t(1)}) {
        w.Pose(1, Act(MkPose(kCellA, 100, 200), 32, seq));
        w.Run(700);
        plays = CountGroup(w, 32);
    }
    CHECK(plays == 3);
}

// Nothing plays while the puppet is not driving, and an event from before it was driving again is never replayed late.
static void t_oneshot_only_while_driving() {
    g_test = "oneshot_only_while_driving";
    // a counter and an action already in the first snapshot: an old event, only read
    {
        Rig r;
        r.Add(1, Act(MkPose(kCellA, 100, 200), 32, 7));
        CHECK(r.UntilDriving(1));
        r.Run(800);
        CHECK(CountGroup(r, 32) == 0);
        r.Pose(1, Act(MkPose(kCellA, 100, 200), 32, 8));
        r.Run(300);
        CHECK(CountGroup(r, 32) == 1);
    }
    // the puppet is waiting for its 3D after a cell move: the attack that happens meanwhile is over for it
    {
        Rig r;
        r.eng.exterior.clear();
        StartDriving(r, 3);
        r.eng.load3DFrames = 12;
        r.Pose(1, Act(MkPose(kCellB, 100, 200), 32, 4));      // another interior: MoveToCell, 3D rebuilt
        r.Step();
        CHECK(r.pm.PhaseOf(1) != Phase::Driving);
        CHECK(r.UntilDriving(1, 200));
        r.Run(300);                                           // the action is still in the stream, the counter is not new to it
        CHECK(CountGroup(r, 32) == 0);
        r.Pose(1, Act(MkPose(kCellB, 100, 200), 32, 5));
        r.Run(300);
        CHECK(CountGroup(r, 32) == 1);
    }
    // parked: the remote's cell is not loaded here, the puppet is deleted; the attack meanwhile is not played when it comes back
    {
        Rig r;
        StartDriving(r, 3);
        r.eng.loadedCells.erase(kCellA);
        r.Run(1500);
        CHECK(r.pm.RefOf(1) == 0);
        r.Pose(1, Act(MkPose(kCellA, 100, 200), 32, 4));
        r.Run(300);
        r.eng.loadedCells.insert(kCellA);
        CHECK(r.UntilDriving(1, 200));
        r.Run(300);
        CHECK(CountGroup(r, 32) == 0);
    }
    // the stream ran dry (frozen): the last thing the owner said is not what he is doing now
    {
        Rig r;
        StartDriving(r);
        PuppetPose f = Act(MkPose(kCellA, 100, 200), 32, 5);
        f.frozen = true;
        r.Pose(1, f);
        r.Run(600);
        CHECK(CountGroup(r, 32) == 0);
        f.frozen = false;
        r.Pose(1, f);
        r.Run(600);
        CHECK(CountGroup(r, 32) == 0);                        // consumed, not deferred
        CHECK(r.pm.stats().oneShotsFrozen == 1);
    }
}

// Block is held: played with the hold flag once, kept until the owner stops reporting it, then the movement group again.
static void t_oneshot_held_block() {
    g_test = "oneshot_held_block";
    Rig r;
    StartDriving(r);
    r.Pose(1, Act(MkPose(kCellA, 100, 200), 170, 5, true));
    r.Run(2500);                                           // held far longer than any swing
    CHECK(CountGroup(r, 170) == 1);
    CHECK(r.Group(1) == 170 && r.eng.groupLog.back().second == 170);   // nothing cut it off
    bool held = false;
    for (size_t i = 0; i < r.eng.groupLog.size(); ++i)
        if (r.eng.groupLog[i].second == 170 && (r.eng.groupFlags[i] & 1)) held = true;
    CHECK(held);
    r.Pose(1, Act(MkPose(kCellA, 100, 200), 0, 5, false));
    r.Run(300);
    CHECK(r.Group(1) == kGrpIdle);
    // a dry stream ends the block as well
    Rig q;
    StartDriving(q);
    q.Pose(1, Act(MkPose(kCellA, 100, 200), 170, 5, true));
    q.Run(600);
    PuppetPose f = Act(MkPose(kCellA, 100, 200), 170, 5, true);
    f.frozen = true;
    q.Pose(1, f);
    q.Run(300);
    CHECK(q.Group(1) == kGrpIdle);
}

// The readback re-assert (the engine reports another group than the one we last issued for 300 ms) must not cut a swing off.
static void t_oneshot_not_cut_by_readback() {
    g_test = "oneshot_not_cut_by_readback";
    Rig r;
    StartDriving(r);
    r.Pose(1, Act(MkPose(kCellA, 100, 200), 92, 5));
    r.Run(500);                                            // a power attack takes 600 ms
    CHECK(r.Group(1) == 92);
    CHECK(CountGroup(r, kGrpIdle) == 1);                   // only the one from before it started
    r.Run(500);
    CHECK(r.Group(1) == kGrpIdle);
}

// Reload plays the owner's own reload group (it depends on the weapon), once; without one the default.
static void t_reload_group_from_owner() {
    g_test = "reload_group_from_owner";
    Rig r;
    StartDriving(r);
    PuppetPose p = Act(MkPose(kCellA, 100, 200), 178, 5);   // ReloadB
    p.anim.reload = true;
    r.Pose(1, p);
    r.Run(1000);
    CHECK(CountGroup(r, 178) == 1);
    CHECK(CountGroup(r, kGrpReloadA) == 0);
    // a reload flag without a reload group in the stream: the default
    Rig q;
    StartDriving(q);
    PuppetPose d = MkPose(kCellA, 100, 200);
    d.anim.reload = true;
    q.Pose(1, d);
    q.Run(600);
    CHECK(CountGroup(q, kGrpReloadA) == 1);
    // the attack that was last in the stream is no reload group
    Rig w;
    StartDriving(w);
    PuppetPose a = Act(MkPose(kCellA, 100, 200), 32, 5);
    a.anim.reload = true;
    w.Pose(1, a);
    w.Run(600);
    CHECK(CountGroup(w, kGrpReloadA) == 1 && CountGroup(w, 32) == 1);
}


// "I don't see the other person punching": an attack on a puppet whose weapon is away (the owner's fists show no weapon out) must still show. The puppet draws first,
// the swing plays after the draw, the weapon goes away again when it is over; the log says what happened (remote attack received / puppet animation started).
static void t_oneshot_unarmed_punch_draws_first() {
    g_test = "oneshot_unarmed_punch_draws_first";
    Rig r;
    r.eng.attackNeedsDrawn = true;                    // the engine plays no attack on a sheathed actor
    StartDriving(r);
    CHECK(!r.F(1)->drawn);
    r.Pose(1, Act(MkPose(kCellA, 100, 200), 26 /*AttackLeft: a punch*/, 5));
    r.Run(500);
    CHECK(r.Group(1) == 26);                          // the punch is on screen
    CHECK(r.pm.stats().attackDraws == 1 && r.pm.stats().oneShots == 1 && r.eng.silentPlays == 0);
    CHECK(r.Logged("remote attack received from slot 1: group 26 (AttackLeft)") && r.Logged("puppet animation started"));
    CHECK(r.pm.stats().oneShotsSeen == 1 && r.pm.stats().oneShotsNotSeen == 0);
    r.Pose(1, Act(MkPose(kCellA, 100, 200), 0, 5));
    r.Run(3000);
    CHECK(!r.F(1)->drawn);                            // the fists go down again once the swing is over
    // a melee swing and a power attack the same way
    for (int g : {32, 92}) {
        Rig q;
        q.eng.attackNeedsDrawn = true;
        StartDriving(q);
        q.Pose(1, Act(MkPose(kCellA, 100, 200), g, 5));
        q.Run(500);
        CHECK(q.Group(1) == g && q.pm.stats().oneShotsSeen == 1);
    }
    // an owner whose weapon IS out needs no extra draw
    Rig w;
    w.eng.attackNeedsDrawn = true;
    w.Add(1, [] { PuppetPose p = MkPose(kCellA, 100, 200); p.anim.weaponDrawn = true; return Act(p, 0, 4); }());
    CHECK(w.UntilDriving(1));
    w.Run(1500);
    PuppetPose armed = MkPose(kCellA, 100, 200);
    armed.anim.weaponDrawn = true;
    w.Pose(1, Act(armed, 32, 5));
    w.Run(500);
    CHECK(w.Group(1) == 32 && w.pm.stats().attackDraws == 0);
}

// A PlayGroup the engine accepts and then does not show is found and logged ("the swing is NOT visible"), not silently counted as played.
static void t_oneshot_unseen_swing_is_reported() {
    g_test = "oneshot_unseen_swing_is_reported";
    Rig r;
    r.eng.playGroupSilent = true;
    StartDriving(r);
    r.Pose(1, Act(MkPose(kCellA, 100, 200), 32, 5));
    r.Run(600);
    CHECK(r.eng.silentPlays >= 1);
    CHECK(r.pm.stats().oneShotsNotSeen >= 1 && r.pm.stats().oneShotsSeen == 0);
    CHECK(r.pm.stats().attackRetries <= 2);            // played again at most twice, not for ever
    CHECK(r.Logged("NOT visible") && r.Logged("weapon out in the engine"));
    Rig q;                                            // a swing that does show is confirmed
    StartDriving(q);
    q.Pose(1, Act(MkPose(kCellA, 100, 200), 32, 5));
    q.Run(600);
    CHECK(q.pm.stats().oneShotsSeen == 1 && q.pm.stats().oneShotsNotSeen == 0 && !q.Logged("NOT visible"));
}

// A swing the engine did not show is played again (and the weapon drawn again if the engine says it is away): the first draw of a process may not take, and a group
// started in the same frame as the draw is lost. The first version logged "NOT visible" and gave up.
static void t_oneshot_unseen_swing_is_retried() {
    g_test = "oneshot_unseen_swing_is_retried";
    Rig r;
    r.eng.attackNeedsDrawn = true;
    r.eng.drawNeedsTwoCalls = true;                   // the first SetWeaponDrawn(true) is accepted and does not take
    StartDriving(r);
    r.Pose(1, Act(MkPose(kCellA, 100, 200), 32, 5));
    r.Run(700);
    CHECK(r.pm.stats().oneShotsNotSeen == 1);         // the first try was not shown ...
    CHECK(r.pm.stats().attackRetries == 1);           // ... the second one was
    CHECK(r.pm.stats().oneShotsSeen == 1);
    CHECK(r.Logged("NOT visible") && r.Logged("the puppet is playing group 32"));
    // given up after two more tries, not for ever
    Rig q;
    q.eng.playGroupSilent = true;
    StartDriving(q);
    q.Pose(1, Act(MkPose(kCellA, 100, 200), 32, 5));
    q.Run(1500);
    CHECK(q.pm.stats().attackRetries == 2);
}

// An attack plays only after the ENGINE says the weapon is out: the puppet's own belief (the draw call was accepted) is not enough.
static void t_oneshot_waits_for_the_engines_weapon() {
    g_test = "oneshot_waits_for_the_engines_weapon";
    Rig r;
    r.eng.attackNeedsDrawn = true;
    // an owner whose weapon is out: the planner draws the puppet's weapon, the engine reads it back as away (the draw did not take)
    r.eng.drawNeedsTwoCalls = true;
    PuppetPose armed = MkPose(kCellA, 100, 200);
    armed.anim.weaponDrawn = true;
    r.Add(1, Act(armed, 0, 4));
    CHECK(r.UntilDriving(1));
    r.Run(1200);
    CHECK(!r.F(1)->drawn);                            // belief: drawn; engine: away
    r.Pose(1, Act(armed, 32, 5));
    r.Run(700);
    CHECK(r.pm.stats().attackDraws >= 1);             // the attack drew it again before playing the group
    CHECK(r.F(1)->drawn);
    CHECK(r.pm.stats().oneShotsSeen == 1 && r.pm.stats().oneShotsNotSeen == 0);
}

// The look the engine built is read back after a spawn and the result is in the log.
static void t_look_is_checked_after_the_spawn() {
    g_test = "look_is_checked_after_the_spawn";
    Appearance want = App(true);
    want.hair = 0x44;
    want.eyes = 0x45;
    want.faceCoeffs.assign(130, 3);
    {
        Rig r;
        r.eng.looksByBase = true;
        r.eng.lookReadable = true;
        r.pm.AddPlayer(1, want);
        r.Pose(1, MkPose(kCellA, 100, 200));
        CHECK(r.UntilDriving(1));
        r.Run(800);
        CHECK(r.pm.stats().looksChecked == 1 && r.pm.stats().looksWrong == 0);
        CHECK(r.Logged("look check of slot 1: the puppet's base has the look that was asked for"));
    }
    {
        Rig r;
        r.eng.looksByBase = true;
        r.eng.lookReadable = true;
        r.eng.lookReadAsAsked = false;                // the engine built another face and skin tone
        r.eng.lookReadBack = want;
        r.eng.lookReadBack.faceCoeffs.assign(130, 0);
        r.eng.lookReadBack.race = 99;
        r.pm.AddPlayer(1, want);
        r.Pose(1, MkPose(kCellA, 100, 200));
        CHECK(r.UntilDriving(1));
        r.Run(800);
        CHECK(r.pm.stats().looksChecked == 1 && r.pm.stats().looksWrong == 1);
        CHECK(r.Logged("DIFFERS from the look asked for in: race, face shape, face asymmetry, skin tone"));
    }
}

// ---- furniture ---------------------------------------------------------------------------------------------------------------------

static constexpr FormId kChair = 0x5000, kBench = 0x5001;
static PuppetPose Sit(PuppetPose p, uint8_t sit, FormId furniture, bool sleep = false) {
    p.anim.sit = sit;
    p.anim.furniture = furniture;
    p.anim.sleep = sleep;
    return p;
}

// The owner sits: his puppet is told to use the same chair, is then left alone (no movement, no group: either would pull it out of the pose), and stands up
// when the owner does, after which it is driven again.
static void t_seat_follows_the_owner() {
    g_test = "seat_follows_the_owner";
    Rig r;
    r.eng.furniture = {kChair, kBench};
    StartDriving(r);
    const unsigned setPosBefore = r.eng.setPosCalls;
    r.Pose(1, Sit(MkPose(kCellA, 100, 200), 2, kChair));
    r.Run(1000);
    CHECK(r.eng.sitCalls == 1 && r.eng.sitLog.back().second == kChair);
    CHECK(r.F(1)->sit == 4 && r.pm.stats().seatsIssued == 1);
    CHECK(r.Logged("sits in furniture 20480"));
    const unsigned setPosSeated = r.eng.setPosCalls;
    const unsigned groupsSeated = r.eng.playGroupCalls;
    r.Run(4000);                                      // many Ticks, heartbeats and a restraint check later
    CHECK(r.eng.sitCalls == 1);                       // never repeated: the chair would toggle
    CHECK(r.eng.setPosWhileSeated == 0 && r.eng.playGroupWhileSeated == 0);
    CHECK(r.eng.setPosCalls == setPosSeated && r.eng.playGroupCalls == groupsSeated);
    CHECK(setPosSeated >= setPosBefore);
    // the owner gets up
    r.Pose(1, MkPose(kCellA, 100, 200));
    r.Run(1500);
    CHECK(r.eng.standCalls >= 1 && r.eng.standAttempts.front() == 0);
    CHECK(r.F(1)->sit == 0);
    CHECK(r.Logged("is out of the furniture"));
    r.Pose(1, MkPose(kCellA, 300, 200));
    r.Run(1000);
    CHECK(r.F(1)->pos.x > 250);                       // driven again
    CHECK(r.eng.setPosWhileSeated == 0);
}

// A sit the engine accepts and then drops is noticed (the sit state reads 0) and tried again after a pause; a refused one is retried at a calm pace and logged once.
static void t_seat_dropped_and_refused() {
    g_test = "seat_dropped_and_refused";
    Rig r;
    r.eng.furniture = {kChair};
    r.eng.sitDropped = true;
    StartDriving(r);
    r.Pose(1, Sit(MkPose(kCellA, 100, 200), 2, kChair));
    r.Run(12000);
    CHECK(r.pm.stats().seatDrops >= 2);
    CHECK(r.eng.sitCalls >= 3 && r.eng.sitCalls <= 6);   // not every Tick
    Rig q;
    q.eng.furniture = {kChair};
    q.eng.sitAccepted = false;
    StartDriving(q);
    q.Pose(1, Sit(MkPose(kCellA, 100, 200), 2, kChair));
    q.Pose(1, Sit(MkPose(kCellA, 150, 200), 2, kChair));
    q.Run(8000);
    CHECK(q.eng.sitCalls >= 3 && q.eng.sitCalls <= 7);
    CHECK(q.Logged("refused to seat the puppet"));
    int refusedLines = 0;
    for (const auto& l : q.log)
        if (l.find("refused to seat the puppet") != std::string::npos) ++refusedLines;
    CHECK(refusedLines == 1);                            // said once
    CHECK(q.F(1)->pos.x > 140);                          // a puppet that cannot sit is still driven where its owner is
}

// A sit state that cannot be read: the belief stands, and a stand-up is taken as done after the settle time (the movement does not wait for ever).
static void t_seat_without_a_readable_state() {
    g_test = "seat_without_a_readable_state";
    Rig r;
    r.eng.furniture = {kChair};
    r.eng.sitReadable = false;
    StartDriving(r);
    r.Pose(1, Sit(MkPose(kCellA, 100, 200), 2, kChair));
    r.Run(6000);
    CHECK(r.eng.sitCalls == 1);                       // no read-back, no drop noticed, no repeats
    r.Pose(1, MkPose(kCellA, 100, 200));
    r.Run(3500);
    CHECK(r.eng.standCalls >= 1);
    r.Pose(1, MkPose(kCellA, 400, 200));
    r.Run(1000);
    CHECK(r.F(1)->pos.x > 350);                       // out and driven again
}

// The puppet is taken down around every save and put back: if its owner still sits, it sits again.
static void t_seat_survives_a_respawn() {
    g_test = "seat_survives_a_respawn";
    Rig r;
    r.eng.furniture = {kChair};
    StartDriving(r);
    r.Pose(1, Sit(MkPose(kCellA, 100, 200), 2, kChair));
    r.Run(1000);
    CHECK(r.eng.sitCalls == 1 && r.F(1)->sit == 4);
    r.pm.OnSaveBegin();
    r.Run(100);
    r.pm.OnSaveEnd();
    CHECK(r.UntilDriving(1));
    r.Run(1500);
    CHECK(r.eng.sitCalls == 2 && r.F(1)->sit == 4);
    CHECK(r.eng.setPosWhileSeated == 0);
}

// Another chair: out of the first one before the second; and a bed is a bed.
static void t_seat_another_furniture_and_bed() {
    g_test = "seat_another_furniture_and_bed";
    Rig r;
    r.eng.furniture = {kChair, kBench};
    StartDriving(r);
    r.Pose(1, Sit(MkPose(kCellA, 100, 200), 2, kChair));
    r.Run(1000);
    r.Pose(1, Sit(MkPose(kCellA, 100, 200), 2, kBench));
    r.Run(2000);
    CHECK(r.eng.sitLog.size() == 2 && r.eng.sitLog.back().second == kBench);
    CHECK(r.eng.standCalls >= 1 && r.F(1)->sit == 4);
    Rig b;
    b.eng.furniture = {kBench};
    StartDriving(b);
    b.Pose(1, Sit(MkPose(kCellA, 100, 200), 1, kBench, true));   // going to bed
    b.Run(1000);
    CHECK(b.Logged("lies down in furniture 20481"));
    // a furniture the owner could not name (0) is nothing to sit in
    Rig c;
    c.eng.furniture = {kChair};
    StartDriving(c);
    c.Pose(1, Sit(MkPose(kCellA, 100, 200), 2, 0));
    c.Run(1000);
    CHECK(c.eng.sitCalls == 0);
}

// An escalation: a stand-up the engine ignores is repeated with a higher attempt number, and given up on (the puppet is driven again) after a while.
static void t_seat_stand_up_escalates() {
    g_test = "seat_stand_up_escalates";
    Rig r;
    r.eng.furniture = {kChair};
    r.eng.standNeedsAttempt = 2;
    StartDriving(r);
    r.Pose(1, Sit(MkPose(kCellA, 100, 200), 2, kChair));
    r.Run(1000);
    r.Pose(1, MkPose(kCellA, 100, 200));
    r.Run(6000);
    CHECK(r.eng.standAttempts.size() >= 3 && r.eng.standAttempts[0] == 0 && r.eng.standAttempts[1] == 1 && r.eng.standAttempts[2] == 2);
    CHECK(r.F(1)->sit == 0);
}

// An engine that refuses the group: a few tries while the event is in the stream, then given up (not every Tick for ever); one refusal is retried.
static void t_oneshot_refusals() {
    g_test = "oneshot_refusals";
    Rig r;
    StartDriving(r);
    r.eng.playGroupRefusalsLeft = 1;
    r.Pose(1, Act(MkPose(kCellA, 100, 200), 32, 5));
    r.Run(600);
    CHECK(CountGroup(r, 32) == 1);
    Rig q;
    StartDriving(q);
    q.eng.playGroupRefusalsLeft = 100;
    q.Pose(1, Act(MkPose(kCellA, 100, 200), 32, 5));
    q.Run(900);
    CHECK(q.pm.stats().oneShotsFailed == 1);
    CHECK(100 - q.eng.playGroupRefusalsLeft == 3);
    q.eng.playGroupRefusalsLeft = 0;
    q.Run(600);
    CHECK(CountGroup(q, 32) == 0);                         // given up for good
}

// The one-shot groups are the NPC layer's: every attack, reaction and block group has a name the puppet can play, a locomotion group is not one.
static void t_action_group_names() {
    g_test = "action_group_names";
    for (int g : {26, 32, 38, 92, 96, 114, 126, 144, 150, 162, 168, 169, 170, 171, 172, 177, 200, 223}) CHECK(IsActionGroup(g) && GroupName(g) != nullptr);
    CHECK(std::string(GroupName(32)) == "AttackRight" && std::string(GroupName(92)) == "AttackPower" && std::string(GroupName(114)) == "AttackThrow");
    CHECK(std::string(GroupName(223)) == "Stagger" && std::string(GroupName(170)) == "BlockIdle" && std::string(GroupName(172)) == "Recoil");
    CHECK(!IsActionGroup(kGrpForward) && !IsActionGroup(kGrpAim) && !IsActionGroup(kGrpJumpStart) && !IsActionGroup(224 /*Death*/) && !IsActionGroup(0));
    CHECK(GroupName(9999) == nullptr && GroupName(-1) == nullptr && GroupName(225) == nullptr);
    CHECK(GroupName(kGrpDeath) != nullptr && std::string(GroupName(kGrpDeath)) == "Death");   // not an action: the held pose of a downed player
    CHECK(IsHeldActionGroup(170) && IsHeldActionGroup(74) && !IsHeldActionGroup(32) && !IsHeldActionGroup(223));
    CHECK(IsReloadActionGroup(177) && IsReloadActionGroup(178) && !IsReloadActionGroup(32));
    // the table the planner uses is unchanged
    CHECK(std::string(GroupName(kGrpIdle)) == "Idle" && std::string(GroupName(kGrpJumpLoop)) == "JumpLoop");
}

// ---- weapon modifications, look updates, hit-target health, the ghost-ref API, the hit router ---------------------------------

static void t_weapon_mods_applied() {
    g_test = "weapon_mods_applied";
    Rig r;
    r.eng.weaponItems = {0x600};
    r.eng.slotOf = {{0x600, 2}};
    r.Add(1, MkPose(kCellA, 100, 200));
    r.pm.SetEquipment(1, {EquipEntry{0x600, true, 0x05}});   // mods 1 and 3
    r.Run(8000);
    auto* f = r.F(1);
    CHECK(f && f->worn.count(0x600) == 1);
    CHECK(f && f->mods[0x600] == 0x05);
    CHECK(r.eng.modLog.size() == 2 && r.eng.modLog[0].second == 1 && r.eng.modLog[1].second == 3);   // one slot per write, in order
    CHECK(r.pm.EquipConverged(1));
    CHECK(r.pm.stats().modOps == 2 && r.pm.stats().modFailures == 0);
    // the mods change later (same weapon): the new mod goes on, the weapon is not equipped again
    const unsigned equips = r.eng.equipCalls;
    r.pm.SetEquipment(1, {EquipEntry{0x600, true, 0x07}});
    r.Run(4000);
    CHECK(r.F(1)->mods[0x600] == 0x07);
    CHECK(r.eng.equipCalls == equips);
    CHECK(r.pm.EquipConverged(1));
}

static void t_weapon_mods_refused_gives_up() {
    g_test = "weapon_mods_refused_gives_up";
    Rig r;
    r.eng.weaponItems = {0x600};
    r.eng.modsAccepted = false;
    r.Add(1, MkPose(kCellA, 100, 200));
    r.pm.SetEquipment(1, {EquipEntry{0x600, true, 0x01}});
    r.Run(30000);
    CHECK(r.F(1)->worn.count(0x600) == 1);        // the weapon itself is on
    CHECK(r.eng.modCalls <= 8);                   // equipMaxFails = 6 tries, then nothing: the mods are cosmetic
    CHECK(r.pm.stats().modGaveUp == 1);
    // a changed wanted set clears the give-up
    r.eng.modsAccepted = true;
    r.pm.SetEquipment(1, {EquipEntry{0x600, true, 0x03}});
    r.Run(4000);
    CHECK(r.F(1)->mods[0x600] == 0x03);
}

static void t_weapon_mods_unverifiable_asked_once() {
    g_test = "weapon_mods_unverifiable_asked_once";
    Rig r;
    r.eng.weaponItems = {0x600};
    r.eng.modsReadable = false;                   // the engine cannot tell which mods the weapon has
    r.Add(1, MkPose(kCellA, 100, 200));
    r.pm.SetEquipment(1, {EquipEntry{0x600, true, 0x03}});
    r.Run(10000);
    CHECK(r.eng.modCalls == 2);                   // once per slot, not every interval
    CHECK(r.pm.EquipConverged(1));
}

static void t_appearance_update() {
    g_test = "appearance_update";
    Rig r;
    r.Add(1, MkPose(kCellA, 100, 200));
    r.Run(1500);
    const RefId ref = r.pm.RefOf(1);
    CHECK(ref != 0);
    const unsigned spawns = r.eng.spawnCalls;
    Appearance a = App();
    a.hair = 0x1234;
    a.height = 1100;
    CHECK(r.pm.SetAppearance(1, a));
    r.Run(500);
    CHECK(r.pm.RefOf(1) == ref && r.eng.spawnCalls == spawns);    // same race and sex: the same puppet, updated in place
    CHECK(r.F(1)->appearance.hair == 0x1234 && r.F(1)->appearance.height == 1100);
    CHECK(r.pm.stats().appearanceUpdates == 1);
    // another sex needs another template NPC: a new puppet, the worn items follow
    r.pm.SetEquipment(1, {{0x510, false}});
    r.Run(3000);
    Appearance f = a;
    f.female = true;
    CHECK(r.pm.SetAppearance(1, f));
    r.Run(4000);
    CHECK(r.pm.RefOf(1) != 0 && r.pm.RefOf(1) != ref);
    CHECK(r.F(1)->base == 0x1001);
    CHECK(r.F(1)->worn.count(0x510) == 1);
    CHECK(!r.pm.SetAppearance(5, a));                              // unknown slot
}

// An engine that builds each puppet from its player's look must not build one from "nothing known": the roster creates the puppet before the Look part
// arrives, and a puppet made from an empty Appearance wears the local player's face with the wrong sex, then is thrown away a moment later.
static void t_spawn_waits_for_the_look() {
    g_test = "spawn_waits_for_the_look";
    {
        Rig r;
        r.eng.looksByBase = true;
        r.pm.AddPlayer(1, Appearance{});
        r.Pose(1, MkPose(kCellA, 100, 200));
        r.Run(1500);
        CHECK(r.eng.spawnCalls == 0 && r.pm.RefOf(1) == 0);
        CHECK(r.Logged("until its look has arrived"));
        Appearance a = App(true);
        a.hair = 0x44;
        a.headParts = {0x55, 0x56};
        CHECK(r.pm.SetAppearance(1, a));
        r.Run(1500);
        CHECK(r.eng.spawnCalls == 1 && r.pm.RefOf(1) != 0);
        CHECK(r.eng.lookBaseCalls.size() == 1);
        CHECK(r.eng.lookBaseCalls.size() == 1 && r.eng.lookBaseCalls[0].second.female && r.eng.lookBaseCalls[0].second.hair == 0x44 &&
              r.eng.lookBaseCalls[0].second.headParts.size() == 2);
        CHECK(r.Logged("spawned as ref") && r.Logged("head parts 2"));
    }
    {   // the look never comes (the other machine has no avatar layer): after lookWaitMs the puppet is made from the template
        Rig r;
        r.eng.looksByBase = true;
        r.pm.AddPlayer(2, Appearance{});
        r.Pose(2, MkPose(kCellA, 100, 200));
        r.Run(2000);
        CHECK(r.eng.spawnCalls == 0);
        r.Run(4000);
        CHECK(r.eng.spawnCalls == 1 && r.pm.RefOf(2) != 0);
        CHECK(r.Logged("no look from slot 2"));
    }
    {   // an engine without look copies has nothing to wait for
        Rig r;
        r.pm.AddPlayer(3, Appearance{});
        r.Pose(3, MkPose(kCellA, 100, 200));
        r.Run(1500);
        CHECK(r.eng.spawnCalls == 1);
    }
}

static void t_puppet_hit_target_health() {
    g_test = "puppet_hit_target_health";
    Rig r;
    r.Add(1, MkPose(kCellA, 100, 200));
    r.Run(1500);
    const RefId ref = r.pm.RefOf(1);
    CHECK(ref != 0);
    r.Run(6000);                                                   // a puppet is hit by enemies: it is topped up every healMs (2 s)
    CHECK(r.eng.resetHealthCalls >= 2 && r.eng.resetHealthCalls <= 4);
    CHECK(r.pm.stats().heals == r.eng.resetHealthCalls);
    r.F(1)->dead = true;                                           // a burst killed it anyway
    r.Run(3000);
    CHECK(r.pm.stats().puppetDeaths == 1);
    CHECK(r.pm.RefOf(1) != 0 && r.pm.RefOf(1) != ref);             // replaced by a fresh puppet, the corpse is deleted
    CHECK(!r.F(1)->dead);
    bool deleted = false;
    for (RefId d : r.eng.deletedRefs) deleted = deleted || d == ref;
    CHECK(deleted);
}

// ---- the pose override: a downed friend's puppet lies there (docs/revive.md "Downed pose") ---------------------------------------

// The override holds against the planned group (the owner keeps walking in the stream) and against the periodic replay: the group is played once,
// with the hold flag, and neither the heartbeat (no readback) nor a pose change plays anything else.
static void t_pose_override_holds() {
    g_test = "pose_override_holds";
    Rig r;
    r.eng.reportsCurrentGroup = false;
    r.Add(1, MkPose(kCellA, 100, 200, 0, 0, kLocoWalkFwd, Vec3{0, 120, 0}));
    r.Run(1500);
    CHECK(r.Group(1) == kGrpForward && r.pm.PoseOverride(1) == kNoGroup);
    r.pm.SetPoseOverride(1, kGrpDeath);
    CHECK(r.pm.PoseOverride(1) == kGrpDeath);
    r.Run(200);
    CHECK(r.Group(1) == kGrpDeath && r.eng.groupLog.back().second == kGrpDeath && (r.eng.groupFlags.back() & 1) == 1);
    const unsigned played = r.eng.playGroupCalls;
    r.Run(r.cfg.animHeartbeatMs * 3);                              // three heartbeats go by
    CHECK(r.eng.playGroupCalls == played && r.Group(1) == kGrpDeath);
    r.Pose(1, MkPose(kCellA, 100, 400, 0, 0, kLocoRunFwd, Vec3{0, 300, 0}));   // the owner runs on in the stream: still lying
    r.Run(600);
    CHECK(r.Group(1) == kGrpDeath && r.eng.playGroupCalls == played);
    CHECK(r.pm.PhaseOf(1) == Phase::Driving && r.pm.LastGroup(1) == kGrpDeath);
    // with readback: the engine standing him up (a reset) is corrected with the held pose, never with the planned group
    Rig q;
    q.Add(1, MkPose(kCellA, 100, 200, 0, 0, kLocoWalkFwd, Vec3{0, 120, 0}));
    q.Run(1500);
    q.pm.SetPoseOverride(1, kGrpDeath);
    q.Run(200);
    const size_t before = q.eng.groupLog.size();
    q.eng.SilentGroupReset(q.pm.RefOf(1));
    q.Run(q.cfg.animMismatchMs + 200);
    CHECK(q.Group(1) == kGrpDeath);
    CHECK(q.eng.groupLog.size() > before);                         // it was corrected ...
    for (size_t i = before; i < q.eng.groupLog.size(); ++i) CHECK(q.eng.groupLog[i].second == kGrpDeath);   // ... with the held pose
    // a menu closing replays the held pose too
    q.eng.SetMenu(true);
    q.Run(200);
    q.eng.SetMenu(false);
    q.Run(300);
    CHECK(q.Group(1) == kGrpDeath);
}

// A puppet the engine calls dead is replaced while it is a hit target; a downed one is not: it lies where it is.
static void t_pose_override_no_dead_respawn() {
    g_test = "pose_override_no_dead_respawn";
    Rig r;
    r.Add(1, MkPose(kCellA, 100, 200));
    r.Run(1500);
    const RefId ref = r.pm.RefOf(1);
    r.pm.SetPoseOverride(1, kGrpDeath);
    r.Run(300);
    r.F(1)->dead = true;
    r.Run(3000);
    CHECK(r.pm.stats().puppetDeaths == 0);
    CHECK(r.pm.RefOf(1) == ref && r.F(1)->dead && r.Group(1) == kGrpDeath);
    for (RefId d : r.eng.deletedRefs) CHECK(d != ref);
    // once he is up again the normal rule applies: the dead puppet is replaced
    r.pm.SetPoseOverride(1, kNoGroup);
    r.Run(3000);
    CHECK(r.pm.stats().puppetDeaths == 1);
    CHECK(r.pm.RefOf(1) != 0 && r.pm.RefOf(1) != ref && !r.F(1)->dead);
}

// Clearing returns to normal driving at once, not at the next heartbeat; clearing what was never set, or for an unknown slot, is harmless.
// S4 (docs/entity-core-plan.md): the binding hook tells the EntityMap every change of RefOf as it happens, so a mirror of what it was told always
// equals the live puppets (twin bindingUnbindForgotten: a parked or deleted puppet stays bound to its player).
static bool MirrorMatches(const Rig& r, const std::map<Slot, RefId>& mirror) {
    std::map<Slot, RefId> live;
    for (const auto& kv : r.pm.LivePuppets()) live[kv.first] = kv.second;
    return live == mirror;
}

static void t_binding_hook_follows_the_live_puppets() {
    g_test = "binding_hook_follows_the_live_puppets";
    Rig r;
    std::map<Slot, RefId> mirror;
    int calls = 0;
    r.pm.SetBindingHook([&](Slot s, RefId ref) { ++calls; if (ref) mirror[s] = ref; else mirror.erase(s); });
    r.Add(1, MkPose(kCellA, 100, 200, 0, 0, kLocoWalkFwd, Vec3{0, 120, 0}));
    r.Add(2, MkPose(kCellA, 300, 200, 0, 0, kLocoWalkFwd, Vec3{0, 120, 0}));
    r.Run(1500);
    CHECK(mirror.size() == 2 && MirrorMatches(r, mirror));
    r.pm.OnSaveBegin();                                   // despawned for the save
    CHECK(mirror.empty() && MirrorMatches(r, mirror));
    r.pm.OnSaveEnd();
    r.Run(1500);
    CHECK(mirror.size() == 2 && MirrorMatches(r, mirror));
    r.pm.OnLoadBegin();                                   // refs dropped without the engine
    CHECK(mirror.empty() && MirrorMatches(r, mirror));
    r.pm.OnLoadEnd();
    r.Run(3000);
    CHECK(MirrorMatches(r, mirror));
    r.pm.RemovePlayer(2);
    CHECK(MirrorMatches(r, mirror) && !mirror.count(2));
    const int before = calls;
    r.Run(500);                                           // nothing changed: nothing is reported
    CHECK(calls == before && MirrorMatches(r, mirror));
    std::map<Slot, RefId> late;                           // a hook set later is told the current state at once
    r.pm.SetBindingHook([&](Slot s, RefId ref) { if (ref) late[s] = ref; else late.erase(s); });
    CHECK(MirrorMatches(r, late));
}

static void t_pose_override_clear() {
    g_test = "pose_override_clear";
    Rig r;
    r.eng.reportsCurrentGroup = false;
    r.Add(1, MkPose(kCellA, 100, 200, 0, 0, kLocoWalkFwd, Vec3{0, 120, 0}));
    r.Run(1500);
    r.pm.SetPoseOverride(1, kGrpDeath);
    r.Run(1000);
    CHECK(r.Group(1) == kGrpDeath);
    r.pm.SetPoseOverride(1, kNoGroup);
    CHECK(r.pm.PoseOverride(1) == kNoGroup);
    r.Step();                                                      // one Tick, not a heartbeat
    CHECK(r.Group(1) == kGrpForward && r.eng.groupLog.back().second == kGrpForward);
    // and the replay of the planned group is back
    r.eng.SilentGroupReset(r.pm.RefOf(1));
    r.Run(r.cfg.animHeartbeatMs + 300);
    CHECK(r.Group(1) == kGrpForward);
    r.pm.SetPoseOverride(1, kNoGroup);
    r.pm.SetPoseOverride(7, kNoGroup);
    r.pm.SetPoseOverride(7, kGrpDeath);                            // a slot without a puppet: remembered, nothing to play
    CHECK(r.pm.PoseOverride(7) == kGrpDeath && r.pm.PoseOverride(1) == kNoGroup);
    r.pm.SetPoseOverride(7, kNoGroup);
    CHECK(r.pm.PoseOverride(7) == kNoGroup);
    // lying, then up, then lying again
    r.pm.SetPoseOverride(1, kGrpDeath);
    r.Run(200);
    CHECK(r.Group(1) == kGrpDeath);
    r.pm.SetPoseOverride(1, kNoGroup);
    r.Run(200);
    CHECK(r.Group(1) == kGrpForward);
    r.pm.SetPoseOverride(1, kGrpDeath);
    r.Run(200);
    CHECK(r.Group(1) == kGrpDeath);
}

// The override belongs to the slot: a new look (another template = a new puppet), a lost ref, a rebuilt 3D, a reconnect, a save and a puppet that
// did not exist yet all keep it. A player who left does not keep it.
static void t_pose_override_survives_respawn() {
    g_test = "pose_override_survives_respawn";
    Rig r;
    r.Add(1, MkPose(kCellA, 100, 200));
    r.Run(1500);
    r.pm.SetPoseOverride(1, kGrpDeath);
    r.Run(200);
    const RefId first = r.pm.RefOf(1);
    Appearance f = App(true);                                      // another sex: a new puppet
    CHECK(r.pm.SetAppearance(1, f));
    r.Run(4000);
    CHECK(r.pm.RefOf(1) != 0 && r.pm.RefOf(1) != first);
    CHECK(r.pm.PoseOverride(1) == kGrpDeath && r.Group(1) == kGrpDeath);
    Appearance same = f;                                           // the same template: updated in place
    same.hair = 0x77;
    CHECK(r.pm.SetAppearance(1, same));
    r.Run(600);
    CHECK(r.Group(1) == kGrpDeath && r.pm.PoseOverride(1) == kGrpDeath);
    const RefId second = r.pm.RefOf(1);
    r.eng.KillRef(second);                                         // the cell dropped it: a fresh puppet
    r.Run(4000);
    CHECK(r.pm.RefOf(1) != 0 && r.pm.RefOf(1) != second && r.Group(1) == kGrpDeath);
    const RefId third = r.pm.RefOf(1);
    r.eng.Rebuild3DInstant(third);                                 // the 3D was rebuilt: the animation is gone, the pose comes back
    r.Run(600);
    CHECK(r.Group(1) == kGrpDeath);
    r.pm.AddPlayer(1, f);                                          // a reconnect replaces the puppet
    r.Run(4000);
    CHECK(r.pm.RefOf(1) != 0 && r.pm.RefOf(1) != third && r.Group(1) == kGrpDeath);
    r.pm.OnSaveBegin();                                            // a save despawns the puppet, it comes back lying
    r.pm.OnSaveEnd();
    r.Run(4000);
    CHECK(r.pm.RefOf(1) != 0 && r.Group(1) == kGrpDeath);
    // the player leaves and joins again: standing
    r.pm.RemovePlayer(1);
    CHECK(r.pm.PoseOverride(1) == kNoGroup);
    r.Add(1, MkPose(kCellA, 100, 200));
    r.Run(2000);
    CHECK(r.pm.RefOf(1) != 0 && r.Group(1) == kGrpIdle);
    // set before the puppet exists
    Rig q;
    q.pm.SetPoseOverride(2, kGrpDeath);
    q.Add(2, MkPose(kCellA, 100, 200));
    q.Run(2000);
    CHECK(q.pm.RefOf(2) != 0 && q.Group(2) == kGrpDeath);
}

// The owner's attacks and reactions do not pop a lying puppet up, and none of them plays late once he is up.
static void t_pose_override_no_one_shots() {
    g_test = "pose_override_no_one_shots";
    Rig r;
    StartDriving(r);
    r.pm.SetPoseOverride(1, kGrpDeath);
    r.Run(200);
    r.Pose(1, Act(MkPose(kCellA, 100, 200), 32, 5));               // a swing in the stream
    r.Run(900);
    r.Pose(1, Act(MkPose(kCellA, 100, 200), 223, 6));              // and a stagger
    r.Run(900);
    CHECK(CountGroup(r, 32) == 0 && CountGroup(r, 223) == 0 && r.pm.stats().oneShots == 0);
    CHECK(r.Group(1) == kGrpDeath);
    r.pm.SetPoseOverride(1, kNoGroup);
    r.Pose(1, Act(MkPose(kCellA, 100, 200), 223, 6));              // still in the stream when he gets up: not played now either
    r.Run(1500);
    CHECK(CountGroup(r, 32) == 0 && CountGroup(r, 223) == 0 && r.pm.stats().oneShots == 0);
    CHECK(r.Group(1) == kGrpIdle);
    r.Pose(1, Act(MkPose(kCellA, 100, 200), 32, 7));               // a new swing after he got up plays as usual
    r.Run(600);
    CHECK(CountGroup(r, 32) == 1 && r.pm.stats().oneShots == 1);
}

// A directory that lies: it lists the local slot and the player's own reference as puppets. The API must not repeat the lie.
struct BadDir : IPuppetDirectory {
    Slot LocalSlot() const override { return 0; }
    std::vector<std::pair<Slot, RefId>> Live() const override { return {{Slot(0), RefId(0x1234)}, {Slot(1), RefId(0x1111)}, {Slot(3), kLocalPlayerRefId}}; }
};
struct DirReg {
    explicit DirReg(const IPuppetDirectory* d) { setPuppetDirectory(d); }
    ~DirReg() { setPuppetDirectory(nullptr); }
};

static void t_api_map() {
    g_test = "api_map";
    {
        Rig r;
        ManagerDirectory dir(r.pm, 0);
        DirReg reg(&dir);
        CHECK(puppetRefOf(1) == 0 && puppetCount() == 0);
        r.Add(1, MkPose(kCellA, 100, 200));
        r.Add(2, MkPose(kCellA, 300, 200));
        r.Run(1500);
        const RefId a = puppetRefOf(1), b = puppetRefOf(2);
        CHECK(a && b && a != b && puppetCount() == 2);
        Slot s = 99;
        CHECK(slotOfPuppet(a, s) && s == 1);
        CHECK(slotOfPuppet(b).value_or(99) == 2);
        CHECK(!slotOfPuppet(0).has_value() && !slotOfPuppet(0xDEAD).has_value());
        CHECK(isPuppet(a) && !isPuppet(kLocalPlayerRefId));
        CHECK(puppetRefOf(0) == 0);                                // the local slot has no puppet
        CHECK(actingRefOf(0) == kLocalPlayerRefId);                // ... the player's own reference acts for it
        CHECK(actingRefOf(1) == a && actingRefOf(2) == b && actingRefOf(5) == 0);
        Slot as = 99;
        CHECK(slotOfActingRef(kLocalPlayerRefId, as) && as == 0);
        CHECK(slotOfActingRef(b, as) && as == 2);
        CHECK(!slotOfActingRef(0, as) && !slotOfActingRef(0xDEAD, as));
        int n = 0;
        forEachPuppet([&](Slot, RefId) { ++n; });
        CHECK(n == 2);
        // a puppet that is gone answers "none", never a stale id
        r.pm.OnSaveBegin();
        CHECK(puppetRefOf(1) == 0 && !isPuppet(a) && puppetCount() == 0 && actingRefOf(1) == 0);
        r.pm.OnSaveEnd();
        r.Run(2000);
        CHECK(puppetRefOf(1) != 0);
    }
    CHECK(puppetRefOf(1) == 0 && actingRefOf(0) == 0 && !isPuppet(0x1111));   // no directory registered: everything answers none
    BadDir bad;
    DirReg reg(&bad);
    CHECK(puppetRefOf(0) == 0);                                    // the lie about the local slot is filtered
    CHECK(puppetRefOf(1) == 0x1111);
    CHECK(!isPuppet(kLocalPlayerRefId));                           // the real player is never a puppet
    CHECK(actingRefOf(0) == kLocalPlayerRefId);
}

struct FakeHitPort : IPuppetHitPort {
    std::vector<std::pair<Slot, PuppetHit>> got;
    bool accept = true;
    HitAction act = HitAction::Clamp;
    bool Forward(Slot s, const PuppetHit& h, HitAction& a) override {
        got.emplace_back(s, h);
        a = act;
        return accept;
    }
};

static void t_hit_router() {
    g_test = "hit_router";
    Rig r;
    ManagerDirectory dir(r.pm, 0);
    DirReg reg(&dir);
    r.Add(1, MkPose(kCellA, 100, 200));
    r.Run(1500);
    const RefId a = puppetRefOf(1);
    CHECK(a != 0);
    PuppetHitRouter rt;
    FakeHitPort port;
    rt.SetPort(&port);
    PuppetHit h;
    h.victimRef = a;
    h.attackerRef = 0x777;
    h.healthDmg = 12;
    h.hitLocation = 3;
    HitOutcome o = rt.OnHit(h);
    CHECK(o.route == HitRoute::Forwarded && o.action == HitAction::Clamp && o.victimSlot == 1);
    CHECK(port.got.size() == 1 && port.got[0].first == 1 && port.got[0].second.healthDmg == 12 && port.got[0].second.attackerRef == 0x777);
    h.victimRef = 0x4242;                                          // not ours: untouched, the engine applies it
    o = rt.OnHit(h);
    CHECK(o.route == HitRoute::NotAPuppet && o.action == HitAction::Proceed && port.got.size() == 1);
    h.victimRef = kLocalPlayerRefId;                               // the real player is not a puppet
    o = rt.OnHit(h);
    CHECK(o.route == HitRoute::NotAPuppet && port.got.size() == 1);
    h.victimRef = a;
    h.healthDmg = 0;                                               // no damage: nothing to report
    o = rt.OnHit(h);
    CHECK(o.route == HitRoute::Ignored && port.got.size() == 1);
    h.healthDmg = 5;
    port.accept = false;                                           // the port could not carry it: the stand-in still must not fall over
    o = rt.OnHit(h);
    CHECK(o.route == HitRoute::PortRefused && o.action == HitAction::Clamp);
    rt.SetPort(nullptr);                                           // combat layer off
    o = rt.OnHit(h);
    CHECK(o.route == HitRoute::NoPort && o.action == HitAction::Clamp);
    port.accept = true;
    port.act = HitAction::Neutralize;
    rt.SetPort(&port);
    o = rt.OnHit(h);
    CHECK(o.route == HitRoute::Forwarded && o.action == HitAction::Neutralize);
    r.pm.OnSaveBegin();                                            // the puppet is gone: a stale ref is not a puppet any more
    o = rt.OnHit(h);
    CHECK(o.route == HitRoute::NotAPuppet);
    CHECK(rt.stats().forwarded == 2 && rt.stats().clamped == 3 && rt.stats().neutralized == 1);
    CHECK(rt.StatusJson().find("\"forwarded\":2") != std::string::npos);
}

// ---- the suite and the twins ------------------------------------------------------------------------------------------

static bool g_verbose = false;
#define RUN(f)                                \
    do {                                      \
        if (g_verbose) {                      \
            std::printf("  > %s\n", #f);      \
            std::fflush(stdout);              \
        }                                     \
        f();                                  \
    } while (0)

static void RunSuite() {
    RUN(t_loco_basics);
    RUN(t_planner_groups);
    RUN(t_speed_matching_planner);
    RUN(t_spawn_waits_for_3d);
    RUN(t_restraint_reasserted);
    RUN(t_anim_after_menu);
    RUN(t_post_menu_snap);
    RUN(t_anim_heartbeat);
    RUN(t_no_slide_manager);
    RUN(t_frozen_and_stale);
    RUN(t_jump_sequence);
    RUN(t_weapon_draw_after_equip);
    RUN(t_equipment_converges);
    RUN(t_equipment_backoff_giveup);
    RUN(t_equipment_flap_guard);
    RUN(t_local_slot_and_limits);
    RUN(t_duplicate_slot);
    RUN(t_remove_player);
    RUN(t_stale_and_reused_refs);
    RUN(t_park_when_cell_unloaded);
    RUN(t_cross_cell_move);
    RUN(t_exterior_border);
    RUN(t_rebuild_3d_reasserts);
    RUN(t_save_despawns);
    RUN(t_load_purges_orphans);
    RUN(t_first_tick_purge);
    RUN(t_spawn_retry);
    RUN(t_template_choice);
    RUN(t_no_pose_no_spawn);
    RUN(t_yaw_handling);
    RUN(t_position_rate);
    RUN(t_snap_and_drift);
    RUN(t_physics_push_not_adopted);
    RUN(t_body_room);
    RUN(t_group_refused_retries);
    RUN(t_call_budget);
    RUN(t_ref_map);
    RUN(t_script_backend);
    RUN(t_manager_on_script_backend);
    RUN(t_live_bridge);
    RUN(t_oneshot_plays_once);
    RUN(t_oneshot_duplicates_and_reordering);
    RUN(t_oneshot_lost_packet);
    RUN(t_oneshot_only_while_driving);
    RUN(t_oneshot_held_block);
    RUN(t_oneshot_not_cut_by_readback);
    RUN(t_reload_group_from_owner);
    RUN(t_oneshot_refusals);
    RUN(t_oneshot_unarmed_punch_draws_first);
    RUN(t_oneshot_unseen_swing_is_reported);
    RUN(t_oneshot_unseen_swing_is_retried);
    RUN(t_oneshot_waits_for_the_engines_weapon);
    RUN(t_look_is_checked_after_the_spawn);
    RUN(t_seat_follows_the_owner);
    RUN(t_seat_dropped_and_refused);
    RUN(t_seat_without_a_readable_state);
    RUN(t_seat_survives_a_respawn);
    RUN(t_seat_another_furniture_and_bed);
    RUN(t_seat_stand_up_escalates);
    RUN(t_action_group_names);
    RUN(t_weapon_mods_applied);
    RUN(t_weapon_mods_refused_gives_up);
    RUN(t_weapon_mods_unverifiable_asked_once);
    RUN(t_appearance_update);
    RUN(t_spawn_waits_for_the_look);
    RUN(t_puppet_hit_target_health);
    RUN(t_pose_override_holds);
    RUN(t_pose_override_no_dead_respawn);
    RUN(t_pose_override_clear);
    RUN(t_binding_hook_follows_the_live_puppets);
    RUN(t_pose_override_survives_respawn);
    RUN(t_pose_override_no_one_shots);
    RUN(t_api_map);
    RUN(t_hit_router);
}

struct Twin {
    const char* name;
    void (*set)(AvatarFaults&);
};
#define TWIN(n) {#n, [](AvatarFaults& f) { f.n = true; }}
static const Twin kTwins[] = {
    TWIN(noRestrainReassert), TWIN(noMenuReassert), TWIN(noWait3D), TWIN(trustLocoCode), TWIN(no3DGenReassert),
    TWIN(saveKeepsPuppets), TWIN(noLoadPurge), TWIN(usesPitchAsYaw), TWIN(noEquipAddFirst), TWIN(noEquipBackoff),
    TWIN(noEquipFlapGuard), TWIN(weaponDrawBeforeEquip), TWIN(allowLocalSlot), TWIN(duplicateSlotSpawns), TWIN(noStaleRefCheck),
    TWIN(neverPark), TWIN(crossCellSetPos), TWIN(frozenKeepsRunning), TWIN(noYawNormalize), TWIN(noAnimHeartbeat),
    TWIN(noJumpLand), TWIN(ignoreBudget), TWIN(staleMapEntries), TWIN(noPostMenuSnap), TWIN(noSaveWait),
    TWIN(noWeaponMods), TWIN(modsIgnoredInSet), TWIN(modsRetryForever), TWIN(appearanceUpdateRespawns), TWIN(noLookWait), TWIN(noPuppetHeal),
    TWIN(noPuppetDeathRespawn), TWIN(apiLocalGetsPuppet), TWIN(apiPlayerIsPuppet), TWIN(apiActingNeedsPuppet), TWIN(hitsForwardEmpty),
    TWIN(puppetHitProceeds), TWIN(adoptPhysicsPush), TWIN(noBodyRoom),
    TWIN(noOneShots), TWIN(oneShotReplays), TWIN(oneShotNoBaseline), TWIN(oneShotKeepsBaseline), TWIN(oneShotNoWrap), TWIN(oneShotNextOnly),
    TWIN(oneShotWhileFrozen), TWIN(noOneShotReassert), TWIN(noOneShotMismatchGuard), TWIN(reloadGroupFixed), TWIN(heldNeverEnds),
    TWIN(bridgeDropsAction), TWIN(oneShotAdoptsStale), TWIN(oneShotNoResync), TWIN(noAttackDraw), TWIN(noOneShotVerify),
    TWIN(poseOverrideIgnored), TWIN(poseReplaysPlanned), TWIN(poseHeartbeats), TWIN(poseNoDeathGuard), TWIN(poseOneShotsPlay),
    TWIN(poseClearIgnored), TWIN(poseLostOnRespawn), TWIN(poseSurvivesRemove),
    TWIN(bindingUnbindForgotten),
    TWIN(noAttackWeaponCheck), TWIN(noAttackRetry), TWIN(noSeatSync), TWIN(seatKeepsDriving), TWIN(seatNoStand), TWIN(seatRepeats),
};

int main(int argc, char** argv) {
    bool twins = true;
    const char* only = nullptr;
    for (int i = 1; i < argc; ++i) {
        if (!std::strcmp(argv[i], "--no-twins")) twins = false;
        else if (!std::strcmp(argv[i], "--verbose")) g_verbose = true;
        else if (!std::strcmp(argv[i], "--twin") && i + 1 < argc) only = argv[++i];
        else if (!std::strcmp(argv[i], "--list-twins")) {
            for (const Twin& t : kTwins) std::printf("%s\n", t.name);
            return 0;
        }
    }
    if (only) {
        for (const Twin& t : kTwins)
            if (!std::strcmp(t.name, only)) {
                AvatarFaults f;
                t.set(f);
                AvatarFaultScope scope(f);
                g_quiet = true;
                RunSuite();
                std::printf("twin %s: %d failed checks -> %s\n", only, g_fail, g_fail ? "killed" : "SURVIVED");
                return g_fail ? 1 : 0;
            }
        std::printf("unknown twin %s\n", only);
        return 2;
    }

    RunSuite();
    std::printf("avatar: %d checks, %d failed\n", g_checks, g_fail);
    int realFail = g_fail;
    if (realFail) return 1;
    if (!twins) {
        std::printf("avatar: suite passes (twins skipped)\n");
        return 0;
    }
    int survivors = 0;
    g_quiet = true;
    for (const Twin& t : kTwins) {
        AvatarFaults f;
        t.set(f);
        AvatarFaultScope scope(f);
        g_fail = 0;
        RunSuite();
        std::printf("  twin %-24s %s (%d failed checks)\n", t.name, g_fail ? "killed" : "SURVIVED", g_fail);
        if (!g_fail) ++survivors;
    }
    std::printf("avatar: %zu twins, %d survived\n", sizeof(kTwins) / sizeof(kTwins[0]), survivors);
    if (survivors) return 1;
    std::printf("avatar: %d checks, 0 failed, all %zu twins killed - ALL OK\n", g_checks, sizeof(kTwins) / sizeof(kTwins[0]));
    return 0;
}
