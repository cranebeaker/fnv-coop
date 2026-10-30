// Tests of the real puppet engine (plugin/avatar/engine) on FakeGameBackend:
//   * GamePuppetEngine: spawn, guards, per-axis writes, console text, 3D tracking, equipment, purge scan;
//   * the full stack PuppetManager -> GamePuppetEngine -> fake game (the manager's traps again, through the real engine);
//   * LocalSampler / EquipWatcher: what the local player publishes;
//   * AvatarRuntime: loopback puppet of the local player, remote players through the interpolation buffer, game events, avatar.status.
//
//   avatar_engine_tests                run everything (exit 1 on a failed check)
//   avatar_engine_tests --list         test names
//   avatar_engine_tests NAME...        only tests whose name contains NAME
//   avatar_engine_tests --mut N        run with mutation twin N (same as FNVCOOP_MUT=N; tools/mut/run_twins.py runs all of them)
//   avatar_engine_tests --verbose      print the console lines of failing tests
//
// Everything here is a model of the game. It proves the logic and the exact console text, not that the engine accepts the calls.
#include <algorithm>
#include <cmath>
#include <cstdio>
#include <cstring>
#include <functional>
#include <map>
#include <memory>
#include <string>
#include <vector>

#include "common/minijson.h"
#include "fnvcoop_mut.h"
#include "avatar_runtime.h"
#include "local_sampler.h"
#include "puppet_game_engine.h"
#include "face_blocks.h"
#include "run_state.h"
#include "testing/fake_backend.h"
#include "../../avatar_groups.h"
#include "../../loco.h"

using namespace nvcoop::avatar;
using namespace nvcoop::avatar::engine;

static int g_fail = 0, g_checks = 0, g_testFail = 0;
static bool g_verbose = false;
static const char* g_test = "";
#define CHECK(c)                                                                                  \
    do {                                                                                          \
        ++g_checks;                                                                               \
        if (!(c)) {                                                                               \
            ++g_fail;                                                                             \
            ++g_testFail;                                                                         \
            std::printf("  FAIL [%s] %s:%d  %s\n", g_test, __FILE__, __LINE__, #c);               \
        }                                                                                         \
    } while (0)
#define CHECK_NEAR(a, b, eps) CHECK(std::fabs(static_cast<double>(a) - static_cast<double>(b)) <= (eps))

namespace {

constexpr FormId kTmpl = 0x1000, kTmplF = 0x1001, kUntagged = 0x7777;
constexpr FormId kArmor = 0x2000, kHat = 0x2001, kOutfit = 0x2FFF, kGun = 0x3000, kGun2 = 0x3001;
constexpr CellId kExtA = FakeGameBackend::kExtA, kExtB = FakeGameBackend::kExtB, kInt = FakeGameBackend::kInterior;

EngineOptions Opts() {
    EngineOptions o;
    o.taggedBases = {kTmpl, kTmplF};
    return o;
}

void SetupBackend(FakeGameBackend& be) {
    be.knownBases = {kTmpl, kTmplF};
    be.weaponItems = {kGun, kGun2};
    be.slotOf = {{kArmor, 1}, {kOutfit, 1}, {kHat, 2}};
}

// ---- helpers ---------------------------------------------------------------------------------------------------------

std::string LastLine(const FakeGameBackend& be) { return be.lines.empty() ? std::string() : be.lines.back().text; }

bool HasLine(const FakeGameBackend& be, RefId ctx, const std::string& text) {
    for (const auto& l : be.lines)
        if (l.ctx == ctx && l.text == text) return true;
    return false;
}

size_t CountLines(const FakeGameBackend& be, RefId ctx, const std::string& prefix, size_t from = 0) {
    size_t n = 0;
    for (size_t i = from; i < be.lines.size(); ++i)
        if (be.lines[i].ctx == ctx && be.lines[i].text.compare(0, prefix.size(), prefix) == 0) ++n;
    return n;
}

double NumberAfter(const std::string& s, const std::string& prefix) {
    return s.compare(0, prefix.size(), prefix) == 0 ? std::atof(s.c_str() + prefix.size()) : -1e9;
}

// engine rig ------------------------------------------------------------------------------------------------------------
struct ERig {
    FakeGameBackend be;
    GamePuppetEngine eng;
    ERig() : eng(be, Opts()) { SetupBackend(be); }
    void Settle(int frames = 4) {
        for (int i = 0; i < frames; ++i) be.Frame();
    }
    // spawns and waits for 3D
    RefId Make(const Vec3& pos = Vec3{300, 200, 0}, CellId cell = kExtA, FormId base = kTmpl) {
        RefId r = eng.Spawn(base, cell, pos, 0.0f);
        Settle();
        return r;
    }
};

// full stack: manager + real engine + fake game -------------------------------------------------------------------------
Config MakeCfg() {
    Config c;
    c.localSlot = 0;
    c.templates = {{1, false, kTmpl}, {1, true, kTmplF}};
    c.defaultTemplate = kTmpl;
    return c;
}

Appearance App(bool female = false) {
    Appearance a;
    a.race = 1;
    a.female = female;
    a.name = "Courier";
    return a;
}

PuppetPose MkPose(CellId cell, float x, float y, float z = 0, float yaw = 0, uint8_t loco = kLocoIdle, Vec3 vel = Vec3{}) {
    PuppetPose p;
    p.valid = true;
    p.cell = cell;
    p.pos = Vec3{x, y, z};
    p.rot = Vec3{0, yaw, 0};
    p.vel = vel;
    p.anim.loco = loco;
    return p;
}

struct SRig {
    FakeGameBackend be;
    GamePuppetEngine eng;
    Config cfg;
    PuppetManager pm;
    uint32_t now = 1000;
    std::map<Slot, PuppetPose> cur;
    explicit SRig(const std::function<void(Config&)>& tweak = nullptr) : eng(be, Opts()), cfg(Make(tweak)), pm(eng, cfg) { SetupBackend(be); }
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
        be.Frame();
        be.now += static_cast<uint64_t>(ms) * 1000;
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
    FakeGameBackend::FRef* F(Slot s) {
        static FakeGameBackend::FRef none;
        none = FakeGameBackend::FRef{};
        auto* f = be.Get(pm.RefOf(s));
        return f ? f : &none;
    }
};

// ======================================================================================================================
// GamePuppetEngine
// ======================================================================================================================

void spawn_places_and_finds_new_ref() {
    ERig r;
    r.be.load3DFrames = 0;   // 3D at once: the placement lines are accepted
    r.be.Player().pos = Vec3{10, 20, 0};
    RefId ref = r.eng.Spawn(kTmpl, kExtA, Vec3{500, 300, 20}, 1.0f);
    CHECK(ref != 0);
    auto* f = r.be.Get(ref);
    CHECK(f && f->base == kTmpl && f->cell == kExtA);
    CHECK(r.be.lines.size() >= 1 && r.be.lines[0].text == "PlaceAtMe 00001000 1 0 0" && r.be.lines[0].ctx == kPlayerRef);
    CHECK(f && std::fabs(f->pos.x - 500) < 0.01 && std::fabs(f->pos.y - 300) < 0.01 && std::fabs(f->pos.z - 20) < 0.01);
    CHECK(f && std::fabs(f->rot.z - 1.0f) < 0.001);   // placed before the first frame is drawn
    CHECK(r.eng.Owns(ref) && r.eng.OwnedCount() == 1);
    CHECK(r.be.badLines.empty());
    CHECK(r.eng.IsValid(ref));
}

void spawn_without_3d_still_returns_ref() {
    ERig r;   // 3D arrives 2 frames later: the placement lines are refused (not counted as a failure of the spawn)
    RefId ref = r.eng.Spawn(kTmpl, kExtA, Vec3{500, 300, 20}, 1.0f);
    CHECK(ref != 0);
    CHECK(r.eng.Owns(ref));
    CHECK(r.be.badLines.empty());
    r.Settle();
    CHECK(r.eng.Has3D(ref));
}

void spawn_anchor_other_cell() {
    ERig r;   // the player is in kExtA; the target cell is the neighbouring exterior cell: anchor on a static there
    r.be.load3DFrames = 0;
    RefId ref = r.eng.Spawn(kTmpl, kExtB, Vec3{4200, 10, 0}, 0.0f);
    CHECK(ref != 0);
    CHECK(r.be.lines.size() >= 1 && r.be.lines[0].ctx == 0x501 && r.be.lines[0].text == "PlaceAtMe 00001000 1 0 0");
    auto* f = r.be.Get(ref);
    CHECK(f && f->cell == kExtB);
    // interior: anchor on the interior static, never on the player
    RefId in = r.eng.Spawn(kTmplF, kInt, Vec3{110, 100, 0}, 0.0f);
    CHECK(in != 0);
    CHECK(r.be.Get(in) && r.be.Get(in)->cell == kInt);
    CHECK(r.be.badLines.empty());
}

void spawn_failures_are_counted() {
    {
        ERig r;   // a cell with no reference to anchor on
        r.be.SetCell(0x999, true, true, 0);
        CHECK(r.eng.Spawn(kTmpl, 0x999, Vec3{}, 0) == 0);
        CHECK(r.eng.stats().spawnNoAnchor == 1);
        CHECK(r.be.lines.empty());
    }
    {
        ERig r;   // the target cell is not attached
        r.be.SetCell(kExtB, false, false, FakeGameBackend::kWorld);
        CHECK(r.eng.Spawn(kTmpl, kExtB, Vec3{}, 0) == 0);
        CHECK(r.be.lines.empty());
    }
    {
        ERig r;   // PlaceAtMe refused
        r.be.refusePlaceAtMe = true;
        CHECK(r.eng.Spawn(kTmpl, kExtA, Vec3{}, 0) == 0);
        CHECK(r.eng.stats().spawnRefused == 1 && r.eng.OwnedCount() == 0);
    }
    {
        ERig r;   // the form exists in our config but the engine does not know it
        ERig r2;
        r2.be.knownBases.clear();
        CHECK(r2.eng.Spawn(kTmpl, kExtA, Vec3{}, 0) == 0);
    }
    {
        ERig r;   // NaN position
        CHECK(r.eng.Spawn(kTmpl, kExtA, Vec3{NAN, 0, 0}, 0) == 0);
        CHECK(r.be.lines.empty());
    }
}

void spawn_reuses_a_freed_id() {
    ERig r;
    r.be.reuseIds = true;
    RefId a = r.Make();
    CHECK(a != 0);
    r.eng.Delete(a);
    CHECK(!r.eng.IsValid(a));
    RefId b = r.Make();   // the engine hands the same id out again for a new object
    CHECK(b == a);
    CHECK(r.eng.Owns(b) && r.eng.IsValid(b));
    CHECK(r.eng.Has3D(b));
}

void spawn_rejected_ref_is_removed_again() {
    ERig r;
    r.be.placeDisabled = true;   // whatever the reason: the new ref fails our own guard
    RefId ref = r.eng.Spawn(kTmpl, kExtA, Vec3{}, 0);
    CHECK(ref == 0);
    CHECK(r.eng.stats().spawnRejected == 1 && r.eng.OwnedCount() == 0);
    std::vector<ActorEntry> es;
    r.be.ListActors(kExtA, es);
    int stray = 0;
    for (auto& e : es)
        if (e.base == kTmpl && !r.be.Get(e.ref)->deleted) ++stray;
    CHECK(stray == 0);   // not left behind in the world
    CHECK(CountLines(r.be, r.be.lines.back().ctx, "MarkForDelete") == 1);
}

void spawn_circuit_breaker() {
    ERig r;
    r.be.load3DFrames = 0;
    int made = 0;
    for (int i = 0; i < 20; ++i) {
        RefId ref = r.eng.Spawn(kTmpl, kExtA, Vec3{}, 0);
        if (!ref) break;
        ++made;
        r.eng.Delete(ref);   // a driver that keeps losing its refs
    }
    CHECK(made == 8);
    CHECK(r.eng.stats().spawnThrottled >= 1);
    size_t n = r.be.lines.size();
    CHECK(r.eng.Spawn(kTmpl, kExtA, Vec3{}, 0) == 0);
    CHECK(r.be.lines.size() == n);   // refused without a single console line
    r.be.now += 11000000;            // the window passes
    CHECK(r.eng.Spawn(kTmpl, kExtA, Vec3{}, 0) != 0);
}

void spawn_untagged_base_refused() {
    ERig r;
    r.be.knownBases.insert(kUntagged);
    CHECK(r.eng.Spawn(kUntagged, kExtA, Vec3{}, 0) == 0);
    CHECK(r.be.lines.empty());   // a base we would not recognise again could never be purged: never created
    CHECK(r.eng.Spawn(0, kExtA, Vec3{}, 0) == 0);
}

void spawn_takes_the_newest_new_ref() {
    ERig r;
    r.be.placeExtraActors = 1;   // two actors appear
    RefId a = r.be.AddActor(kTmpl, kExtA, Vec3{1, 1, 0});   // an older actor with the same base is already there
    RefId ref = r.eng.Spawn(kTmpl, kExtA, Vec3{}, 0);
    CHECK(ref != 0 && ref != a);
    CHECK(r.eng.stats().spawnAmbiguous == 1);
    // the two new refs have rising ids; the newest one is ours
    std::vector<ActorEntry> es;
    r.be.ListActors(kExtA, es);
    RefId maxId = 0;
    for (auto& e : es) maxId = std::max(maxId, e.ref);
    CHECK(ref == maxId);
}

void spawn_does_not_adopt_existing() {
    ERig r;
    RefId old = r.be.AddActor(kTmpl, kExtA, Vec3{5, 5, 0});
    RefId ref = r.eng.Spawn(kTmpl, kExtA, Vec3{}, 0);
    CHECK(ref != 0 && ref != old);
    CHECK(!r.eng.Owns(old));
}

void guard_foreign_actor_untouched() {
    ERig r;
    RefId f = r.be.AddActor(kUntagged, kExtA, Vec3{10, 10, 0});
    r.Settle();
    CHECK(!r.eng.IsValid(f));
    CHECK(!r.eng.SetPos(f, Vec3{50, 50, 0}));
    CHECK(!r.eng.SetYaw(f, 1.0f));
    CHECK(!r.eng.PlayGroup(f, kGrpForward, 1));
    CHECK(!r.eng.SetRestrained(f, true));
    CHECK(!r.eng.MakeNonCombatant(f));
    CHECK(!r.eng.AddItem(f, kArmor, 1));
    CHECK(!r.eng.EquipItem(f, kArmor));
    CHECK(!r.eng.SetSneak(f, true));
    CHECK(!r.eng.MoveToCell(f, kExtB, Vec3{}, 0));
    r.eng.Delete(f);
    CHECK(r.be.lines.empty());
    CHECK(r.be.Get(f)->exists && !r.be.Get(f)->deleted);
    CHECK(r.eng.stats().foreignRefused > 0);
}

void guard_placed_actor_with_a_tagged_base_is_not_ours() {
    // template = a vanilla NPC's base form: the real NPC (placed in a plugin, id 0x00xxxxxx) shares the base and must never be touched
    ERig r;
    RefId real = r.be.AddActor(kTmpl, kExtA, Vec3{10, 10, 0}, true, 0x00123456);
    RefId made = r.be.AddActor(kTmpl, kExtA, Vec3{20, 20, 0});
    CHECK(!r.eng.IsValid(real) && r.eng.IsValid(made));
    CHECK(!r.eng.SetPos(real, Vec3{50, 50, 0}));
    r.eng.Delete(real);
    CHECK(r.be.linesTo(real) == 0);
    std::vector<RefId> got = r.eng.FindTaggedPuppets();
    CHECK(got.size() == 1 && got[0] == made);   // the purge scan sees only created references
}

void guard_player_untouched() {
    ERig r;
    r.eng.SetTaggedBases({kTmpl, kTmplF, 7});   // even when the player's own base is a puppet template (template=player)
    CHECK(!r.eng.IsValid(kPlayerRef));
    CHECK(!r.eng.SetPos(kPlayerRef, Vec3{9, 9, 9}));
    CHECK(!r.eng.PlayGroup(kPlayerRef, kGrpForward, 1));
    CHECK(!r.eng.SetRestrained(kPlayerRef, true));
    r.eng.Delete(kPlayerRef);
    CHECK(r.be.lines.empty());
}

void guard_reused_id_with_other_base() {
    ERig r;
    RefId ref = r.Make();
    CHECK(r.eng.IsValid(ref));
    r.be.HijackId(ref, kTmplF);   // another tagged base took the id: recorded base differs, so it is not the object we spawned
    CHECK(!r.eng.IsValid(ref));
    size_t before = r.be.lines.size();
    CHECK(!r.eng.SetPos(ref, Vec3{1, 1, 1}));
    r.eng.Delete(ref);
    CHECK(r.be.lines.size() == before);
    CHECK(!r.eng.Owns(ref));
    r.be.HijackId(ref, kUntagged);   // an untagged base: also invalid
    CHECK(!r.eng.IsValid(ref));
}

void delete_two_commands_in_order() {
    ERig r;
    RefId ref = r.Make();
    size_t n0 = r.be.lines.size();
    r.eng.Delete(ref);
    CHECK(r.be.lines.size() == n0 + 2);
    CHECK(r.be.lines[n0].text == "Disable" && r.be.lines[n0].ctx == ref);
    CHECK(r.be.lines[n0 + 1].text == "MarkForDelete" && r.be.lines[n0 + 1].ctx == ref);
    CHECK(!r.eng.Owns(ref));
    CHECK(!r.eng.IsValid(ref));   // flagged deleted: no longer ours to drive
    // deleting something that is gone sends nothing
    r.be.Kill(ref);
    size_t n1 = r.be.lines.size();
    r.eng.Delete(ref);
    r.eng.Delete(0);
    CHECK(r.be.lines.size() == n1);
}

void setpos_sends_only_differing_axes() {
    ERig r;
    RefId ref = r.Make(Vec3{100, 100, 0});
    r.be.Get(ref)->pos = Vec3{100, 100, 0};
    size_t n0 = r.be.lines.size();
    CHECK(r.eng.SetPos(ref, Vec3{100, 100, 0}));
    CHECK(r.be.lines.size() == n0);   // nothing to do costs nothing
    CHECK(r.eng.stats().setPosSkipped >= 1);
    CHECK(r.eng.SetPos(ref, Vec3{101.5f, 100, 0}));
    CHECK(r.be.lines.size() == n0 + 1 && LastLine(r.be) == "SetPos X 101.500");
    CHECK(r.eng.SetPos(ref, Vec3{110, 90, 5.25f}));
    CHECK(r.be.lines.size() == n0 + 4);
    CHECK(r.be.lines[n0 + 1].text == "SetPos X 110.000" && r.be.lines[n0 + 2].text == "SetPos Y 90.000" && r.be.lines[n0 + 3].text == "SetPos Z 5.250");
    CHECK_NEAR(r.be.Get(ref)->pos.x, 110, 0.01);
    CHECK_NEAR(r.be.Get(ref)->pos.z, 5.25, 0.01);
    // a push by physics is undone with exactly the axes that differ
    r.be.Push(ref, Vec3{110, 140, 5.25f});
    size_t n1 = r.be.lines.size();
    CHECK(r.eng.SetPos(ref, Vec3{110, 90, 5.25f}));
    CHECK(r.be.lines.size() == n1 + 1 && LastLine(r.be) == "SetPos Y 90.000");
    // negative numbers, tiny values, no exponent, no decimal comma
    CHECK(r.eng.SetPos(ref, Vec3{-1234.5678f, 90, 5.25f}));
    CHECK(LastLine(r.be) == "SetPos X -1234.568");
    CHECK(r.be.badLines.empty());
}

void setpos_needs_3d_and_finite() {
    ERig r;
    RefId ref = r.Make(Vec3{100, 100, 0});
    r.be.Lose3D(ref, 5);
    size_t n0 = r.be.lines.size();
    CHECK(!r.eng.SetPos(ref, Vec3{500, 500, 0}));
    CHECK(!r.eng.SetYaw(ref, 2.0f));
    CHECK(!r.eng.PlayGroup(ref, kGrpIdle, 1));
    CHECK(!r.eng.SetRestrained(ref, true));
    CHECK(!r.eng.EquipItem(ref, kArmor));
    CHECK(r.be.lines.size() == n0);   // no console line is paid for while the ref has no 3D
    CHECK(r.eng.stats().refused3D >= 5);
    r.Settle(6);
    CHECK(!r.eng.SetPos(ref, Vec3{NAN, 0, 0}));
    CHECK(!r.eng.SetPos(ref, Vec3{0, INFINITY, 0}));
    CHECK(!r.eng.SetYaw(ref, NAN));
    CHECK(r.be.lines.size() == n0);
    CHECK(r.eng.SetPos(ref, Vec3{500, 500, 0}));
    CHECK(r.be.lines.size() > n0);
}

void setyaw_degrees_wrapped_and_skipped() {
    ERig r;
    RefId ref = r.Make();
    CHECK(r.eng.SetYaw(ref, 0.5f));
    CHECK(LastLine(r.be) == "SetAngle Z 28.648");
    size_t n = r.be.lines.size();
    CHECK(r.eng.SetYaw(ref, 0.5f));
    CHECK(r.be.lines.size() == n);   // already there
    CHECK(r.eng.SetYaw(ref, 0.5005f));
    CHECK(r.be.lines.size() == n);   // within the epsilon
    CHECK(r.eng.SetYaw(ref, 3.5f));  // 3.5 rad is past pi: sent as the equivalent negative angle
    double deg = NumberAfter(LastLine(r.be), "SetAngle Z ");
    CHECK(deg < -150.0 && deg > -170.0);
    CHECK(r.eng.SetYaw(ref, 7.0f));  // more than a full turn
    deg = NumberAfter(LastLine(r.be), "SetAngle Z ");
    CHECK(deg > 35.0 && deg < 45.0);
    CHECK(r.be.badLines.empty());
}

void playgroup_text() {
    ERig r;
    RefId ref = r.Make();
    CHECK(r.eng.PlayGroup(ref, kGrpForward, 1));
    CHECK(LastLine(r.be) == "PlayGroup Forward 1");
    CHECK(r.eng.PlayGroup(ref, kGrpFastRight, 0));
    CHECK(LastLine(r.be) == "PlayGroup FastRight 0");
    size_t n = r.be.lines.size();
    CHECK(!r.eng.PlayGroup(ref, 999, 1));   // a group this layer does not know is never guessed
    CHECK(!r.eng.PlayGroup(ref, -1, 1));
    CHECK(r.be.lines.size() == n);
    CHECK(r.be.badLines.empty());
}

void currentgroup_reads_any_slot() {
    ERig r;
    RefId ref = r.Make();
    CHECK(r.eng.CurrentGroup(ref) == kGrpIdle);   // nothing running yet
    CHECK(r.eng.PlayGroup(ref, kGrpForward, 1));
    CHECK(r.eng.CurrentGroup(ref) == kGrpForward);
    r.be.Get(ref)->groupSlot = 4;   // wherever the engine keeps it
    CHECK(r.eng.CurrentGroup(ref) == kGrpForward);
    r.be.Get(ref)->groupSlot = 0;
    CHECK(r.eng.CurrentGroup(ref) == kGrpForward);
    r.be.Get(ref)->group = kGrpIdle;   // the engine went back to idle by itself
    CHECK(r.eng.CurrentGroup(ref) == kGrpIdle);
    r.be.Get(ref)->group = kGrpAim;
    CHECK(r.eng.CurrentGroup(ref) == kGrpAim);   // a different group is reported as it is (the driver sees the mismatch)
    r.be.Get(ref)->group = -1;                   // reset by a menu: every slot empty
    CHECK(r.eng.CurrentGroup(ref) == kGrpIdle);
    r.be.animReadable = false;
    CHECK(r.eng.CurrentGroup(ref) == -1);        // unreadable: the driver falls back to its heartbeat
}

void restraint_readback_and_fallback() {
    ERig r;
    RefId ref = r.Make();
    CHECK(r.eng.SetRestrained(ref, true));
    CHECK(LastLine(r.be) == "SetRestrained 1");
    CHECK(r.eng.IsRestrained(ref));
    r.be.Get(ref)->restrained = false;   // the AI took over
    CHECK(!r.eng.IsRestrained(ref));
    CHECK(r.eng.SetRestrained(ref, true));
    r.be.evalBroken = {Cond::GetRestrained};
    uint64_t f0 = r.eng.stats().evalFailures;
    CHECK(r.eng.IsRestrained(ref));   // evaluator unavailable: trust what we asked for
    CHECK(r.eng.stats().evalFailures == f0 + 1);
    CHECK(r.eng.SetRestrained(ref, false));
    CHECK(LastLine(r.be) == "SetRestrained 0");
    CHECK(!r.eng.IsRestrained(ref));
    // a rebuilt 3D drops the restraint: the fallback must not claim it still holds
    CHECK(r.eng.SetRestrained(ref, true));
    CHECK(r.eng.Generation3D(ref) == 1);
    r.be.Rebuild3D(ref);
    CHECK(r.eng.Generation3D(ref) == 2);
    CHECK(!r.eng.IsRestrained(ref));
}

void nonCombatant_lines() {
    ERig r;
    RefId ref = r.Make();
    size_t n = r.be.lines.size();
    CHECK(r.eng.MakeNonCombatant(ref));
    CHECK(r.be.lines.size() == n + 1 && r.be.lines[n].text == "SetIgnoreFriendlyHits 1");
    CHECK(!r.be.Get(ref)->ghost && r.be.Get(ref)->ignoreFriendly);   // hittable: the combat layer needs the hit event on a puppet
    ERig g;
    EngineOptions o = Opts();
    o.ghostPuppets = true;
    GamePuppetEngine ge(g.be, o);
    RefId gr = g.be.AddActor(kTmpl, kExtA, Vec3{1, 1, 0});
    g.Settle();
    CHECK(ge.MakeNonCombatant(gr));
    CHECK(g.be.Get(gr)->ghost && g.be.Get(gr)->ignoreFriendly);
}

void generation3d_tracking() {
    ERig r;
    RefId ref = r.Make();
    uint32_t g = r.eng.Generation3D(ref);
    CHECK(g >= 1);
    CHECK(r.eng.Generation3D(ref) == g);   // stable while the scene object is the same
    r.be.Rebuild3D(ref);
    CHECK(r.eng.Generation3D(ref) == g + 1);
    CHECK(r.eng.Generation3D(ref) == g + 1);
    r.be.Lose3D(ref, 3);
    CHECK(!r.eng.Has3D(ref));
    r.Settle(4);
    CHECK(r.eng.Has3D(ref));
    CHECK(r.eng.Generation3D(ref) == g + 2);   // lost and regained counts as a rebuild even if the node were reused
    CHECK(r.eng.Generation3D(kPlayerRef) == 0);
    CHECK(!r.eng.Has3D(0xFFFF));
}

void stance_and_weapon() {
    ERig r;
    RefId ref = r.Make();
    CHECK(r.eng.SetSneak(ref, true));
    CHECK(LastLine(r.be) == "SetForceSneak 1");
    CHECK(r.eng.SetSneak(ref, false));
    CHECK(LastLine(r.be) == "SetForceSneak 0");
    CHECK(r.eng.SetAlert(ref, true));
    CHECK(LastLine(r.be) == "SetAlert 1");
    r.eng.SetAlert(ref, false);
    // weapon draw = SetAlert; IsWeaponDrawn reports the engine's own answer, not the request
    CHECK(!r.eng.IsWeaponDrawn(ref));
    CHECK(r.eng.SetWeaponDrawn(ref, true));
    CHECK(LastLine(r.be) == "SetAlert 1");
    CHECK(!r.eng.IsWeaponDrawn(ref));   // alerted but empty handed: nothing is out
    r.be.Get(ref)->inv[kGun] = 1;
    CHECK(r.eng.EquipItem(ref, kGun));
    CHECK(r.eng.IsWeaponDrawn(ref));
    r.be.alertDrawsWeapon = false;      // the engine does not do what we hoped
    CHECK(!r.eng.IsWeaponDrawn(ref));
    r.be.alertDrawsWeapon = true;
    r.be.evalBroken = {Cond::IsWeaponOut};
    CHECK(r.eng.IsWeaponDrawn(ref));    // falls back to the process read
    CHECK(r.eng.SetWeaponDrawn(ref, false));
    CHECK(LastLine(r.be) == "SetAlert 0");
}

void equipment_calls() {
    ERig r;
    RefId ref = r.Make();
    CHECK(!r.eng.AddItem(ref, kArmor, 0));
    CHECK(!r.eng.AddItem(ref, 0, 1));
    CHECK(r.eng.ItemCount(ref, kArmor) == 0);
    CHECK(r.eng.AddItem(ref, kArmor, 1));
    CHECK(LastLine(r.be) == "AddItem 00002000 1 1");
    CHECK(r.eng.ItemCount(ref, kArmor) == 1);
    CHECK(r.eng.EquipItem(ref, kArmor));
    CHECK(LastLine(r.be) == "EquipItem 00002000 1 1");
    std::vector<FormId> worn;
    CHECK(r.eng.GetEquipped(ref, worn));
    CHECK(worn.size() == 1 && worn[0] == kArmor);
    CHECK(r.eng.UnequipItem(ref, kArmor));
    CHECK(LastLine(r.be) == "UnequipItem 00002000 0 1");
    CHECK(r.eng.GetEquipped(ref, worn) && worn.empty());
    CHECK(r.eng.RemoveItem(ref, kArmor, 1));
    CHECK(LastLine(r.be) == "RemoveItem 00002000 1 1");
    CHECK(r.eng.ItemCount(ref, kArmor) == 0);
    r.be.equippedReadable = false;
    CHECK(!r.eng.GetEquipped(ref, worn));   // unknown, not "nothing worn"
    CHECK(r.be.badLines.empty());
}

void itemcount_fallback_when_evaluator_unavailable() {
    ERig r;
    RefId ref = r.Make();
    r.be.evalBroken = {Cond::GetItemCount};
    CHECK(r.eng.ItemCount(ref, kArmor) == 0);
    CHECK(r.eng.AddItem(ref, kArmor, 2));
    CHECK(r.eng.ItemCount(ref, kArmor) == 2);   // what we added ourselves
    CHECK(r.eng.RemoveItem(ref, kArmor, 1));
    CHECK(r.eng.ItemCount(ref, kArmor) == 1);
    r.be.evalBroken.clear();
    CHECK(r.eng.ItemCount(ref, kArmor) == 1);   // the engine's own answer wins when it can answer (fake: 2 added, 1 removed)
    r.be.Get(ref)->inv[kHat] = 3;               // something the engine gave it that we never added
    CHECK(r.eng.ItemCount(ref, kHat) == 3);
}

void appearance_is_honest() {
    ERig r;
    RefId ref = r.Make();
    Appearance a;
    a.race = 1;
    CHECK(r.eng.ApplyAppearance(ref, a));          // race and sex are the template's job: exact
    a.hair = 0x1234;
    size_t n = r.be.lines.size();
    CHECK(!r.eng.ApplyAppearance(ref, a));         // face / hair cannot be set from the console: reported as approximate
    CHECK(r.eng.stats().appearanceApproximate == 1);
    CHECK(r.be.lines.size() == n);                 // and no line was sent for it
    CHECK(!r.eng.ApplyAppearance(0xDEAD, a));
}

void find_tagged_scans_every_loaded_cell() {
    ERig r;
    RefId a = r.be.AddActor(kTmpl, kExtA, Vec3{1, 1, 0});
    RefId b = r.be.AddActor(kTmplF, kExtB, Vec3{2, 2, 0});
    RefId c = r.be.AddActor(kTmpl, kInt, Vec3{3, 3, 0});
    RefId bystander = r.be.AddActor(kUntagged, kExtA, Vec3{4, 4, 0});
    RefId dead = r.be.AddActor(kTmpl, kExtA, Vec3{5, 5, 0});
    r.be.Get(dead)->deleted = true;
    std::vector<RefId> got = r.eng.FindTaggedPuppets();
    CHECK(got.size() == 3);
    CHECK(std::find(got.begin(), got.end(), a) != got.end());
    CHECK(std::find(got.begin(), got.end(), b) != got.end());
    CHECK(std::find(got.begin(), got.end(), c) != got.end());
    CHECK(std::find(got.begin(), got.end(), bystander) == got.end());
    CHECK(std::find(got.begin(), got.end(), dead) == got.end());
    CHECK(std::is_sorted(got.begin(), got.end()));
    // the player is never a puppet, even when its base form is a template
    r.eng.SetTaggedBases({kTmpl, kTmplF, 7});
    got = r.eng.FindTaggedPuppets();
    CHECK(std::find(got.begin(), got.end(), kPlayerRef) == got.end());
    CHECK(got.size() == 3);
}

void world_state_mapping() {
    ERig r;
    CHECK(r.eng.WorldReady());
    r.be.Player().cell = 0;
    CHECK(!r.eng.WorldReady());
    r.be.Player().cell = kExtA;
    r.be.Player().node = 0;
    CHECK(!r.eng.WorldReady());
    r.be.Player().node = 0x9000;
    CHECK(r.eng.WorldReady());
    CHECK(!r.eng.MenuOpen());
    r.be.menuOpen = true;
    CHECK(r.eng.MenuOpen());
    uint32_t e = r.eng.WorldEpoch();
    r.be.Load();
    CHECK(r.eng.WorldEpoch() == e + 1);
    CHECK(r.eng.CellLoaded(kExtA) && r.eng.CellLoaded(kInt));
    CHECK(!r.eng.CellLoaded(0) && !r.eng.CellLoaded(0x7777));
    r.be.SetCell(kExtB, false, false, FakeGameBackend::kWorld);
    CHECK(!r.eng.CellLoaded(kExtB));
    r.be.SetCell(kExtB, true, false, FakeGameBackend::kWorld);
    CHECK(r.eng.SameSpace(kExtA, kExtB));
    CHECK(r.eng.SameSpace(kExtA, kExtA));
    CHECK(!r.eng.SameSpace(kExtA, kInt));
    CHECK(!r.eng.SameSpace(kInt, FakeGameBackend::kOtherInterior));
    CHECK(!r.eng.SameSpace(kExtA, FakeGameBackend::kOtherWorldExt));   // another worldspace: SetPos cannot move a ref there
    CHECK(!r.eng.SameSpace(kExtA, 0x4444));
}

void move_to_cell_uses_an_anchor() {
    ERig r;
    RefId ref = r.Make();
    size_t n = r.be.lines.size();
    CHECK(r.eng.MoveToCell(ref, kInt, Vec3{100, 100, 0}, 0));
    CHECK(r.be.lines.size() == n + 1 && LastLine(r.be) == "MoveToMarker 00000502 0 0 0" && r.be.lines.back().ctx == ref);
    CHECK(r.be.Get(ref)->cell == kInt);
    CHECK(!r.eng.Has3D(ref));   // the move rebuilds the 3D
    r.Settle();
    // into the player's own cell: anchor on the player
    CHECK(r.eng.MoveToCell(ref, kExtA, Vec3{}, 0));
    CHECK(LastLine(r.be) == "MoveToMarker 00000014 0 0 0");
    // a cell that is not attached, or has nothing to anchor on, is refused without a line
    r.be.SetCell(kExtB, false, false, FakeGameBackend::kWorld);
    n = r.be.lines.size();
    CHECK(!r.eng.MoveToCell(ref, kExtB, Vec3{}, 0));
    r.be.SetCell(0x999, true, true, 0);
    CHECK(!r.eng.MoveToCell(ref, 0x999, Vec3{}, 0));
    CHECK(r.be.lines.size() == n);
}

void get_pose_reports_the_real_place() {
    ERig r;
    RefId ref = r.Make(Vec3{10, 20, 30});
    r.be.Get(ref)->pos = Vec3{10, 20, 30};
    CellId c = 0;
    Vec3 p;
    CHECK(r.eng.GetPose(ref, c, p));
    CHECK(c == kExtA && p.x == 10 && p.y == 20 && p.z == 30);
    r.be.Push(ref, Vec3{99, 98, 97});
    CHECK(r.eng.GetPose(ref, c, p) && p.x == 99);
    CHECK(!r.eng.GetPose(0xBEEF, c, p));
    CHECK(!r.eng.GetPose(kPlayerRef, c, p));
}

void get_local_pose_reads_the_player() {
    ERig r;
    r.be.Player().pos = Vec3{120, 340, 15};
    CellId c = 0;
    Vec3 p;
    CHECK(r.eng.GetLocalPose(c, p));
    CHECK(c == kExtA && p.x == 120 && p.y == 340 && p.z == 15);
    r.be.Player().pos.x = NAN;
    CHECK(!r.eng.GetLocalPose(c, p));      // an unreadable place is "unknown", never a place
    r.be.Player().pos.x = 120;
    r.be.Player().cell = 0;
    CHECK(!r.eng.GetLocalPose(c, p));      // no cell (loading): unknown
}

void console_cost_accounting() {
    ERig r;
    r.be.lineCostUs = 40;
    RefId ref = r.Make();
    for (int i = 0; i < 20; ++i) r.eng.SetYaw(ref, 0.1f * static_cast<float>(i + 1) + 0.5f);
    r.be.lineCostUs = 400;   // a slow minority
    r.eng.SetYaw(ref, 3.0f);
    r.eng.SetYaw(ref, -3.0f);
    EngineStats s = r.eng.stats();
    CHECK(s.costP50Us == 40);
    CHECK(s.costMaxUs == 400);
    CHECK(s.costP95Us >= 40 && s.costP95Us <= 400);
    std::string js = r.eng.StatsJson();
    nvcoop::json::Value v;
    std::string err;
    CHECK(nvcoop::json::parse(js, v, err));
    CHECK(v.get("costMaxUs").asInt() == 400);
    CHECK(v.get("lines").asInt() == static_cast<int64_t>(s.lines));
}

void console_exceptions_and_gone_refs() {
    ERig r;
    RefId ref = r.Make();
    r.be.throwOnPrefix = "SetPos";
    CHECK(!r.eng.SetPos(ref, Vec3{999, 999, 999}));
    CHECK(r.eng.stats().linesException >= 1);
    r.be.throwOnPrefix.clear();
    CHECK(r.eng.SetPos(ref, Vec3{999, 999, 999}));
    // the ref disappears between the guard and the line: the backend reports -2, counted, never a crash
    r.be.Kill(ref);
    CHECK(!r.eng.SetPos(ref, Vec3{1, 1, 1}));
    CHECK(!r.eng.IsValid(ref));
}

// ======================================================================================================================
// full stack
// ======================================================================================================================

void stack_spawn_and_drive() {
    SRig r;
    r.Add(1, MkPose(kExtA, 400, 200, 0, 0.5f));
    CHECK(r.UntilDriving(1));
    r.Run(500);
    auto* f = r.F(1);
    CHECK(f->exists && f->base == kTmpl);
    CHECK_NEAR(f->pos.x, 400, 0.5);
    CHECK_NEAR(f->pos.y, 200, 0.5);
    CHECK_NEAR(f->rot.z, 0.5, 0.01);
    CHECK(f->restrained && !f->ghost && f->ignoreFriendly);
    CHECK(f->group == kGrpIdle);
    CHECK(r.pm.RefOf(1) != 0 && r.pm.SlotOf(r.pm.RefOf(1)).value_or(99) == 1);
    CHECK(r.be.badLines.empty());
    CHECK(r.eng.stats().foreignRefused == 0);
    // a female player gets the female template
    r.Add(2, MkPose(kExtA, 420, 220, 0, 0), true);
    CHECK(r.UntilDriving(2));
    CHECK(r.F(2)->base == kTmplF);
    CHECK(r.pm.stats().spawnFailures == 0);
}

void stack_running_costs_one_line_per_step() {
    SRig r;
    r.Add(1, MkPose(kExtA, 400, 200, 0, 0.0f));
    CHECK(r.UntilDriving(1));
    r.Run(500);
    // run forward along +Y at 300 u/s: 15 units per 50 ms step
    float y = 200;
    size_t lines0 = r.be.lines.size();
    unsigned steps = 0;
    for (int i = 0; i < 120; ++i) {
        y += 15;
        r.Pose(1, MkPose(kExtA, 400, y, 0, 0.0f, kLocoRunFwd, Vec3{0, 300, 0}));
        r.Step();
        ++steps;
    }
    size_t lines = r.be.lines.size() - lines0;
    CHECK(r.F(1)->group == kGrpFastForward);
    CHECK_NEAR(r.F(1)->pos.y, y, 1.0);
    CHECK_NEAR(r.F(1)->pos.x, 400, 0.01);
    // Y every step, the animation once: about one console line per step, never the three axes
    CHECK(lines <= steps + 6);
    CHECK(CountLines(r.be, r.pm.RefOf(1), "SetPos X", lines0) == 0);
    CHECK(CountLines(r.be, r.pm.RefOf(1), "SetPos Z", lines0) == 0);
    CHECK(CountLines(r.be, r.pm.RefOf(1), "SetPos Y", lines0) >= steps - 2);
    CHECK(r.be.badLines.empty());
}

void stack_equipment_converges() {
    SRig r;
    r.be.defaultOutfit = {kOutfit};
    r.Add(1, MkPose(kExtA, 400, 200, 0, 0));
    r.pm.SetEquipment(1, {EquipEntry{kArmor, false}, EquipEntry{kGun, true}});
    CHECK(r.UntilDriving(1));
    r.Run(8000);
    auto* f = r.F(1);
    CHECK(r.pm.EquipConverged(1));
    CHECK(f->worn.count(kArmor) == 1 && f->worn.count(kGun) == 1 && f->worn.count(kOutfit) == 0);
    CHECK(f->inv[kArmor] == 1 && f->inv[kGun] == 1);   // owned exactly once, not added again and again
    // the weapon is drawn only once it is held: the pose says drawn, the engine says out
    PuppetPose p = MkPose(kExtA, 400, 200, 0, 0);
    p.anim.weaponDrawn = true;
    r.Pose(1, p);
    r.Run(600);
    CHECK(r.F(1)->alert);
    CHECK(r.eng.IsWeaponDrawn(r.pm.RefOf(1)));
    CHECK(r.be.badLines.empty());
}

void stack_save_and_load_purge() {
    SRig r;
    RefId bystander = r.be.AddActor(kUntagged, kExtA, Vec3{50, 50, 0});
    r.Add(1, MkPose(kExtA, 400, 200, 0, 0));
    CHECK(r.UntilDriving(1));
    RefId first = r.pm.RefOf(1);
    // saving: the puppet is despawned (Disable + MarkForDelete), and comes back afterwards
    r.pm.OnSaveBegin();
    CHECK(r.be.Get(first)->deleted && r.be.Get(first)->disabled);
    r.Run(500);
    CHECK(r.pm.RefOf(1) == 0);
    r.pm.OnSaveEnd();
    CHECK(r.UntilDriving(1));
    RefId second = r.pm.RefOf(1);
    CHECK(second != first && second != 0);
    // loading: an orphan from an old save sits in a neighbouring cell, the old puppet ref is forgotten, both are purged
    RefId orphan = r.be.AddActor(kTmpl, kExtB, Vec3{4200, 0, 0});
    r.be.Load();
    r.pm.OnLoadBegin();
    r.Run(100);
    r.pm.OnLoadEnd();
    r.Run(200);
    CHECK(r.be.Get(orphan)->deleted && r.be.Get(orphan)->disabled);
    CHECK(r.be.Get(second)->deleted);   // the forgotten puppet was found by its template base
    CHECK(r.UntilDriving(1, 200));
    CHECK(r.pm.RefOf(1) != second);
    CHECK(r.be.linesTo(bystander) == 0);   // a foreign actor is never touched
    CHECK(r.be.Get(bystander)->exists && !r.be.Get(bystander)->deleted);
}

void stack_cell_change() {
    SRig r;
    r.Add(1, MkPose(kExtA, 400, 200, 0, 0));
    CHECK(r.UntilDriving(1));
    // another exterior cell of the same worldspace: SetPos only
    r.Pose(1, MkPose(kExtB, 4300, 100, 0, 0));
    r.Run(500);
    CHECK(CountLines(r.be, r.pm.RefOf(1), "MoveToMarker") == 0);
    // into an interior: MoveToMarker on the interior anchor, then the exact place once the 3D is back
    r.Pose(1, MkPose(kInt, 120, 130, 0, 1.0f));
    r.Run(1500);
    auto* f = r.F(1);
    CHECK(CountLines(r.be, r.pm.RefOf(1), "MoveToMarker 00000502") == 1);
    CHECK(f->cell == kInt);
    CHECK_NEAR(f->pos.x, 120, 0.5);
    CHECK_NEAR(f->pos.y, 130, 0.5);
    CHECK(f->restrained);
    CHECK(r.pm.PhaseOf(1) == Phase::Driving);
    CHECK(r.be.badLines.empty());
}

void stack_rebuilt_3d_is_reasserted() {
    SRig r;
    r.Add(1, MkPose(kExtA, 400, 200, 0, 0));
    CHECK(r.UntilDriving(1));
    r.Run(500);
    RefId ref = r.pm.RefOf(1);
    r.be.Rebuild3D(ref);   // restraint and animation gone, no gap
    r.Run(600);
    CHECK(r.F(1)->restrained);
    CHECK(r.F(1)->group == kGrpIdle);
    // the same with every evaluator unavailable (the fallback must not claim the restraint survived)
    r.be.evalBroken = {Cond::GetRestrained, Cond::IsWeaponOut, Cond::GetItemCount, Cond::GetEquipped, Cond::IsSneaking, Cond::GetDead};
    r.be.Rebuild3D(ref);
    r.Run(800);
    CHECK(r.F(1)->restrained);
    CHECK(r.pm.PhaseOf(1) == Phase::Driving);
}

void stack_menu_pauses_and_replays() {
    SRig r;
    r.Add(1, MkPose(kExtA, 400, 200, 0, 0, kLocoRunFwd, Vec3{0, 300, 0}));
    CHECK(r.UntilDriving(1));
    r.Run(600);
    r.be.menuOpen = true;
    size_t n = r.be.lines.size();
    r.Run(1000);
    CHECK(r.be.lines.size() == n);   // nothing is written while a menu is up
    r.be.menuOpen = false;
    r.be.Get(r.pm.RefOf(1))->group = -1;   // closing the menu reset the animation
    r.Run(300);
    CHECK(r.pm.stats().menuReasserts >= 1);
    CHECK(r.F(1)->group == kGrpFastForward);
}

void stack_survives_all_evaluators_missing() {
    SRig r;
    r.be.evalBroken = {Cond::GetRestrained, Cond::IsWeaponOut, Cond::GetItemCount, Cond::GetEquipped, Cond::IsSneaking, Cond::GetDead};
    r.be.equippedReadable = false;
    r.Add(1, MkPose(kExtA, 400, 200, 0, 0));
    r.pm.SetEquipment(1, {EquipEntry{kArmor, false}});
    CHECK(r.UntilDriving(1));
    r.Run(3000);
    CHECK(r.pm.stats().restrainReasserts == 0);   // the fallback says "restrained" because we asked for it
    CHECK(r.pm.PhaseOf(1) == Phase::Driving);
    CHECK(r.eng.stats().evalFailures > 0);
    CHECK(r.be.badLines.empty());
}

void stack_reused_ref_ids() {
    SRig r;
    r.be.reuseIds = true;
    r.Add(1, MkPose(kExtA, 400, 200, 0, 0));
    CHECK(r.UntilDriving(1));
    RefId first = r.pm.RefOf(1);
    r.pm.RemovePlayer(1);   // deleted: its id goes back to the pool
    r.Add(2, MkPose(kExtA, 420, 220, 0, 0));
    CHECK(r.UntilDriving(2));
    CHECK(r.pm.RefOf(2) == first);   // the very same id, now a new object
    CHECK(r.eng.Owns(first));
    CHECK(r.F(2)->exists && !r.F(2)->deleted);
    r.Run(500);
    CHECK_NEAR(r.F(2)->pos.x, 420, 0.5);
}

// The remote player stands exactly where the local player does (two people on one spot): through the real engine the puppet is put one
// metre beside the real player, never inside it, so the engine has nothing to push apart.
void stack_makes_room_for_the_local_player() {
    SRig r;
    r.be.Player().pos = Vec3{400, 200, 0};
    r.Add(1, MkPose(kExtA, 400, 200, 0, 0.0f));
    CHECK(r.UntilDriving(1));
    r.Run(500);
    CHECK_NEAR(std::hypot(r.F(1)->pos.x - 400, r.F(1)->pos.y - 200), 70.0, 1.0);
    CHECK(r.pm.stats().bodyYields > 0);
    r.Pose(1, MkPose(kExtA, 900, 200, 0, 0.0f));   // the remote walks off: its true place again
    r.Run(500);
    CHECK_NEAR(r.F(1)->pos.x, 900, 0.5);
    CHECK_NEAR(r.F(1)->pos.y, 200, 0.5);
}

// ======================================================================================================================
// local player
// ======================================================================================================================

struct LRig {
    FakeGameBackend be;
    LocalSampler s;
    uint32_t now = 1000;
    std::vector<live::ActorSnapshot> out;
    LRig() {
        be.Player().pos = Vec3{1000, 1000, 0};
        s.SetIdentity(0x77, 3);
    }
    // 60 fps frames; `move` is called each frame with dt in seconds
    void Run(uint32_t ms, const std::function<void(float)>& move = nullptr) {
        for (uint32_t t = 0; t < ms; t += 16) {
            if (move) move(0.016f);
            live::ActorSnapshot snap;
            if (s.Sample(be, now, snap)) out.push_back(snap);
            now += 16;
        }
    }
};

void sampler_period_identity_and_sequence() {
    LRig r;
    r.Run(2000);
    CHECK(r.out.size() >= 38 && r.out.size() <= 41);   // 20 Hz
    CHECK(r.out.front().netId == 0x77 && r.out.front().ownerEpoch == 3 && r.out.front().seq == 1);
    bool rising = true, ticks = true;
    for (size_t i = 1; i < r.out.size(); ++i) {
        if (r.out[i].seq != r.out[i - 1].seq + 1) rising = false;
        int32_t dt = static_cast<int32_t>(r.out[i].tick - r.out[i - 1].tick);
        if (dt < 48 || dt > 70) ticks = false;
    }
    CHECK(rising);
    CHECK(ticks);
    CHECK(r.out.back().cell == kExtA);
    r.s.SetIdentity(0x77, 4);   // a new ownership epoch restarts the per-(actor, epoch) sequence
    size_t n = r.out.size();
    r.Run(300);
    CHECK(r.out.size() > n + 3);
    CHECK(r.out[n].seq == 1 && r.out[n].ownerEpoch == 4 && r.out[n + 1].seq == 2);
}

void sampler_velocity_and_locomotion() {
    LRig r;
    // run forward along +Y at 300 u/s, facing +Y
    r.be.Player().rot.z = 0;
    r.Run(1500, [&](float dt) { r.be.Player().pos.y += 300 * dt; });
    CHECK(r.out.back().anim.locomotion == kLocoRunFwd);
    CHECK_NEAR(r.out.back().vel.y, 300, 12);
    CHECK_NEAR(r.out.back().vel.x, 0, 1);
    // walk at 110 u/s
    r.Run(1500, [&](float dt) { r.be.Player().pos.y += 110 * dt; });
    CHECK(r.out.back().anim.locomotion == kLocoWalkFwd);
    // strafe right at run speed (moving +X while facing +Y)
    r.Run(1500, [&](float dt) { r.be.Player().pos.x += 300 * dt; });
    CHECK(r.out.back().anim.locomotion == kLocoRunRight);
    // backwards
    r.Run(1500, [&](float dt) { r.be.Player().pos.y -= 300 * dt; });
    CHECK(r.out.back().anim.locomotion == kLocoRunBack);
    // standing
    r.Run(1500);
    CHECK(r.out.back().anim.locomotion == kLocoIdle);
    CHECK(std::fabs(r.out.back().vel.y) < 3);
    // turning to face +X and running +X is forward again; yaw 90 degrees clockwise = REFR rotZ pi/2
    r.be.Player().rot.z = 1.5707963f;
    r.Run(1500, [&](float dt) { r.be.Player().pos.x += 300 * dt; });
    CHECK(r.out.back().anim.locomotion == kLocoRunFwd);
    CHECK_NEAR(r.out.back().rot.y, 1.5707963, 0.001);   // snapshot rot = (pitch, yaw, roll)
}

void sampler_teleport_and_cell_change_restart_the_speed() {
    LRig r;
    r.Run(1500, [&](float dt) { r.be.Player().pos.y += 300 * dt; });
    CHECK_NEAR(r.out.back().vel.y, 300, 15);
    r.be.Player().pos.y += 2000;   // 28 m in one frame
    r.Run(100);
    CHECK(r.s.counters().teleports == 1);
    for (const auto& snap : r.out) CHECK(std::fabs(snap.vel.y) < 400);   // never the 40000 u/s a naive difference gives
    // a new cell: the old position means nothing
    LRig q;
    q.Run(500);
    q.be.Player().cell = kInt;
    q.be.Player().pos = Vec3{1010, 1004, 0};   // a different cell, and only a few units from where the old one put us
    q.Run(200);
    CHECK(q.out.back().cell == kInt);
    CHECK(std::fabs(q.out.back().vel.x) < 1 && std::fabs(q.out.back().vel.y) < 1);
    CHECK(q.s.counters().cellChanges == 1);
}

void sampler_speed_is_clamped() {
    // 90 units in 33 ms (2700 u/s) is under the 100 unit floor of the teleport rule, so it is a speed, and never an unbounded one on the wire
    LocalSamplerConfig cfg;
    cfg.periodMs = 33;
    LocalSampler fast(cfg);
    fast.SetIdentity(0x77, 3);
    LocalFrame f;
    f.valid = true;
    f.cell = 0x10;
    f.pos = Vec3{1000, 1000, 0};
    live::ActorSnapshot snap, lastSnap;
    uint32_t t = 1000;
    for (int i = 0; i < 60; ++i) {
        f.pos.y += 90;
        t += 33;
        if (fast.Update(t, f, snap)) {
            lastSnap = snap;
            CHECK(std::sqrt(snap.vel.x * snap.vel.x + snap.vel.y * snap.vel.y) <= 2200.5f);
        }
    }
    CHECK(fast.counters().teleports == 0);
    CHECK(lastSnap.vel.y > 1500);
    // 3000 u/s on the 50 ms period is faster than anything that falls or runs (maxSpeed 2200) and over 100 units a period: a teleport each period, restarted
    LRig q;
    q.Run(1500, [&](float dt) { q.be.Player().pos.y += 3000 * dt; });
    CHECK(q.s.counters().teleports > 0);
    for (const auto& snap : q.out) CHECK(std::sqrt(snap.vel.x * snap.vel.x + snap.vel.y * snap.vel.y) <= 2200.5f);
}

// A jump of 3 m in one period is no speed (the first version only called more than 350 units a teleport, so it went on the wire as 2200 u/s and the other copy
// dead-reckoned it 330 units past the target: lab walk room, spawn room)
void sampler_small_jump_is_a_teleport_not_a_speed() {
    LRig r;
    r.Run(1500, [&](float dt) { r.be.Player().pos.y += 300 * dt; });
    r.be.Player().pos.y += 200;
    r.Run(300);
    CHECK(r.s.counters().teleports == 1);
    for (size_t i = r.out.size() > 8 ? r.out.size() - 8 : 0; i < r.out.size(); ++i) CHECK(std::fabs(r.out[i].vel.y) < 400);
    // the same jump spread over a long period (a hitch of 0.5 s) is a speed of 400 u/s: no teleport
    LRig q;
    q.Run(1500, [&](float dt) { q.be.Player().pos.y += 300 * dt; });
    q.now += 450;
    q.be.Player().pos.y += 135;
    q.Run(100);
    CHECK(q.s.counters().teleports == 0);
}

void sampler_flags_from_the_engine() {
    LRig r;
    auto& p = r.be.Player();
    p.rot = Vec3{0.3f, 1.2f, 0.1f};   // rotX pitch, rotY roll, rotZ heading
    r.Run(300);
    CHECK_NEAR(r.out.back().rot.x, 0.3, 0.001);
    CHECK_NEAR(r.out.back().rot.y, 0.1 * 0 + 0.1, 0.001);   // rot.y is the heading = rotZ = 0.1
    CHECK_NEAR(r.out.back().rot.z, 1.2, 0.001);             // rot.z is the roll = rotY
    CHECK(!r.out.back().anim.weaponDrawn && !r.out.back().anim.sneak && !r.out.back().anim.aim && !r.out.back().anim.jump && !r.out.back().anim.reload);
    p.weaponOutForced = true;
    p.aiming = true;
    p.sneak = true;
    r.Run(200);
    CHECK(r.out.back().anim.weaponDrawn && r.out.back().anim.aim && r.out.back().anim.sneak);
    p.group = 177;   // ReloadA
    r.Run(200);
    CHECK(r.out.back().anim.reload && !r.out.back().anim.jump);
    p.group = 228;   // JumpLoop
    r.Run(200);
    CHECK(r.out.back().anim.jump && !r.out.back().anim.reload);
    p.group = 7;     // FastForward
    r.Run(200);
    CHECK(!r.out.back().anim.jump && !r.out.back().anim.reload);
    // sneak from the mover when the evaluator is unavailable
    p.sneak = false;
    p.moverFlags = 1u << 9;
    p.haveMover = true;
    r.be.evalBroken = {Cond::IsSneaking, Cond::IsWeaponOut};
    p.weaponOutForced = true;
    r.Run(200);
    CHECK(r.out.back().anim.sneak);
    CHECK(r.out.back().anim.weaponDrawn);   // falls back to the process read
}

void sampler_mover_flags_only_when_informative() {
    LRig r;
    auto& p = r.be.Player();
    p.haveMover = true;
    // flags that say nothing about walk / run (only the sneak bit): speed decides
    p.moverFlags = 1u << 9;
    r.Run(1500, [&](float dt) { p.pos.y += 300 * dt; });
    CHECK(r.out.back().anim.locomotion == kLocoRunFwd);
    // informative flags: walk bit set while moving at 150 u/s -> walking
    p.moverFlags = 1u << 7;
    r.Run(1500, [&](float dt) { p.pos.y += 150 * dt; });
    CHECK(r.out.back().anim.locomotion == kLocoWalkFwd);
    // run bit
    p.moverFlags = 1u << 8;
    r.Run(1500, [&](float dt) { p.pos.y += 150 * dt; });
    CHECK(r.out.back().anim.locomotion == kLocoRunFwd);
}

void sampler_not_ready_publishes_nothing() {
    LRig r;
    r.Run(300);
    size_t n = r.out.size();
    r.be.Player().cell = 0;   // loading screen
    r.Run(500);
    CHECK(r.out.size() == n);
    CHECK(r.s.counters().notReady > 0);
    r.be.Player().cell = kExtA;
    r.be.Player().pos.x = NAN;
    r.Run(300);
    CHECK(r.out.size() == n);
    r.be.Player().pos.x = 1000;
    r.Run(300);
    CHECK(r.out.size() > n);
    CHECK(std::fabs(r.out.back().vel.x) < 1);   // the first sample after a gap has no speed
    for (const auto& s2 : r.out) CHECK(std::isfinite(s2.pos.x) && std::isfinite(s2.vel.y));
}

// ---- the player's one-shots (attacks, block, stagger) in the snapshot ------------------------------------------------------------
// The sampler reads the action group that plays on the player's animation slots and publishes it the way the NPC layer's sampler does: the group, held for
// a while, and a counter that bumps when one STARTS. Twins 87 to 95 (local_sampler.cpp).

// The player plays `group` for `ms` (the sequence's play time advances with the frames), then it ends. `slot` is the AnimData slot it sits in.
static void PlayerPlays(LRig& r, int group, uint32_t ms, int slot = 1) {
    auto& p = r.be.Player();
    p.group = group;
    p.groupSlot = slot;
    p.groupState = 1;
    p.groupPlayMs = 0;
    r.Run(ms, [&](float) { p.groupPlayMs += 16; });
    p.group = -1;
    p.groupPlayMs = -1;
}

static size_t SnapsWithAction(const LRig& r, int group) {
    size_t n = 0;
    for (const auto& s : r.out)
        if (s.anim.action == group) ++n;
    return n;
}

void sampler_attack_reaches_the_snapshot() {
    LRig r;
    r.Run(500);
    CHECK(r.out.back().anim.action == 0 && r.out.back().anim.actionSeq == 0 && !r.out.back().anim.block);
    PlayerPlays(r, 32, 400);                    // a swing
    r.Run(1200);
    CHECK(SnapsWithAction(r, 32) >= 8);         // in the stream for as long as it played plus the hold (600 ms)
    CHECK(r.out.back().anim.action == 0 && r.out.back().anim.actionSeq == 1);   // gone afterwards, the counter stays
    for (const auto& s : r.out) CHECK(s.anim.actionSeq <= 1);                   // one event, however many samples saw the same sequence
    // a swing that the engine shows for 60 ms only still reaches the stream, several snapshots long (a lost datagram or a slow frame cannot skip it)
    LRig q;
    q.Run(500);
    PlayerPlays(q, 92, 60);
    q.Run(1000);
    CHECK(SnapsWithAction(q, 92) >= 6);
    CHECK(q.out.back().anim.actionSeq == 1);
}


// A human plays in FIRST PERSON: his swings and shots run on the first-person AnimData (PlayerCharacter+0x690), not on the process's one the old sampler read
// ("I don't see the other person punching"). Twin 96 (local_sampler.cpp) never reads that set.
void sampler_first_person_attack_reaches_the_snapshot() {
    LRig r;
    r.be.Player().firstPerson = true;
    r.be.Player().fpSet = true;
    r.Run(500);
    CHECK(r.out.back().anim.action == 0 && r.out.back().anim.actionSeq == 0);
    auto& p = r.be.Player();
    p.fpGroup = 26;                             // the punch plays on the first-person set only; the third-person set shows nothing
    p.fpState = 1;
    p.fpPlayMs = 0;
    r.Run(400, [&](float) { p.fpPlayMs += 16; });
    p.fpGroup = -1;
    p.fpPlayMs = -1;
    r.Run(1200);
    CHECK(SnapsWithAction(r, 26) >= 8);
    CHECK(r.out.back().anim.action == 0 && r.out.back().anim.actionSeq == 1);
    const LocalSampler::ActionEvent& e = r.s.lastActionEvent();
    CHECK(e.group == 26 && e.set == kSetFirstPerson && e.firstPerson && e.setCount == 2 && r.s.counters().actionEvents == 1);
    // two punches in a row on the first-person set are two events (the play time runs backwards)
    LRig q;
    q.be.Player().firstPerson = true;
    q.be.Player().fpSet = true;
    auto& pq = q.be.Player();
    q.Run(300);
    pq.fpGroup = 26; pq.fpState = 1; pq.fpPlayMs = 0;
    q.Run(400, [&](float) { pq.fpPlayMs += 16; });
    pq.fpPlayMs = 0;
    q.Run(400, [&](float) { pq.fpPlayMs += 16; });
    CHECK(q.out.back().anim.actionSeq == 2);
    // a player in third person with the first-person set present but idle: the process's set still carries the attack
    LRig w;
    w.be.Player().fpSet = true;
    w.Run(300);
    auto& pw = w.be.Player();
    pw.group = 32; pw.groupSlot = 1; pw.groupState = 1; pw.groupPlayMs = 0;
    w.Run(300, [&](float) { pw.groupPlayMs += 16; });
    CHECK(w.out.back().anim.action == 32 && w.out.back().anim.actionSeq == 1);
    // a leftover (state 0) in the first-person set is no action
    LRig v;
    v.be.Player().firstPerson = true;
    auto& pv = v.be.Player();
    pv.fpGroup = 26; pv.fpState = 0; pv.fpPlayMs = 100;
    v.Run(600);
    CHECK(v.out.back().anim.action == 0 && v.out.back().anim.actionSeq == 0);
}

void sampler_every_start_is_an_event() {
    LRig r;
    r.Run(300);
    PlayerPlays(r, 26, 400);                    // a punch ...
    PlayerPlays(r, 32, 400);                    // ... another group at once ...
    r.Run(1000);
    CHECK(r.out.back().anim.actionSeq == 2);
    // ... and the SAME group restarted without a gap in between: its play time runs backwards. Two punches in a row are two events.
    LRig q;
    q.Run(300);
    auto& p = q.be.Player();
    p.group = 26;
    p.groupState = 1;
    p.groupPlayMs = 0;
    q.Run(400, [&](float) { p.groupPlayMs += 16; });
    p.groupPlayMs = 0;
    q.Run(400, [&](float) { p.groupPlayMs += 16; });
    CHECK(q.out.back().anim.actionSeq == 2);
    // a sequence the backend cannot time (-1) cannot be seen restarting: one event, and not a new one on every sample
    LRig w;
    w.Run(300);
    auto& pw = w.be.Player();
    pw.group = 26;
    pw.groupState = 1;
    pw.groupPlayMs = -1;
    w.Run(1000);
    CHECK(w.out.back().anim.actionSeq == 1);
    // one long sequence is one event
    LRig e;
    e.Run(300);
    PlayerPlays(e, 38, 1500);
    CHECK(e.out.back().anim.actionSeq == 1);
}

void sampler_block_is_held_and_released() {
    LRig r;
    r.Run(300);
    auto& p = r.be.Player();
    p.group = 170;                              // BlockIdle
    p.groupState = 1;
    p.groupPlayMs = 0;
    r.Run(1500, [&](float) { p.groupPlayMs += 16; });
    CHECK(r.out.back().anim.action == 170 && r.out.back().anim.block && r.out.back().anim.actionSeq == 1);
    p.group = -1;
    r.Run(300);                                 // released: the stream says so within about 150 ms, not after a swing's 600 ms
    CHECK(r.out.back().anim.action == 0 && !r.out.back().anim.block);
    CHECK(r.out.back().anim.actionSeq == 1);
    // a hit taken while blocking is a reaction, not a block
    PlayerPlays(r, 171, 300);
    CHECK(r.out.back().anim.action == 171 && !r.out.back().anim.block);
}

void sampler_ignores_dead_players_and_leftover_sequences() {
    LRig r;
    r.Run(300);
    PlayerPlays(r, 32, 300);
    CHECK(r.out.back().anim.actionSeq == 1);
    auto& p = r.be.Player();
    p.dead = true;
    p.group = 32;                               // a sequence still sitting in a slot of a dead body
    p.groupPlayMs = 0;
    r.Run(500);
    CHECK(r.out.back().anim.action == 0 && r.out.back().anim.actionSeq == 1);   // no action, the counter goes on
    p.dead = false;
    p.group = -1;
    // a finished sequence the engine left in its slot (state not animated) is no action
    LRig q;
    q.Run(300);
    auto& pq = q.be.Player();
    pq.group = 32;
    pq.groupState = 0;
    pq.groupPlayMs = 400;
    q.Run(500);
    CHECK(q.out.back().anim.action == 0 && q.out.back().anim.actionSeq == 0);
    pq.groupState = 2;                          // easing in: it has started
    q.Run(200);
    CHECK(q.out.back().anim.action == 32 && q.out.back().anim.actionSeq == 1);
    // a backend that cannot read the state (-1) is trusted
    LRig w;
    w.Run(300);
    auto& pw = w.be.Player();
    pw.group = 32;
    pw.groupState = -1;
    pw.groupPlayMs = -1;
    w.Run(200);
    CHECK(w.out.back().anim.action == 32);
    // the groups that are state, not actions, are never one
    for (int g : {7, 17, 24, 224, 225, 227, 228}) {
        LRig v;
        v.Run(300);
        auto& pv = v.be.Player();
        pv.group = g;
        pv.groupState = 1;
        pv.groupPlayMs = 100;
        v.Run(300);
        CHECK(v.out.back().anim.action == 0 && v.out.back().anim.actionSeq == 0);
    }
}

void sampler_action_counter_survives_loading_screens() {
    LRig r;
    r.Run(300);
    PlayerPlays(r, 32, 300);
    r.Run(800);
    CHECK(r.out.back().anim.actionSeq == 1);
    r.be.Player().cell = 0;                     // a loading screen
    r.Run(500);
    r.be.Player().cell = kExtA;
    r.Run(300);
    PlayerPlays(r, 92, 300);
    r.Run(300);
    CHECK(r.out.back().anim.actionSeq == 2);    // a copy that remembers 1 reads 2 as the next event; a counter that restarted would read 1 as a repeat
    r.s.Reset();                                // a teleport / load reset
    r.Run(200);
    PlayerPlays(r, 38, 300);
    r.Run(300);
    CHECK(r.out.back().anim.actionSeq == 3);
}

void equip_watcher_publishes_stable_changes_only() {
    FakeGameBackend be;
    SetupBackend(be);
    EquipWatcher w(100, 250);
    std::vector<EquipEntry> out;
    uint32_t now = 1000;
    auto Poll = [&](uint32_t ms) {
        bool got = false;
        for (uint32_t t = 0; t < ms; t += 50) {
            std::vector<EquipEntry> o;
            if (w.Poll(be, now, o)) {
                out = o;
                got = true;
            }
            now += 50;
        }
        return got;
    };
    // nothing worn: still published once (the remote needs to know "nothing")
    CHECK(Poll(600));
    CHECK(out.empty());
    CHECK(!Poll(600));   // and not again
    be.Player().worn = {kGun, kArmor};   // weapon first in the set: published sorted, weapon flagged
    CHECK(Poll(600));
    CHECK(out.size() == 2 && out[0].form == kArmor && !out[0].weapon && out[1].form == kGun && out[1].weapon);
    // a flicker that is gone before the stability window passes is not published (weapon swap animation)
    be.Player().worn = {kGun, kArmor, kHat};
    Poll(100);
    be.Player().worn = {kGun, kArmor};
    CHECK(!Poll(600));
    // a real change is
    be.Player().worn = {kGun2, kArmor};
    CHECK(Poll(600));
    CHECK(out.size() == 2 && out[1].form == kGun2 && out[1].weapon);
    // the engine may list the same items in another order: not a change, and always published sorted
    be.descendingEquipped = true;
    CHECK(!Poll(600));
    be.Player().worn = {kGun2, kArmor, kHat};
    CHECK(Poll(600));
    CHECK(out.size() == 3 && out[0].form == kArmor && out[1].form == kHat && out[2].form == kGun2);
    be.Player().worn = {kGun2, kArmor};
    CHECK(Poll(600));
    be.descendingEquipped = false;
    CHECK(!Poll(600));
    // unreadable equipment keeps what was published
    be.equippedReadable = false;
    be.Player().worn = {};
    CHECK(!Poll(600));
    CHECK(w.published().size() == 2);
}

void local_appearance_is_read_from_the_base() {
    FakeGameBackend be;
    Appearance a;
    a.race = 0xAB;
    a.female = true;
    a.hair = 0x55;
    a.name = "Courier";
    be.baseAppearance[7] = a;
    Appearance got;
    CHECK(be.ReadAppearance(kPlayerRef, got));
    CHECK(got.race == 0xAB && got.female && got.hair == 0x55);
}

// ======================================================================================================================
// round of 2026-10-22: body height, weapon modifications, hit-target health, look updates (twins 80-85)
// ======================================================================================================================

void appearance_height_is_matched_with_setscale() {
    ERig r;
    RefId ref = r.Make();
    Appearance a;
    a.race = 1;
    a.height = 1100;                                  // the remote player is 10 percent taller than 1.0
    size_t n = r.be.lines.size();
    CHECK(r.eng.ApplyAppearance(ref, a));             // race, sex and a height: nothing the template cannot show, so exact
    CHECK(r.be.lines.size() == n + 1 && HasLine(r.be, ref, "SetScale 1.100"));
    CHECK_NEAR(r.be.Get(ref)->scale, 1.1, 1e-4);
    CHECK(r.eng.stats().scaleApplied == 1);
    // the template NPC has its own height: the ratio is what is applied (template 1.25, remote 1.1 -> 0.88)
    Appearance t;
    t.height = 1250;
    r.be.baseAppearance[kTmpl] = t;
    a.height = 1100;
    CHECK(r.eng.ApplyAppearance(ref, a));
    CHECK_NEAR(r.be.Get(ref)->scale, 0.88, 1e-3);
    // the same height as the template: no line at all
    n = r.be.lines.size();
    a.height = 1250;
    CHECK(r.eng.ApplyAppearance(ref, a));
    CHECK(r.be.lines.size() == n);
    // nonsense is ignored: 0 = unknown, 30x is not a person
    a.height = 0;
    CHECK(r.eng.ApplyAppearance(ref, a) && r.be.lines.size() == n);
    a.height = 40000;
    CHECK(r.eng.ApplyAppearance(ref, a) && r.be.lines.size() == n);
    // the weight has no console command: reported as approximate, never faked
    a.height = 1250;
    a.weight = 900;
    CHECK(!r.eng.ApplyAppearance(ref, a));
    CHECK(r.be.badLines.empty());
    // never on a foreign actor, never on the player
    RefId foreign = r.be.AddActor(kUntagged, kExtA, Vec3{1, 1, 0});
    a.weight = 0;
    a.height = 1100;
    n = r.be.lines.size();
    CHECK(!r.eng.ApplyAppearance(foreign, a) && !r.eng.ApplyAppearance(kPlayerRef, a));
    CHECK(r.be.lines.size() == n);
}

void weapon_mod_lines_and_readback() {
    ERig r;
    RefId ref = r.Make();
    CHECK(r.eng.AddItem(ref, kGun, 1));
    CHECK(r.eng.EquipItem(ref, kGun));
    CHECK(r.eng.WeaponMods(ref, kGun) == 0);          // worn, no mods: known and zero
    CHECK(r.eng.WeaponMods(ref, kArmor) == -1);       // not worn: unknown
    CHECK(r.eng.EnableWeaponMod(ref, kGun, 2));
    CHECK(HasLine(r.be, ref, "EnableWeaponMod 00003000 2"));
    CHECK(r.eng.WeaponMods(ref, kGun) == 0x02);
    CHECK(r.eng.EnableWeaponMod(ref, kGun, 1));
    CHECK(r.eng.WeaponMods(ref, kGun) == 0x03);
    CHECK(!r.eng.EnableWeaponMod(ref, kGun, 0) && !r.eng.EnableWeaponMod(ref, kGun, 4) && !r.eng.EnableWeaponMod(ref, 0, 1));
    CHECK(r.be.count("EnableWeaponMod") == 2);        // the bad slots were refused before a line was sent
    CHECK(!r.eng.EnableWeaponMod(0xDEAD, kGun, 1));
    CHECK(r.eng.stats().modLines == 2);
    // an engine that refuses the command: false, and nothing changes
    r.be.enableModWorks = false;
    CHECK(!r.eng.EnableWeaponMod(ref, kGun, 3));
    CHECK(r.eng.WeaponMods(ref, kGun) == 0x03);
    // a backend that cannot read the extra data: unknown, not "zero"
    r.be.modsReadable = false;
    CHECK(r.eng.WeaponMods(ref, kGun) == -1);
    CHECK(r.be.badLines.empty());
}

void hit_target_health_lines() {
    ERig r;
    RefId ref = r.Make();
    CHECK(!r.eng.IsDead(ref));
    CHECK(r.eng.ResetHealth(ref));
    CHECK(HasLine(r.be, ref, "ResetHealth"));
    r.be.Hit(ref, 60);
    CHECK(!r.eng.IsDead(ref));
    CHECK(r.eng.ResetHealth(ref) && r.be.Get(ref)->health == 100);
    r.be.Hit(ref, 500);
    CHECK(r.eng.IsDead(ref));                          // the GetDead evaluator
    r.be.evalBroken.insert(Cond::GetDead);
    CHECK(r.eng.IsDead(ref));                          // the evaluator is unavailable: the process knows too
    CHECK(r.eng.stats().evalFailures >= 1);
    CHECK(!r.eng.IsDead(0xDEAD) && !r.eng.ResetHealth(0xDEAD) && !r.eng.ResetHealth(kPlayerRef));
    CHECK(r.be.badLines.empty());
}

void stack_weapon_mods_converge_and_follow_changes() {
    SRig r;
    r.Add(1, MkPose(kExtA, 400, 200, 0, 0));
    r.pm.SetEquipment(1, {EquipEntry{kArmor, false, 0}, EquipEntry{kGun, true, 0x05}});
    CHECK(r.UntilDriving(1));
    r.Run(10000);
    auto* f = r.F(1);
    CHECK(f->worn.count(kGun) == 1 && f->mods[kGun] == 0x05);
    CHECK(r.pm.EquipConverged(1));
    CHECK(r.pm.stats().modOps == 2 && r.pm.stats().modFailures == 0);
    // the owner takes a mod off / puts another on: the wanted set changed, the puppet follows (the engine fake only adds mods, so the weapon is swapped)
    r.pm.SetEquipment(1, {EquipEntry{kArmor, false, 0}, EquipEntry{kGun2, true, 0x02}});
    r.Run(10000);
    f = r.F(1);
    CHECK(f->worn.count(kGun2) == 1 && f->worn.count(kGun) == 0 && f->mods[kGun2] == 0x02);
    CHECK(r.be.badLines.empty());
    // an engine that cannot read mods: asked once per wanted slot, no loop
    SRig q;
    q.be.modsReadable = false;
    q.Add(1, MkPose(kExtA, 400, 200, 0, 0));
    q.pm.SetEquipment(1, {EquipEntry{kGun, true, 0x03}});
    CHECK(q.UntilDriving(1));
    q.Run(10000);
    CHECK(q.be.count("EnableWeaponMod") == 2);
}

void stack_hit_target_puppet_survives_and_is_replaced() {
    SRig r;
    r.Add(1, MkPose(kExtA, 400, 200, 0, 0));
    CHECK(r.UntilDriving(1));
    r.Run(1000);
    RefId first = r.pm.RefOf(1);
    r.be.Hit(first, 70);                               // an enemy hurts it
    r.Run(3000);
    CHECK(r.be.Get(first)->health == 100);             // topped up again (ResetHealth every 2 s)
    CHECK(r.pm.stats().heals >= 1);
    r.be.Hit(first, 500);                              // and one burst kills it
    r.Run(4000);
    CHECK(r.pm.stats().puppetDeaths == 1);
    RefId second = r.pm.RefOf(1);
    CHECK(second != 0 && second != first);
    CHECK(r.be.Get(first)->deleted || !r.be.Get(first)->exists);
    CHECK(!r.be.Get(second)->dead && r.be.Get(second)->health == 100);
    CHECK(r.pm.PhaseOf(1) == Phase::Driving);
    CHECK(r.be.badLines.empty());
}

void stack_look_update_keeps_the_puppet() {
    SRig r;
    r.Add(1, MkPose(kExtA, 400, 200, 0, 0));
    CHECK(r.UntilDriving(1));
    r.Run(500);
    RefId first = r.pm.RefOf(1);
    Appearance a = App();
    a.height = 1200;
    CHECK(r.pm.SetAppearance(1, a));
    r.Run(500);
    CHECK(r.pm.RefOf(1) == first);
    CHECK_NEAR(r.F(1)->scale, 1.2, 1e-4);
    a.female = true;                                   // another sex: another template, a new puppet with the height again
    CHECK(r.pm.SetAppearance(1, a));
    r.Run(3000);
    CHECK(r.pm.RefOf(1) != first && r.F(1)->base == kTmplF);
    CHECK_NEAR(r.F(1)->scale, 1.2, 1e-4);
    CHECK(r.be.badLines.empty());
}

void equip_watcher_carries_weapon_mods() {
    FakeGameBackend be;
    SetupBackend(be);
    EquipWatcher w(100, 250);
    std::vector<EquipEntry> out;
    uint32_t now = 1000;
    auto Poll = [&](uint32_t ms) {
        bool got = false;
        for (uint32_t t = 0; t < ms; t += 50) {
            std::vector<EquipEntry> o;
            if (w.Poll(be, now, o)) {
                out = o;
                got = true;
            }
            now += 50;
        }
        return got;
    };
    be.Player().worn = {kGun, kArmor};
    be.Player().mods[kGun] = 0x03;
    be.Player().mods[kArmor] = 0x07;                   // armor never carries modification flags
    CHECK(Poll(600));
    CHECK(out.size() == 2 && out[0].form == kArmor && out[0].mods == 0 && out[1].form == kGun && out[1].mods == 0x03);
    CHECK(!Poll(600));
    be.Player().mods[kGun] = 0x07;                     // a mod added to the held weapon is a change
    CHECK(Poll(600));
    CHECK(out[1].mods == 0x07);
}

// ======================================================================================================================
// the look of a remote player in the puppet's own base (LookBase) and the face block layout
// ======================================================================================================================

Appearance RemoteLook() {
    Appearance a;
    a.race = 0x2000;
    a.female = true;
    a.hair = 0x44;
    a.eyes = 0x45;
    a.hairColor = 0x00ABCDEF;
    a.height = 1050;
    a.weight = 900;
    for (int i = 0; i < 130; ++i) a.faceCoeffs.push_back(static_cast<int8_t>((i * 7) % 61 - 30));
    a.headParts = {0x70, 0x71};
    a.name = "Remote";
    return a;
}

void SetupLooks(FakeGameBackend& be) {
    be.cloneWorks = true;
    Appearance mine;   // the local player's base is the template: its look must never change
    mine.race = 0x1000;
    mine.hair = 0x10;
    mine.eyes = 0x11;
    mine.hairColor = 0x111111;
    mine.height = 1000;
    mine.faceCoeffs.assign(130, 3);
    mine.headParts = {0x60};
    mine.name = "Mine";
    be.baseAppearance[kTmpl] = mine;
}

// A remote player's puppet is built from a private copy of the template base with the remote's race, sex, hair, eyes, face and head parts written
// into it, and a changed look makes a new puppet from a new copy. The local player's own base stays what it was.
void look_copy_carries_the_remote_look() {
    SRig r;
    SetupLooks(r.be);
    std::vector<std::string> log;
    r.pm.SetLog([&](const std::string& s) { log.push_back(s); });
    r.pm.AddPlayer(1, RemoteLook());
    r.Pose(1, MkPose(kExtA, 300, 200));
    CHECK(r.UntilDriving(1));
    const FormId base = r.F(1)->base;
    CHECK(base != kTmpl && r.be.copyOf.count(base) == 1 && r.be.copyOf[base] == kTmpl);
    const Appearance w = r.be.copyLook[base];
    CHECK(w.female && w.race == 0x2000 && w.hair == 0x44 && w.eyes == 0x45 && w.hairColor == 0x00ABCDEFu && w.height == 1050 && w.weight == 900);
    CHECK(w.faceCoeffs == RemoteLook().faceCoeffs);
    CHECK(w.headParts == (std::vector<uint32_t>{0x70, 0x71}));
    CHECK(r.be.baseAppearance[kTmpl].hair == 0x10 && r.be.baseAppearance[kTmpl].headParts == (std::vector<uint32_t>{0x60}) && !r.be.baseAppearance[kTmpl].female);
    bool logged = false;
    for (const std::string& l : log)
        logged = logged || (l.find("look copy") != std::string::npos && l.find("face written") != std::string::npos && l.find("head parts written") != std::string::npos);
    CHECK(logged);
    Appearance b = RemoteLook();
    b.female = false;
    b.hair = 0x99;
    b.headParts = {0x72};
    CHECK(r.pm.SetAppearance(1, b));
    r.Run(1000);
    CHECK(r.UntilDriving(1));
    const FormId base2 = r.F(1)->base;
    CHECK(base2 != base && r.be.copyLook[base2].hair == 0x99 && !r.be.copyLook[base2].female && r.be.copyLook[base2].headParts == (std::vector<uint32_t>{0x72}));
    CHECK(r.eng.stats().lookCopies == 2 && r.eng.stats().lookWrites == 2 && r.eng.stats().lookFaces == 2);
}

// The game may refuse a copy once (a moment when the data handler is busy). The first version gave the whole session up after one refusal, and from then on
// every puppet was the template: the local player's own look.
void look_copy_refusal_is_not_forever() {
    SRig r;
    SetupLooks(r.be);
    r.be.cloneRefusals = 1;
    r.pm.AddPlayer(1, RemoteLook());
    r.Pose(1, MkPose(kExtA, 300, 200));
    CHECK(r.UntilDriving(1));
    CHECK(r.F(1)->base == kTmpl);                  // the first try was refused: the template, for now
    CHECK(r.eng.LooksByBase());                    // ... but the engine keeps trying copies
    CHECK(r.eng.stats().lookFailures == 1 && r.eng.LookReport(1).find("will try again") != std::string::npos);
    CHECK(r.pm.SetAppearance(1, RemoteLook()));
    r.Run(1000);
    CHECK(r.UntilDriving(1));
    CHECK(r.F(1)->base != kTmpl && r.be.copyOf.count(r.F(1)->base) == 1);
    // three refusals in a row: the game does not do it
    SRig q;
    SetupLooks(q.be);
    q.be.cloneRefusals = 5;
    q.pm.AddPlayer(1, RemoteLook());
    q.Pose(1, MkPose(kExtA, 300, 200));
    for (int i = 0; i < 3; ++i) {
        CHECK(q.UntilDriving(1));
        CHECK(q.pm.SetAppearance(1, RemoteLook()));
        q.Run(1000);
    }
    CHECK(!q.eng.LooksByBase());
    CHECK(q.eng.LookReport(1).find("given up") != std::string::npos);
}

// The bare Appearance{} of a player whose look has not arrived is not a look: it must not be written into a copy (it would make the sex male and keep
// everything else of the local player).
void look_base_ignores_an_unknown_look() {
    ERig r;
    SetupLooks(r.be);
    CHECK(r.eng.LookBase(1, kTmpl, Appearance{}) == kTmpl);
    CHECK(r.be.clones == 0 && r.eng.stats().lookCopies == 0);
    CHECK(r.eng.LookReport(1).find("not known") != std::string::npos);
    CHECK(r.eng.LookBase(1, kTmpl, RemoteLook()) != kTmpl && r.be.clones == 1);
}

// A copy that shares its FaceGen arrays with the template (CopyFrom did not allocate its own) must not be written: the face of the local player would change.
void look_copy_without_own_face_arrays_keeps_the_template_face() {
    SRig r;
    SetupLooks(r.be);
    r.be.faceArraysOwned = false;
    r.pm.AddPlayer(1, RemoteLook());
    r.Pose(1, MkPose(kExtA, 300, 200));
    CHECK(r.UntilDriving(1));
    const Appearance w = r.be.copyLook[r.F(1)->base];
    CHECK(w.hair == 0x44 && w.faceCoeffs.size() == 130 && w.faceCoeffs[0] == 3);   // race, hair and parts are the remote's, the face numbers stay the template's
    CHECK(r.eng.stats().lookFaces == 0 && r.eng.stats().lookWrites == 1);
    CHECK(r.eng.LookReport(1).find("face not written") != std::string::npos);
}

// The face vector: every readable block goes out, an all-zero one included, and the receiver accepts it only at the length of its own readable blocks.
void face_blocks_line_up_and_zeros_travel() {
    FaceBlockRead in[kFaceBlockCount];
    const uint32_t counts[kFaceBlockCount] = {50, 30, 50};
    for (int b = 0; b < kFaceBlockCount; ++b) {
        in[b].readable = true;
        in[b].values.assign(counts[b], b == 0 ? 0.0f : 0.25f * static_cast<float>(b));   // the geometry block is all zero (a default face), the others are not
    }
    in[2].values[7] = -1.5f;
    std::vector<int8_t> packed;
    CHECK(PackFaceBlocks(in, packed) == 3);
    CHECK(packed.size() == 130);
    CHECK(packed.size() == 130 && packed[0] == 0 && packed[49] == 0 && packed[50] == 4 && packed[80] == 8 && packed[87] == -24);
    float a[50], b[30], c[50];
    for (float& f : a) f = 9.0f;
    for (float& f : b) f = 9.0f;
    for (float& f : c) f = 9.0f;
    FaceSlot slots[kFaceBlockCount] = {{50, a}, {30, b}, {50, c}};
    bool wrote = false;
    CHECK(UnpackFaceBlocks(packed.data(), packed.size(), slots, &wrote) && wrote);
    CHECK(a[0] == 0.0f && a[49] == 0.0f && b[0] == 0.25f && c[0] == 0.5f && c[7] == -1.5f);
    // a block that is shared with the template keeps its numbers, and the next block still lands in its own array
    for (float& f : a) f = 9.0f;
    for (float& f : b) f = 9.0f;
    for (float& f : c) f = 9.0f;
    FaceSlot shared[kFaceBlockCount] = {{50, a}, {30, nullptr}, {50, c}};
    CHECK(UnpackFaceBlocks(packed.data(), packed.size(), shared, &wrote) && wrote);
    CHECK(b[0] == 9.0f && c[0] == 0.5f && a[0] == 0.0f);
    // a vector of another length (a block one side could not read) writes nothing
    FaceSlot twoBlocks[kFaceBlockCount] = {{50, a}, {0, nullptr}, {50, c}};
    for (float& f : a) f = 9.0f;
    CHECK(!UnpackFaceBlocks(packed.data(), packed.size(), twoBlocks, &wrote) && !wrote && a[0] == 9.0f);
    // the sender could not read the middle block: its vector has two blocks, and a receiver that reads all three refuses it
    in[1].readable = false;
    CHECK(PackFaceBlocks(in, packed) == 2 && packed.size() == 100);
    CHECK(!UnpackFaceBlocks(packed.data(), packed.size(), slots, &wrote) && !wrote);
    CHECK(UnpackFaceBlocks(packed.data(), packed.size(), twoBlocks, &wrote) && wrote && a[0] == 0.0f && c[0] == 0.5f);
    // an oversized block is not a block
    in[1].readable = true;
    in[1].values.assign(65, 0.0f);
    CHECK(PackFaceBlocks(in, packed) == 2);
}

// ======================================================================================================================
// runtime: loopback, remote players, events, status
// ======================================================================================================================

struct RRig {
    FakeGameBackend be;
    std::unique_ptr<AvatarRuntime> rt;
    uint32_t now = 1000;
    std::vector<live::ActorSnapshot> sent;
    std::vector<std::vector<EquipEntry>> equipSent;
    explicit RRig(const std::function<void(RuntimeConfig&)>& tweak = nullptr) {
        SetupBackend(be);
        Appearance a;
        a.race = 1;
        a.name = "Courier";
        be.baseAppearance[7] = a;
        be.Player().pos = Vec3{1000, 1000, 0};
        RuntimeConfig c;
        c.puppet = MakeCfg();
        if (tweak) tweak(c);
        rt = std::make_unique<AvatarRuntime>(be, c);
        rt->SetLocalIdentity(0x10, 1);
        rt->SetSnapshotSink([this](const live::ActorSnapshot& s) { sent.push_back(s); });
        rt->SetEquipSink([this](const std::vector<EquipEntry>& e) { equipSent.push_back(e); });
    }
    void Frame(const std::function<void(float)>& move = nullptr) {
        if (move) move(0.016f);
        be.Frame();
        be.now += 16000;
        rt->Tick(now);
        now += 16;
    }
    void Run(uint32_t ms, const std::function<void(float)>& move = nullptr) {
        for (uint32_t t = 0; t < ms; t += 16) Frame(move);
    }
};

float DistXY(const Vec3& a, const Vec3& b) { return std::sqrt((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)); }

void runtime_loopback_follows_the_player() {
    RRig q;
    CHECK(q.rt->SetLoopback(true));
    CHECK(q.rt->loopback());
    float maxStep = 0, firstStep = 0;
    Vec3 prev;
    bool havePrev = false;
    double lagSum = 0;
    int lagN = 0;
    int runFrames = 0, runGroupFrames = 0;
    auto& pq = q.be.Player();
    uint32_t qt0 = q.now;
    for (int i = 0; i < 5000 / 16; ++i) {
        uint32_t t = q.now - qt0;
        if (t >= 1000 && t < 4000) pq.pos.y += 280 * 0.016f;
        q.Frame();
        RefId ref = q.rt->RefOf(1);
        auto* f = q.be.Get(ref);
        if (!ref || !f || q.rt->manager().PhaseOf(1) != Phase::Driving) continue;
        Vec3 want{pq.pos.x + 160, pq.pos.y, pq.pos.z};
        // the first frames: the new ref still stands where it was placed (on the player), and the driver moves it out of the player's body once
        if (havePrev) {
            if (t > 200) maxStep = std::max(maxStep, DistXY(prev, f->pos));
            else firstStep = std::max(firstStep, DistXY(prev, f->pos));
        }
        prev = f->pos;
        havePrev = true;
        if (t > 1800 && t < 3900) {
            lagSum += DistXY(want, f->pos);
            ++lagN;
            ++runFrames;
            if (f->group == kGrpFastForward) ++runGroupFrames;
        }
    }
    RefId ref = q.rt->RefOf(1);
    auto* f = q.be.Get(ref);
    CHECK(ref != 0 && f != nullptr);
    CHECK(q.rt->manager().PhaseOf(1) == Phase::Driving);
    CHECK(lagN > 50);
    CHECK(lagSum / std::max(1, lagN) < 80.0);        // behind by delay + render time at 280 u/s: a few tens of units
    CHECK(maxStep < 40.0f);                          // no teleports, no rubber band
    CHECK(firstStep < 100.0f);                       // at most the one hop out of the player's body (70), never the 160 of an eased arrival
    CHECK(runFrames > 50 && runGroupFrames * 10 >= runFrames * 8);   // the run group is playing for most of the run
    CHECK_NEAR(f->pos.x, pq.pos.x + 160, 3.0);       // and it ends up beside the player when the player stops
    CHECK_NEAR(f->pos.y, pq.pos.y, 3.0);
    CHECK(f->group == kGrpIdle);
    CHECK(f->restrained);
    CHECK(q.be.badLines.empty());
    CHECK(q.rt->engine().stats().foreignRefused == 0);
    // published stream at the sampler's rate (30 Hz as shipped, avatar_tuning.h), with the identity the co-op layer gave
    const double sentWant = 5.0 * 1000.0 / q.rt->config().sampler.periodMs;
    CHECK(q.sent.size() >= sentWant - 8 && q.sent.size() <= sentWant + 8);
    CHECK(!q.sent.empty() && q.sent.front().netId == 0x10 && q.sent.front().ownerEpoch == 1 && q.sent.front().seq == 1);
    // console cost: running costs about a line a frame, standing nothing
    CHECK(q.rt->engine().stats().lines < 5000 / 16 * 2);
}

// The whole chain on one machine: the local player swings, shoots, throws, blocks and gets hit; the loopback puppet (the same path a remote player's puppet takes:
// snapshot -> interpolation buffer -> driver -> console) plays each one exactly once, with a group name the console accepts.
void runtime_loopback_shows_the_players_attacks() {
    RRig q;
    CHECK(q.rt->SetLoopback(true));
    q.Run(2500);
    const RefId ref = q.rt->RefOf(1);
    CHECK(ref != 0 && q.rt->manager().PhaseOf(1) == Phase::Driving);
    auto& pl = q.be.Player();
    auto play = [&](int group, uint32_t ms) {
        pl.group = group;
        pl.groupSlot = 1;
        pl.groupState = 1;
        pl.groupPlayMs = 0;
        q.Run(ms, [&](float) { pl.groupPlayMs += 16; });
        pl.group = -1;
        pl.groupPlayMs = -1;
        q.Run(1500);
    };
    for (int i = 0; i < 3; ++i) play(26, 350);   // three punches
    CHECK(CountLines(q.be, ref, "PlayGroup AttackLeft 0") == 3);
    play(32, 350);                               // a swing
    play(92, 600);                               // a power attack
    play(114, 400);                              // a throw
    play(38, 200);                               // a shot
    play(223, 500);                              // a stagger
    CHECK(CountLines(q.be, ref, "PlayGroup AttackRight 0") == 1);
    CHECK(CountLines(q.be, ref, "PlayGroup AttackPower 0") == 1);
    CHECK(CountLines(q.be, ref, "PlayGroup AttackThrow 0") == 1);
    CHECK(CountLines(q.be, ref, "PlayGroup Attack3 0") == 1);
    CHECK(CountLines(q.be, ref, "PlayGroup Stagger 0") == 1);
    // block is held with the hold flag, released again, and the movement group comes back
    pl.group = 170;
    pl.groupState = 1;
    pl.groupPlayMs = 0;
    q.Run(1500, [&](float) { pl.groupPlayMs += 16; });
    CHECK(CountLines(q.be, ref, "PlayGroup BlockIdle 1") == 1);
    pl.group = -1;
    q.Run(1500);
    CHECK(q.be.Get(ref)->group == kGrpIdle);
    // nothing else was said about a group the console does not know, and the sampler's counter is the number of events
    CHECK(q.be.badLines.empty());
    CHECK(q.rt->manager().stats().oneShots == 3 + 5 + 1);
    CHECK(q.rt->manager().stats().oneShotsFailed == 0);
}


// The same through the whole runtime: a human's first-person punch is sent in a snapshot, the loopback puppet punches, and nvcoop.log says so
// (local attack seen ... / remote attack received ... puppet animation started ...).
void runtime_first_person_punch_is_sent_shown_and_logged() {
    RRig q;
    std::vector<std::string> lines;
    q.rt->SetLog([&](const std::string& l) { lines.push_back(l); });
    CHECK(q.rt->SetLoopback(true));
    q.be.Player().firstPerson = true;
    q.be.Player().fpSet = true;
    q.Run(2500);
    const RefId ref = q.rt->RefOf(1);
    CHECK(ref != 0 && q.rt->manager().PhaseOf(1) == Phase::Driving);
    auto& pl = q.be.Player();
    for (int i = 0; i < 2; ++i) {
        pl.fpGroup = 26;
        pl.fpState = 1;
        pl.fpPlayMs = 0;
        q.Run(350, [&](float) { pl.fpPlayMs += 16; });
        pl.fpGroup = -1;
        pl.fpPlayMs = -1;
        q.Run(1500);
    }
    size_t withAction = 0;
    for (const auto& s : q.sent)
        if (s.anim.action == 26) ++withAction;
    CHECK(withAction >= 6);                                           // sent
    CHECK(CountLines(q.be, ref, "PlayGroup AttackLeft 0") == 2);      // shown
    auto count = [&](const char* needle) {
        int n = 0;
        for (const auto& l : lines)
            if (l.find(needle) != std::string::npos) ++n;
        return n;
    };
    CHECK(count("local attack seen: group 26 (AttackLeft) from the first-person set, camera first person") == 2);
    CHECK(count("remote attack received from slot 1: group 26 (AttackLeft)") == 2 && count("puppet animation started") == 2);
    CHECK(count("local attack source: 2 animation set(s) readable, camera first person") == 1);
}

void runtime_loopback_off_removes_the_puppet() {
    RRig r;
    r.rt->SetLoopback(true);
    r.Run(1500);
    RefId ref = r.rt->RefOf(1);
    CHECK(ref != 0);
    CHECK(r.rt->SetLoopback(false));
    CHECK(!r.rt->loopback());
    CHECK(r.be.Get(ref)->deleted);
    CHECK(r.rt->RefOf(1) == 0);
    r.Run(500);
    CHECK(r.rt->RefOf(1) == 0);
    CHECK(r.rt->manager().Count() == 0);
}

void runtime_loopback_refuses_the_local_slot() {
    RRig r([](RuntimeConfig& c) { c.loopbackSlot = c.puppet.localSlot; });
    CHECK(!r.rt->SetLoopback(true));
    CHECK(!r.rt->loopback());
}

void runtime_loopback_carries_equipment() {
    RRig r;
    r.be.Player().worn = {kArmor, kGun};
    r.be.Player().inv[kArmor] = 1;
    r.be.Player().inv[kGun] = 1;
    r.rt->SetLoopback(true);
    r.Run(10000);
    CHECK(r.equipSent.size() == 1);
    auto* f = r.be.Get(r.rt->RefOf(1));
    CHECK(f && f->worn.count(kArmor) == 1 && f->worn.count(kGun) == 1);
    CHECK(r.rt->manager().EquipConverged(1));
}

live::ActorSnapshot RemoteSnap(live::NetId id, uint32_t seq, uint32_t tick, CellId cell, Vec3 pos, Vec3 vel, uint8_t loco) {
    live::ActorSnapshot s;
    s.netId = id;
    s.ownerEpoch = 1;
    s.seq = seq;
    s.tick = tick;
    s.cell = cell;
    s.pos = ToLive(pos);
    s.vel = ToLive(vel);
    s.anim.locomotion = loco;
    return s;
}

void runtime_remote_players_follow_their_own_streams() {
    RRig r;
    Appearance a;
    a.race = 1;
    CHECK(r.rt->AddRemote(1, a));
    CHECK(r.rt->AddRemote(2, a));
    CHECK(!r.rt->AddRemote(0, a));   // the local slot never gets a puppet
    uint32_t seq1 = 0, seq2 = 0;
    // slot 1 walks +Y, slot 2 walks -X; 20 Hz, 40 ms latency
    for (int i = 0; i < 200; ++i) {
        uint32_t tick = 5000 + i * 50;
        r.rt->OnRemoteSnapshot(1, RemoteSnap(0x21, ++seq1, tick, kExtA, Vec3{500, 500.0f + 7.5f * i, 0}, Vec3{0, 150, 0}, kLocoWalkFwd), tick + 40);
        r.rt->OnRemoteSnapshot(2, RemoteSnap(0x22, ++seq2, tick, kExtA, Vec3{2000 - 7.5f * i, 800, 0}, Vec3{-150, 0, 0}, kLocoWalkFwd), tick + 40);
        for (int f = 0; f < 3; ++f) {
            r.be.Frame();
            r.rt->Tick(tick + 40 + f * 16);
        }
    }
    auto* p1 = r.be.Get(r.rt->RefOf(1));
    auto* p2 = r.be.Get(r.rt->RefOf(2));
    CHECK(p1 && p2 && p1->id != p2->id);
    CHECK(p1->pos.y > 1800 && std::fabs(p1->pos.x - 500) < 1);
    CHECK(p2->pos.x < 700 && std::fabs(p2->pos.y - 800) < 1);
    CHECK(r.rt->remoteSnapshots() == 400 && r.rt->droppedRemote() == 0);
    r.rt->OnRemoteSnapshot(5, RemoteSnap(0x23, 1, 99999, kExtA, Vec3{}, Vec3{}, 0), 100000);
    CHECK(r.rt->droppedRemote() == 1);   // unknown slot: dropped and counted
    CHECK(r.be.badLines.empty());
    // the ghost-ref map
    CHECK(r.rt->SlotOf(p1->id).value_or(99) == 1 && r.rt->SlotOf(p2->id).value_or(99) == 2);
}

void runtime_no_puppet_before_the_first_snapshot() {
    RRig r;
    Appearance a;
    a.race = 1;
    r.rt->AddRemote(1, a);
    r.Run(1000);
    CHECK(r.rt->posesFed() == 0);   // an empty buffer feeds no pose: the driver never sees a made-up place
    CHECK(r.rt->RefOf(1) == 0);
    CHECK(r.rt->manager().PhaseOf(1) == Phase::Absent);
    CHECK(r.be.lines.empty());
}

void runtime_game_events() {
    RRig r;
    r.rt->SetLoopback(true);
    r.Run(2000);
    RefId first = r.rt->RefOf(1);
    CHECK(first != 0);
    r.rt->OnEvent(GameEvent::SaveBegin);
    CHECK(r.be.Get(first)->deleted);
    r.Run(200);
    CHECK(r.rt->RefOf(1) == 0);
    r.rt->OnEvent(GameEvent::SaveEnd);
    r.Run(2000);
    RefId second = r.rt->RefOf(1);
    CHECK(second != 0 && second != first);
    // a save that never reports back must not keep the puppets away for ever
    r.rt->OnEvent(GameEvent::SaveBegin);
    r.Run(500);
    CHECK(r.rt->RefOf(1) == 0);
    r.Run(11000);
    CHECK(r.rt->RefOf(1) != 0);
    second = r.rt->RefOf(1);
    // a load: nothing is published while it runs, the old puppet ref is forgotten and purged afterwards
    size_t sent0 = r.sent.size();
    r.be.Load();
    r.rt->OnEvent(GameEvent::LoadBegin);
    r.Run(500);
    CHECK(r.sent.size() == sent0);
    r.rt->OnEvent(GameEvent::LoadEnd);
    r.Run(3000);
    CHECK(r.be.Get(second)->deleted);
    CHECK(r.rt->RefOf(1) != 0 && r.rt->RefOf(1) != second);
    CHECK(r.sent.size() > sent0);
    // a new game / return to the main menu behaves like a load without a Pre / Post pair
    RefId third = r.rt->RefOf(1);
    r.be.Load();
    r.rt->OnEvent(GameEvent::NewGame);
    r.Run(3000);
    CHECK(r.be.Get(third)->deleted);
    CHECK(r.rt->RefOf(1) != third);
}

void runtime_status_json_contract() {
    RRig r;
    r.be.AddActor(kTmpl, kExtB, Vec3{4200, 0, 0});   // one orphan counts as tagged
    r.rt->SetLoopback(true);
    r.Run(2500);
    std::string js = r.rt->StatusJson();
    nvcoop::json::Value v;
    std::string err;
    CHECK(nvcoop::json::parse(js, v, err));
    CHECK(v.get("v").asInt() == 1);
    CHECK(v.get("loopback").asBool());
    CHECK(v.get("tagged").asInt() >= 1);
    // everything proof_puppet.py asserts on
    const nvcoop::json::Value& st = v.get("stats");
    for (const char* k : {"spawnFailures", "refLost", "restrainReasserts", "groupRefused", "drift", "equipGaveUp", "load3DTimeouts"})
        CHECK(st.find(k) != nullptr);
    const nvcoop::json::Value* pup = v.find("puppets");
    CHECK(pup != nullptr);
    CHECK(js.find("\"slot\":1") != std::string::npos && js.find("\"phase\":\"Driving\"") != std::string::npos);
    const nvcoop::json::Value& eng = v.get("engine");
    CHECK(eng.find("costP95Us") != nullptr && eng.find("lines") != nullptr && eng.find("evalFailures") != nullptr);
    const nvcoop::json::Value& loc = v.get("local");
    CHECK(loc.get("have").asBool());
    CHECK(loc.get("published").asInt() > 40);
    std::string lj = r.rt->LocalJson();
    CHECK(nvcoop::json::parse(lj, v, err));
    CHECK(v.get("pos").type() == nvcoop::json::Value::Array);
}

// ======================================================================================================================

// W3a item 3: the facts a guest's snapshot does not carry. Twin 77: no action id is sampled.
void runtime_reads_the_local_facts() {
    RRig r;
    auto& p = r.be.Player();
    p.level = 17;
    p.sitState = 2;
    p.actions = {9, 3};
    r.be.playerRegions = {0x00ABCD, 0x01FF00};
    PlayerFacts f;
    CHECK(r.rt->ReadLocalFacts(f));
    CHECK(f.level == 17 && f.sitting == 2);
    CHECK(f.actions == ((1u << 9) | (1u << 3)));
    CHECK(f.regions.size() == 2 && f.regions[0] == 0x00ABCD && f.regions[1] == 0x01FF00);
    r.be.regionsReadable = false;                       // the region list cannot be read: the rest still goes
    PlayerFacts g;
    CHECK(r.rt->ReadLocalFacts(g) && g.level == 17 && g.regions.empty());
    r.be.evalBroken = {Cond::GetLevel, Cond::GetSitting, Cond::IsPlayerActionActive};
    PlayerFacts h;
    CHECK(!r.rt->ReadLocalFacts(h));                    // nothing readable: no facts at all, not zeros
}

void runtime_reads_the_local_pack() {
    RRig r;
    auto& p = r.be.Player();
    p.inv[0x00A84A] = 3;
    p.inv[0x0093C4] = 1;
    p.inv[0x00C0DE] = 50;                 // carried, not watched
    p.inventoryWeight = 75.96;
    const std::vector<FormId> watch = {0x00F85D, 0x0093C4, 0x00A84A, 0x0093C4};   // unsorted, a form twice, one not carried
    PlayerPack k;
    CHECK(r.rt->ReadLocalPack(watch, k));
    CHECK(k.haveWeight && k.weightMilli == 75960);                                // InventoryWeight, in thousandths
    CHECK(k.items.size() == 2 && k.items[0].form == 0x0093C4 && k.items[0].count == 1 && k.items[1].form == 0x00A84A && k.items[1].count == 3);   // sorted, only what is carried
    CHECK(k.watchId == packWatchId({0x00A84A, 0x0093C4, 0x00F85D}));              // the id of the list, not of the order it came in
    CHECK(k.watchId != packWatchId({0x00A84A, 0x0093C4}));
    PlayerPack none;
    CHECK(r.rt->ReadLocalPack({}, none) && none.items.empty() && none.haveWeight);   // no list: the weight alone
    r.be.evalBroken = {Cond::GetActorValue};
    PlayerPack noW;
    CHECK(r.rt->ReadLocalPack(watch, noW) && !noW.haveWeight && noW.items.size() == 2);   // the weight cannot be read: the counts still go
    r.be.evalBroken = {Cond::GetActorValue, Cond::GetItemCount};
    PlayerPack h;
    CHECK(!r.rt->ReadLocalPack(watch, h));                                        // nothing readable: no pack at all, not zeros
}

void runtime_remote_look_update_and_local_look() {
    RRig r;
    r.be.baseAppearance[7].height = 1050;              // the player's base (fake player base id 7)
    Appearance mine;
    CHECK(r.rt->ReadLocalAppearance(mine) && mine.height == 1050 && mine.name == "Courier");
    Appearance a;
    a.race = 1;
    CHECK(!r.rt->SetRemoteAppearance(3, a));           // unknown slot
    CHECK(r.rt->AddRemote(2, a));
    uint32_t seq = 0;
    auto Feed = [&](int from, int to) {
        for (int i = from; i < to; ++i) {
            uint32_t tick = 5000 + i * 50;
            r.rt->OnRemoteSnapshot(2, RemoteSnap(0x22, ++seq, tick, kExtA, Vec3{500, 500, 0}, Vec3{}, 0), tick + 40);
            for (int f = 0; f < 3; ++f) {
                r.be.Frame();
                r.rt->Tick(tick + 40 + f * 16);
            }
        }
    };
    Feed(0, 60);
    const RefId first = r.rt->RefOf(2);
    CHECK(first != 0);
    a.height = 1100;
    CHECK(r.rt->SetRemoteAppearance(2, a));            // same race and sex: the live puppet is updated, not replaced
    Feed(60, 100);
    CHECK(r.rt->RefOf(2) == first);
    CHECK_NEAR(r.be.Get(first)->scale, 1.1, 1e-4);
    CHECK(r.be.badLines.empty());
}

// Twin 72 (live wiring 2026-10-22): the runtime waits for a player in a loaded cell and the status says so.
void runtime_starts_only_with_a_player_and_says_why_not() {
    using namespace nvcoop::avatar;
    CHECK(!shouldCreateRuntime(true, true, false, 30, false));    // main menu: nothing to place a puppet in (twin 72: created anyway)
    CHECK(shouldCreateRuntime(true, true, false, 30, true));      // a save is loaded
    CHECK(!shouldCreateRuntime(true, true, false, 31, true));     // checked every 30th frame only
    CHECK(!shouldCreateRuntime(true, true, true, 60, true));      // exists already
    CHECK(!shouldCreateRuntime(false, true, false, 30, true));    // [avatar] enabled=0 and no live switch
    CHECK(!shouldCreateRuntime(true, false, false, 30, true));    // no console interface
    CHECK(std::string(notRunningBecause(true, true)).empty());
    CHECK(std::string(notRunningBecause(true, false)).find("loaded game") != std::string::npos);
    CHECK(std::string(notRunningBecause(false, false)).find("enabled=0") != std::string::npos);
}

struct TestCase {
    const char* name;
    void (*fn)();
};
#define T(n) {#n, n}
const TestCase kTests[] = {
    T(spawn_places_and_finds_new_ref), T(spawn_without_3d_still_returns_ref), T(spawn_anchor_other_cell), T(spawn_failures_are_counted),
    T(spawn_reuses_a_freed_id), T(spawn_rejected_ref_is_removed_again), T(spawn_circuit_breaker), T(spawn_untagged_base_refused), T(spawn_takes_the_newest_new_ref), T(spawn_does_not_adopt_existing), T(guard_foreign_actor_untouched),
    T(guard_placed_actor_with_a_tagged_base_is_not_ours), T(guard_player_untouched), T(guard_reused_id_with_other_base), T(delete_two_commands_in_order), T(setpos_sends_only_differing_axes),
    T(setpos_needs_3d_and_finite), T(setyaw_degrees_wrapped_and_skipped), T(playgroup_text), T(currentgroup_reads_any_slot),
    T(restraint_readback_and_fallback), T(nonCombatant_lines), T(generation3d_tracking), T(stance_and_weapon), T(equipment_calls),
    T(itemcount_fallback_when_evaluator_unavailable), T(appearance_is_honest), T(find_tagged_scans_every_loaded_cell), T(world_state_mapping),
    T(move_to_cell_uses_an_anchor), T(get_pose_reports_the_real_place), T(get_local_pose_reads_the_player), T(console_cost_accounting), T(console_exceptions_and_gone_refs),
    T(stack_spawn_and_drive), T(stack_running_costs_one_line_per_step), T(stack_equipment_converges), T(stack_save_and_load_purge),
    T(stack_cell_change), T(stack_rebuilt_3d_is_reasserted), T(stack_menu_pauses_and_replays), T(stack_survives_all_evaluators_missing),
    T(stack_reused_ref_ids), T(stack_makes_room_for_the_local_player), T(sampler_period_identity_and_sequence), T(sampler_velocity_and_locomotion),
    T(sampler_teleport_and_cell_change_restart_the_speed), T(sampler_speed_is_clamped), T(sampler_small_jump_is_a_teleport_not_a_speed), T(sampler_flags_from_the_engine), T(sampler_mover_flags_only_when_informative),
    T(sampler_attack_reaches_the_snapshot), T(sampler_first_person_attack_reaches_the_snapshot), T(sampler_every_start_is_an_event), T(sampler_block_is_held_and_released),
    T(sampler_ignores_dead_players_and_leftover_sequences), T(sampler_action_counter_survives_loading_screens),
    T(sampler_not_ready_publishes_nothing), T(equip_watcher_publishes_stable_changes_only), T(local_appearance_is_read_from_the_base),
    T(runtime_loopback_follows_the_player), T(runtime_loopback_shows_the_players_attacks), T(runtime_first_person_punch_is_sent_shown_and_logged), T(runtime_loopback_off_removes_the_puppet), T(runtime_loopback_refuses_the_local_slot),
    T(runtime_loopback_carries_equipment), T(runtime_remote_players_follow_their_own_streams), T(runtime_no_puppet_before_the_first_snapshot),
    T(runtime_game_events), T(runtime_status_json_contract),
    T(appearance_height_is_matched_with_setscale), T(look_copy_carries_the_remote_look), T(look_copy_refusal_is_not_forever),
    T(look_base_ignores_an_unknown_look), T(look_copy_without_own_face_arrays_keeps_the_template_face), T(face_blocks_line_up_and_zeros_travel), T(weapon_mod_lines_and_readback), T(hit_target_health_lines),
    T(stack_weapon_mods_converge_and_follow_changes), T(stack_hit_target_puppet_survives_and_is_replaced), T(stack_look_update_keeps_the_puppet),
    T(equip_watcher_carries_weapon_mods), T(runtime_reads_the_local_facts), T(runtime_reads_the_local_pack), T(runtime_remote_look_update_and_local_look), T(runtime_starts_only_with_a_player_and_says_why_not),
};

}  // namespace

int main(int argc, char** argv) {
    int mut = fnvcoop_mut::initFromArgs(argc, argv);
    std::vector<std::string> filters;
    bool list = false;
    for (int i = 1; i < argc; ++i) {
        if (!argv[i][0]) continue;
        if (!std::strcmp(argv[i], "--list")) list = true;
        else if (!std::strcmp(argv[i], "--verbose")) g_verbose = true;
        else filters.push_back(argv[i]);
    }
    if (list) {
        for (const TestCase& t : kTests) std::printf("%s\n", t.name);
        return 0;
    }
    int ran = 0;
    for (const TestCase& t : kTests) {
        bool pick = filters.empty();
        for (const std::string& f : filters)
            if (std::strstr(t.name, f.c_str())) pick = true;
        if (!pick) continue;
        g_test = t.name;
        g_testFail = 0;
        if (g_verbose) std::printf("RUN %s\n", t.name);
        t.fn();
        ++ran;
    }
    std::printf("avatar_engine tests (mut %d): %d tests, %d checks, %d failed\n", mut, ran, g_checks, g_fail);
    if (g_fail) return 1;
    std::printf("avatar_engine: tests pass\n");
    return 0;
}
