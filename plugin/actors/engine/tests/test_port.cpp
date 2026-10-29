// EnginePort over the fake engine: identity, AI switch (and what it must NOT touch), driving (epsilons, rate cap, line budget, cells),
// sampling, actor values, placement, protection, stand-ins, leveled-pick clones, death, ledger-driven clean-up.
// Twins 11..38 live in engine_port.cpp (see the FNV_MUT markers there).
#include <cmath>
#include <string>

#include "minitest.h"
#include "actors/state_parts.h"
#include "port_rig.h"

using namespace nvae_test;

namespace {
constexpr float kPi = 3.14159265f;
}

// ---- identity ------------------------------------------------------------------------------------------------------------------------

TEST(port, bind_and_unbind_keep_the_directory) {
    PortRig r;
    const NetId id = r.npc(0x100);
    CHECK(r.port.loaded(id) && r.port.bound(id));
    CHECK(r.port.activeRef(id) == 0x100 && r.port.realRef(id) == 0x100);
    CHECK(r.port.netOfRef(0x100) == id && r.port.netOfRef(0x999) == 0);
    CHECK(!r.port.loaded(12345));
    r.port.unbindReal(id);
    CHECK(!r.port.loaded(id) && r.port.netOfRef(0x100) == 0);
    CHECK(r.port.ids().empty());
}

TEST(port, cell_answers_are_cached_within_a_frame) {
    PortRig r;
    r.eng.cells[7] = FakeEngine::Cell{};
    CHECK(r.port.cellLoaded(7));
    r.eng.cells[7].loaded = false;
    CHECK(r.port.cellLoaded(7));    // same frame
    r.frame();
    CHECK(!r.port.cellLoaded(7));
}

// ---- the AI switch --------------------------------------------------------------------------------------------------------------------

TEST(port, ai_off_stops_combat_then_restrains_and_is_recorded) {
    PortRig r;
    const NetId id = r.npc(0x100);
    r.eng.a[0x100].target = 0x14;     // in combat with the player
    r.port.setAi(id, false);
    CHECK(r.eng.a[0x100].restrained && r.eng.a[0x100].target == 0);
    CHECK(r.eng.log.size() >= 2 && r.eng.log[0] == "stopcombat:256:" && r.eng.log[1] == "restrain:256:");
    CHECK(r.led.has(LedgerKind::Restrained, 0x100));
    CHECK(r.port.aiOff(id));
    const size_t n = r.eng.log.size();
    r.port.setAi(id, false);          // idempotent
    CHECK(r.eng.log.size() == n);
    CHECK(r.port.stats().aiOff == 1);
}

TEST(port, a_calm_actor_is_restrained_with_one_line) {
    PortRig r;
    const NetId id = r.npc(0x100);
    r.port.setAi(id, false);
    CHECK(r.eng.log.size() == 1 && r.eng.log[0] == "restrain:256:");
    CHECK(r.eng.countOp("stopcombat") == 0);
}

TEST(port, ai_on_releases_only_what_the_port_holds) {
    PortRig r;
    const NetId id = r.npc(0x100);
    r.port.setAi(id, true);           // an owner that never lost its AI: nothing to do
    CHECK(r.eng.log.empty());
    r.port.setAi(id, false);
    r.port.setAi(id, true);
    CHECK(!r.eng.a[0x100].restrained && r.eng.count("evalpackage", 0x100) == 1);
    CHECK(!r.led.has(LedgerKind::Restrained, 0x100) && !r.port.aiOff(id));
    CHECK(r.port.stats().aiOn == 1);
}

TEST(port, a_script_restrained_actor_is_never_unfrozen_by_the_port) {
    PortRig r;
    const NetId id = r.npc(0x100);
    r.eng.a[0x100].scriptRestrained = true;     // a quest holds this NPC
    r.port.setAi(id, false);
    CHECK(r.eng.count("restrain", 0x100) == 0 && !r.led.has(LedgerKind::Restrained, 0x100));
    CHECK(r.port.aiOff(id));
    r.port.setAi(id, true);
    CHECK(r.eng.count("unrestrain", 0x100) == 0);
    CHECK(r.eng.a[0x100].scriptRestrained);
}

TEST(port, an_actor_restrained_by_the_port_earlier_is_still_released_after_it_was_rebound) {
    PortRig r;
    NetId id = r.npc(0x100);
    r.port.setAi(id, false);
    r.port.unbindReal(id);                       // the cell unloaded; the flag stays on the ref
    id = r.npc(0x100);                           // ... and loaded again (same ref, fresh directory entry)
    r.eng.a[0x100].restrained = true;
    r.port.setAi(id, false);                     // ActorSync asks again: must recognise its own flag, not call it a script's
    CHECK(r.eng.count("restrain", 0x100) == 1);
    r.port.setAi(id, true);
    CHECK(!r.eng.a[0x100].restrained);
}

TEST(port, a_takeover_restarts_combat_against_the_last_known_target) {
    PortRig r;
    const NetId a = r.npc(0x100), b = r.npc(0x200, {1500, 1000, 0});
    live::InterpOut o = interpAt({1000, 1000, 0});
    o.ai.combatTarget = b;
    r.port.setAi(a, false);
    r.port.drive(a, o);                          // the owner's snapshots carry its target
    r.port.setAi(a, true);
    CHECK(r.eng.did("startcombat:256:" + std::to_string(0x200)));
    CHECK(r.port.stats().combatRestarted == 1);
    // an unknown target (a player id nobody resolves) is counted, not guessed
    r.port.setAi(a, false);
    o.ai.combatTarget = 0xDEAD;
    r.port.drive(a, o);
    r.port.setAi(a, true);
    CHECK(r.port.stats().combatTargetUnresolved == 1 && r.eng.countOp("startcombat") == 1);
}

// ---- driving --------------------------------------------------------------------------------------------------------------------------

TEST(port, drive_writes_only_what_moved_and_respects_the_epsilons) {
    PortRig r;
    const NetId id = r.npc(0x100, {1000, 1000, 0});
    r.port.setAi(id, false);
    r.eng.log.clear();
    r.port.drive(id, interpAt({1100, 1000, 0}, 1.0f));
    r.step();
    CHECK(r.eng.count("setpos", 0x100) == 1 && r.eng.did("setpos:256:x"));
    CHECK(r.eng.count("setyaw", 0x100) == 1);
    CHECK(std::fabs(r.eng.a[0x100].pos.x - 1100) < 0.01f && std::fabs(r.eng.a[0x100].rot.z - 1.0f) < 0.001f);
    r.eng.log.clear();
    r.port.drive(id, interpAt({1100.1f, 1000.1f, 0.4f}, 1.005f));     // below every epsilon
    r.step();
    CHECK_MSG(r.eng.log.empty(), "a sub-epsilon drive wrote " + std::to_string(r.eng.log.size()) + " lines");
    CHECK(r.port.stats().drivesSkippedSmall >= 1);
    r.port.drive(id, interpAt({1100, 1000, 9}, 1.0f));                // z only
    r.step();
    CHECK(r.eng.did("setpos:256:z") && r.eng.count("setpos", 0x100) == 1 && r.eng.count("setyaw", 0x100) == 0);
}

TEST(port, drive_is_rate_limited_per_actor_and_keeps_the_latest_pose) {
    PortRig r;
    const NetId id = r.npc(0x100, {1000, 1000, 0});
    r.port.setAi(id, false);
    r.port.drive(id, interpAt({1100, 1000, 0}));
    r.step();                                     // applied
    r.port.drive(id, interpAt({1200, 1000, 0}));
    r.step(10);                                   // 10 ms later: inside the 33 ms window
    CHECK_MSG(std::fabs(r.eng.a[0x100].pos.x - 1100) < 0.01f, "x=" + std::to_string(r.eng.a[0x100].pos.x));
    CHECK(r.port.pendingDrive(id));
    r.step(40);                                   // window over: the held pose goes in
    CHECK(std::fabs(r.eng.a[0x100].pos.x - 1200) < 0.01f);
    CHECK(!r.port.pendingDrive(id));
}

// A copy getting out of its chair keeps its place until the animation driver says it is out (Doc Mitchell slid across the room seated, 2026-10-29).
TEST(port, a_copy_still_in_its_furniture_is_not_moved_until_it_stood_up) {
    PortRig r;
    const NetId id = r.npc(0x100, {1000, 1000, 0});
    r.port.setAi(id, false);
    bool seated = true;
    nvcoop::actors::engine::setPoseLock([&](NetId n) { return n == id && seated; });
    r.port.drive(id, interpAt({1200, 1000, 0}));
    r.step(50);
    CHECK_MSG(std::fabs(r.eng.a[0x100].pos.x - 1000) < 0.01f, "moved while seated: x=" + std::to_string(r.eng.a[0x100].pos.x));
    CHECK(r.port.pendingDrive(id) && r.port.stats().drivesHeldSeated >= 1);
    seated = false;
    r.step(50);
    CHECK(std::fabs(r.eng.a[0x100].pos.x - 1200) < 0.01f && !r.port.pendingDrive(id));
    seated = true;
    r.port.drive(id, interpAt({1300, 1000, 0}, 0, 1, true));   // a teleport is applied even while seated
    r.step(50);
    CHECK(std::fabs(r.eng.a[0x100].pos.x - 1300) < 0.01f);
    nvcoop::actors::engine::setPoseLock(nullptr);
}

TEST(port, a_snap_ignores_the_window_and_writes_every_axis) {
    PortRig r;
    const NetId id = r.npc(0x100, {1000, 1000, 0});
    r.port.setAi(id, false);
    r.port.drive(id, interpAt({1100, 1000, 0}));
    r.step();
    r.eng.log.clear();
    r.port.drive(id, interpAt({1105, 1000, 0.1f}, 0, 1, true));       // snapped, 5 ms-ish later, tiny change
    r.step(5);
    CHECK(r.eng.did("setpos:256:xyz") && r.eng.count("setyaw", 0x100) == 1);
    CHECK(r.port.stats().snaps == 1);
}

TEST(port, the_line_budget_bounds_a_frame_and_nobody_starves) {
    Config c = rigConfig();
    c.maxLinesPerFrame = 8;
    PortRig r(c);
    std::vector<NetId> ids;
    for (int i = 0; i < 6; ++i) {
        ids.push_back(r.npc(0x100 + static_cast<RefId>(i), {1000.0f + 100.0f * static_cast<float>(i), 1000, 0}));
        r.port.setAi(ids.back(), false);
    }
    r.eng.log.clear();
    r.frame();
    for (int i = 0; i < 6; ++i)
        r.port.drive(ids[static_cast<size_t>(i)], interpAt({1500.0f + 100.0f * static_cast<float>(i), 1500, 0}, 1.0f));
    r.port.flush();
    const int served1 = [&] { int n = 0; for (int i = 0; i < 6; ++i) if (r.eng.count("setpos", 0x100 + static_cast<RefId>(i))) ++n; return n; }();
    CHECK_MSG(served1 >= 2 && served1 <= 3, "served " + std::to_string(served1) + " in one frame, lines=" + std::to_string(r.eng.log.size()));
    CHECK(r.port.stats().linesDeferred >= 1);
    for (int f = 0; f < 3; ++f) {      // everybody keeps being re-driven: all must still get their turn
        r.frame();
        for (int i = 0; i < 6; ++i)
            r.port.drive(ids[static_cast<size_t>(i)], interpAt({1500.0f + 100.0f * static_cast<float>(i) + 50.0f * static_cast<float>(f + 1),
                                                                1500.0f + 60.0f * static_cast<float>(f + 1), 0}, 1.0f + 0.3f * static_cast<float>(f + 1)));
        r.port.flush();
    }
    for (int i = 0; i < 6; ++i)
        CHECK_MSG(r.eng.count("setpos", 0x100 + static_cast<RefId>(i)) >= 1, "actor " + std::to_string(i) + " was never served");
}

TEST(port, a_cell_change_between_interiors_is_a_move_and_an_unloaded_cell_is_not_entered) {
    PortRig r;
    r.eng.cells[5] = FakeEngine::Cell{};
    const NetId id = r.npc(0x100, {1000, 1000, 0});
    r.port.setAi(id, false);
    r.port.drive(id, interpAt({10, 20, 0}, 0, 5));
    r.step();
    CHECK(r.eng.count("moveto", 0x100) == 1 && r.eng.a[0x100].cell == 5);
    CHECK(r.eng.count("setpos", 0x100) == 0);
    // a cell that is not loaded here: the actor stays where it is
    r.eng.cells[6] = FakeEngine::Cell{};
    r.eng.cells[6].loaded = false;
    r.port.drive(id, interpAt({50, 50, 0}, 0, 6, true));
    r.step();
    CHECK(r.eng.a[0x100].cell == 5 && r.eng.count("moveto", 0x100) == 1 && r.eng.count("setpos", 0x100) == 0);
}

TEST(port, a_step_across_exterior_grid_cells_of_one_worldspace_is_a_plain_setpos) {
    PortRig r;
    r.eng.cells[10] = FakeEngine::Cell{true, true, 3, 0, 0};
    r.eng.cells[11] = FakeEngine::Cell{true, true, 3, 1, 0};
    const NetId id = r.npc(0x100, {4000, 100, 0}, 0x500, false, 10);
    r.port.setAi(id, false);
    r.port.drive(id, interpAt({4200, 100, 0}, 0, 11));
    r.step();
    CHECK(r.eng.count("moveto", 0x100) == 0 && r.eng.did("setpos:256:x"));
}

TEST(port, pose_reports_where_the_ref_really_is) {
    PortRig r;
    const NetId id = r.npc(0x100, {1000, 1000, 0});
    r.port.setAi(id, false);
    r.port.drive(id, interpAt({1100, 1000, 0}));
    r.step();
    r.eng.a[0x100].pos.x += 50;          // physics pushed it
    live::ActualPose p;
    CHECK(r.port.pose(id, p));
    CHECK_MSG(std::fabs(p.pos.x - 1150) < 0.01f, "pose x=" + std::to_string(p.pos.x));
    CHECK(!r.port.pose(777, p));
}

TEST(port, drive_listeners_see_every_driven_actor_even_when_nothing_is_written) {
    PortRig r;
    const NetId id = r.npc(0x100, {1000, 1000, 0});
    r.port.setAi(id, false);
    live::InterpOut o = interpAt({1000, 1000, 0});
    o.anim.weaponDrawn = true;
    o.ai.alert = 3;
    r.port.drive(id, o);
    r.port.drive(id, o);
    r.step();
    CHECK(r.driven.size() == 2 && r.driven[0].first == id && r.driven[0].second.anim.weaponDrawn && r.driven[1].second.ai.alert == 3);
}

// ---- sampling ----------------------------------------------------------------------------------------------------------------------------

TEST(port, sample_reports_the_native_state_of_an_owned_actor) {
    PortRig r;
    const NetId a = r.npc(0x100, {1000, 1000, 0}), b = r.npc(0x200, {1500, 1000, 0});
    FakeActor& f = r.eng.a[0x100];
    f.rot = {0.1f, 0.2f, kPi / 2};
    f.weaponOut = true; f.aiming = true; f.sneak = false; f.vx = 100;
    f.target = 0x200;
    f.package = 0x01234567;
    r.frame(1000);
    live::ActorSnapshot s;
    CHECK(r.port.sample(a, s));
    CHECK(s.netId == a && s.cell == 1);
    CHECK(s.vel.x == 0);                 // first sample: no history
    f.pos.x += 70;                       // walked 70 units in 0.5 s
    r.frame(500);
    CHECK(r.port.sample(a, s));
    CHECK_MSG(std::fabs(s.vel.x - 140) < 1.0f, "vel.x=" + std::to_string(s.vel.x));
    CHECK(std::fabs(s.rot.x - 0.1f) < 1e-4f && std::fabs(s.rot.y - kPi / 2) < 1e-4f && std::fabs(s.rot.z - 0.2f) < 1e-4f);
    CHECK(s.anim.weaponDrawn && s.anim.aim && !s.anim.sneak);
    CHECK_MSG(s.anim.locomotion == 1, "loco=" + std::to_string(s.anim.locomotion));    // walking forward (east while facing east)
    CHECK(s.ai.combatTarget == b && s.ai.alert == 3 && s.ai.package == 0x234567);
    CHECK(s.avDeltas.size() == watchedAvs().size());
    bool health = false;
    for (auto& v : s.avDeltas) if (v.av == kAvHealth && v.value == 100) health = true;
    CHECK(health);
    f.sneak = true; f.weaponOut = false; f.target = 0;
    r.frame(100);
    CHECK(r.port.sample(a, s) && s.anim.sneak && !s.anim.weaponDrawn && s.ai.alert == 0 && s.ai.combatTarget == 0);
    CHECK(!r.port.sample(555, s));
}

TEST(port, a_teleport_is_not_a_velocity) {
    PortRig r;
    const NetId a = r.npc(0x100);
    live::ActorSnapshot s;
    r.frame(1000);
    CHECK(r.port.sample(a, s));
    r.eng.a[0x100].pos.x += 5000;
    r.frame(100);
    CHECK(r.port.sample(a, s) && s.vel.x == 0);
}

TEST(port, the_animation_sampler_gets_the_last_word) {
    PortRig r;
    const NetId a = r.npc(0x100);
    registry().animSampler = [](RefId ref, live::ActorSnapshot& s) { s.anim.jump = ref == 0x100; s.anim.reload = true; };
    live::ActorSnapshot s;
    CHECK(r.port.sample(a, s) && s.anim.jump && s.anim.reload);
}

// ---- actor values ----------------------------------------------------------------------------------------------------------------------

TEST(port, applied_values_are_written_only_when_they_differ_and_health_never_reaches_zero) {
    PortRig r;
    const NetId id = r.npc(0x100);
    r.port.applyAv(id, {{kAvHealth, 40}});
    CHECK(r.eng.a[0x100].av[kAvHealth] == 40 && r.eng.count("forceav", 0x100) == 1);
    r.port.applyAv(id, {{kAvHealth, 40}});
    CHECK(r.eng.count("forceav", 0x100) == 1);                 // equal: no line
    r.port.applyAv(id, {{kAvHealth, 0}});
    CHECK_MSG(r.eng.a[0x100].life == 0 && r.eng.a[0x100].av[kAvHealth] == 1, "life=" + std::to_string(r.eng.a[0x100].life));
    r.port.applyAv(id, {{kAvLeftLeg, 0}});                       // a crippled limb is a legitimate zero
    CHECK(r.eng.a[0x100].av[kAvLeftLeg] == 0 && r.eng.a[0x100].life == 0);
}

// The audit of a copy's mirrored values (ActorSync::auditAvs): the value is written when the owner changes it, and an engine that keeps an actor's damage elsewhere
// while it is not fully loaded gives the copy another health once it loads (vats room behind bad_wifi: -55 for the owner's 5). Health only, one write when it differs.
TEST(port, the_audit_brings_a_drifted_health_back_and_leaves_an_equal_one_alone) {
    PortRig r;
    const NetId id = r.npc(0x100);
    r.port.applyAv(id, {{kAvHealth, 5}});
    const int lines = r.eng.count("forceav", 0x100);
    CHECK(r.port.auditAv(id, {{kAvHealth, 5}}) == 0 && r.eng.count("forceav", 0x100) == lines);   // equal: nothing written
    r.eng.a[0x100].av[kAvHealth] = -55;                                                              // the engine moved the damage when the actor loaded
    CHECK(r.port.auditAv(id, {{kAvHealth, 5}}) == 1);                                                // twin 77: seen, not written
    CHECK_MSG(r.eng.a[0x100].av[kAvHealth] == 5, std::to_string(r.eng.a[0x100].av[kAvHealth]));
    r.eng.a[0x100].av[kAvHealth] = 50;
    CHECK(r.port.auditAv(id, {{kAvHealth, -55}}) == 1 && r.eng.a[0x100].av[kAvHealth] == 1);       // a dying owner: the copy is floored at 1, death is its own message
    r.eng.a[0x100].av[kAvLeftLeg] = 77;
    CHECK(r.port.auditAv(id, {{kAvLeftLeg, 0}}) == 0 && r.eng.a[0x100].av[kAvLeftLeg] == 77);     // limbs are not audited
}

TEST(port, place_puts_the_authoritative_state_in_completely) {
    PortRig r;
    const NetId id = r.npc(0x100, {1000, 1000, 0});
    live::ActorSnapshot s;
    s.netId = id; s.cell = 1; s.pos = {5, 6, 7}; s.rot.y = 0.5f;
    s.avDeltas = {{kAvHealth, 33}, {kAvHead, 0}};
    s.ai.combatTarget = 99;
    r.port.drive(id, interpAt({1, 1, 1}));
    r.port.place(id, s);
    CHECK(r.eng.did("setpos:256:xyz") && std::fabs(r.eng.a[0x100].rot.z - 0.5f) < 1e-4f);
    CHECK(r.eng.a[0x100].av[kAvHealth] == 33 && r.eng.a[0x100].av[kAvHead] == 0);
    CHECK(!r.port.pendingDrive(id));                           // an older wanted pose must not overwrite the placed state
    CHECK(r.port.stats().places == 1);
}

// ---- protection --------------------------------------------------------------------------------------------------------------------------

TEST(port, protection_sets_the_reference_flag_and_gives_it_back) {
    PortRig r;
    const NetId id = r.npc(0x100, {1000, 1000, 0}, 0x500, true);
    r.port.setProtected(id, true);
    CHECK(r.eng.a[0x100].essentialRef && r.led.has(LedgerKind::Essential, 0x100) && r.port.isProtected(id));
    r.port.setProtected(id, true);
    CHECK(r.eng.count("essential1", 0x100) == 1);
    r.port.setProtected(id, false);
    CHECK(!r.eng.a[0x100].essentialRef && !r.led.has(LedgerKind::Essential, 0x100) && !r.port.isProtected(id));
}

TEST(port, a_naturally_essential_actor_keeps_its_own_flag) {
    PortRig r;
    const NetId id = r.npc(0x100, {1000, 1000, 0}, 0x500, true);
    r.eng.a[0x100].baseEssential = true;
    r.port.setProtected(id, true);
    r.port.setProtected(id, false);
    CHECK_MSG(r.eng.count("essential1", 0x100) == 0 && r.eng.count("essential0", 0x100) == 0, "the port touched an essential actor's flag");
    CHECK(r.led.empty());
}

// ---- stand-ins ---------------------------------------------------------------------------------------------------------------------------

TEST(port, a_stand_in_is_a_restrained_clone_at_the_remote_pose_and_is_recorded) {
    PortRig r;
    r.eng.addNpc(0x300, 0x500, {0, 0, 0});     // some other ref in the cell to anchor on
    ActorMeta m;
    m.ref = fnvcoop::FormRef("falloutnv.esm", 0x777);
    m.netId = deriveNetId(m.ref);
    m.base = fnvcoop::FormRef("falloutnv.esm", 0x500);
    r.eng.form("falloutnv.esm", 0x500);
    live::ActorSnapshot at;
    at.cell = 1; at.pos = {400, 500, 600}; at.rot.y = 0.75f;
    at.avDeltas = {{kAvHealth, 61}};
    CHECK(r.port.spawnStandIn(m, at));
    const RefId c = r.port.activeRef(m.netId);
    CHECK(c != 0 && (c >> 24) == 0xFF && r.port.loaded(m.netId) && r.port.isStandInRef(c) && r.port.isOurClone(c));
    CHECK(r.eng.a[c].restrained);
    CHECK(std::fabs(r.eng.a[c].pos.x - 400) < 0.01f && std::fabs(r.eng.a[c].pos.z - 600) < 0.01f && std::fabs(r.eng.a[c].rot.z - 0.75f) < 1e-4f);
    CHECK(r.eng.a[c].av[kAvHealth] == 61);
    CHECK(r.led.has(LedgerKind::StandIn, c) && r.led.has(LedgerKind::Restrained, c));
    CHECK(r.tracked.size() == 1 && r.tracked[0].standIn && r.tracked[0].ref == c && r.tracked[0].netId == m.netId);
    CHECK(r.port.standInCount() == 1 && r.port.stats().standInsMade == 1);
    CHECK(r.port.spawnStandIn(m, at) && r.eng.countOp("placeatme") == 1);      // already there: no second clone
    // removal deletes it and clears the ledger
    r.port.removeStandIn(m.netId);
    CHECK(r.eng.a[c].deleted && r.eng.a[c].disabled);
    CHECK(!r.led.has(LedgerKind::StandIn, c) && !r.led.has(LedgerKind::Restrained, c));
    CHECK(!r.port.loaded(m.netId) && r.port.standInCount() == 0);
    CHECK(r.lost.size() == 1 && r.lost[0].standIn);
    r.port.removeStandIn(m.netId);                                                // idempotent
    CHECK(r.lost.size() == 1);
}

// In game 2026-10-22 three client crashes ran the engine's own "SetPos X" handler on a stand-in's id AFTER the stand-in was removed: the id
// had been recycled by a NavMesh and the handler called through text (EIP 6C676E61 = "angl"). The engine can keep a SetPos for a created
// ref queued, so a removed clone is left alone for deleteQuietMs after its last write. Twin 73 deletes it at once.
TEST(port, a_removed_stand_in_is_deleted_only_after_the_quiet_period_and_never_written_to_again) {
    Config qc = quietRigConfig();
    qc.deleteQuietMs = 3000;       // longer than the 2 s sweep, so the sweep gets its chance to jump the queue
    PortRig r(qc);
    r.eng.addNpc(0x300, 0x500, {0, 0, 0});
    ActorMeta m;
    m.ref = fnvcoop::FormRef("falloutnv.esm", 0x777);
    m.netId = deriveNetId(m.ref);
    m.base = fnvcoop::FormRef("falloutnv.esm", 0x500);
    r.eng.form("falloutnv.esm", 0x500);
    live::ActorSnapshot at;
    at.cell = 1; at.pos = {400, 500, 600};
    CHECK(r.port.spawnStandIn(m, at));
    const RefId c = r.port.activeRef(m.netId);
    r.frame();
    r.port.drive(m.netId, interpAt({450, 500, 600}));
    r.port.flush();
    CHECK(std::fabs(r.eng.a[c].pos.x - 450) < 0.01f);                    // the clone was being driven
    const int writesBefore = r.eng.count("setpos", c);
    r.port.removeStandIn(m.netId);
    CHECK(!r.port.loaded(m.netId) && r.port.standInCount() == 0);        // gone for ActorSync at once
    CHECK(!r.eng.a[c].deleted && !r.eng.a[c].disabled);                  // but not touched in the engine yet
    CHECK(r.led.has(LedgerKind::StandIn, c) && r.led.has(LedgerKind::Restrained, c));   // a crash or a save here still finds it
    r.port.drive(m.netId, interpAt({600, 500, 600}));                    // a late drive for the removed actor changes nothing
    for (int i = 0; i < 12; ++i) r.step(40);                             // 480 ms
    CHECK(!r.eng.a[c].deleted && r.eng.count("setpos", c) == writesBefore);
    r.step(2000);                                                        // a sweep runs in here (every 2 s): it must not jump the queue
    CHECK(!r.eng.a[c].deleted && r.led.has(LedgerKind::StandIn, c));
    r.step(1000);                                                        // 3.5 s after the removal
    CHECK(r.eng.a[c].deleted && r.eng.a[c].disabled);
    CHECK(!r.led.has(LedgerKind::StandIn, c) && !r.led.has(LedgerKind::Restrained, c));
    CHECK(r.eng.count("setpos", c) == writesBefore);                     // the last write was before the removal, and nothing came after
}

TEST(port, a_save_or_the_end_of_a_session_deletes_a_waiting_clone_at_once) {
    PortRig r(quietRigConfig());
    r.eng.addNpc(0x300, 0x500, {0, 0, 0});
    ActorMeta m;
    m.ref = fnvcoop::FormRef("falloutnv.esm", 0x777);
    m.netId = deriveNetId(m.ref);
    m.base = fnvcoop::FormRef("falloutnv.esm", 0x500);
    r.eng.form("falloutnv.esm", 0x500);
    live::ActorSnapshot at;
    at.cell = 1; at.pos = {400, 500, 600};
    CHECK(r.port.spawnStandIn(m, at));
    const RefId c = r.port.activeRef(m.netId);
    r.port.removeStandIn(m.netId);
    CHECK(!r.eng.a[c].deleted);
    r.port.beforeSave();                                                 // the save must not contain a clone
    CHECK(r.eng.a[c].deleted && !r.led.has(LedgerKind::StandIn, c));
    // a clone still waiting when the session ends goes too
    r.step(3500);                                                        // past the save's no-clone grace
    CHECK(r.port.spawnStandIn(m, at));
    const RefId c2 = r.port.activeRef(m.netId);
    r.port.removeStandIn(m.netId);
    CHECK(!r.eng.a[c2].deleted);
    r.port.releaseAll();
    CHECK(r.eng.a[c2].deleted && r.led.empty());
}

TEST(port, inventory_goes_through_the_providers_for_a_bound_live_actor_only) {
    PortRig r;
    const NetId id = r.npc(0x100);
    InvList stored{{fnvcoop::FormRef("falloutnv.esm", 0x15169), 2}};
    int reads = 0, writes = 0;
    registry().invRead = [&](NetId n, InvList& out) { ++reads; out = stored; return n != 0; };
    registry().invWrite = [&](NetId, const InvList& w) { ++writes; stored = w; return true; };
    InvList got;
    CHECK(r.port.sampleInventory(id, got) && got.size() == 1 && got[0].count == 2 && reads == 1);
    CHECK(r.port.applyInventory(id, InvList{{fnvcoop::FormRef("falloutnv.esm", 0x15169), 1}}) && stored[0].count == 1 && writes == 1);
    CHECK(!r.port.sampleInventory(deriveNetId(fnvcoop::FormRef("falloutnv.esm", 0x999)), got) && reads == 1);    // not bound: no engine read
    CHECK(!r.port.applyInventory(deriveNetId(fnvcoop::FormRef("falloutnv.esm", 0x999)), stored) && writes == 1);
    r.port.kill(id);
    CHECK(!r.port.applyInventory(id, stored) && writes == 1);                                                      // a dying actor's contents are the loot service's
    registry().invRead = nullptr;
    registry().invWrite = nullptr;
    CHECK(!r.port.sampleInventory(id, got));                                                                       // no provider: nothing is synced
}

TEST(port, equipment_goes_through_the_providers_for_a_bound_live_actor_only) {
    PortRig r;
    const NetId id = r.npc(0x100);
    EquipList worn{fnvcoop::FormRef("falloutnv.esm", 0x8ED04)};
    int reads = 0, writes = 0;
    registry().equipRead = [&](NetId n, EquipList& out) { ++reads; out = worn; return n != 0; };
    registry().equipWrite = [&](NetId, const EquipList& w) { ++writes; worn = w; return true; };
    EquipList got;
    CHECK(r.port.sampleEquipment(id, got) && got.size() == 1 && got[0].localId == 0x8ED04 && reads == 1);
    CHECK(r.port.applyEquipment(id, EquipList{fnvcoop::FormRef("falloutnv.esm", 0x1234)}) && worn[0].localId == 0x1234 && writes == 1);
    CHECK(!r.port.sampleEquipment(deriveNetId(fnvcoop::FormRef("falloutnv.esm", 0x999)), got) && reads == 1);   // not bound: no engine read
    CHECK(!r.port.applyEquipment(deriveNetId(fnvcoop::FormRef("falloutnv.esm", 0x999)), worn) && writes == 1);
    registry().equipWrite = [&](NetId, const EquipList&) { ++writes; return false; };                              // the engine says "not yet": the caller hears it
    CHECK(!r.port.applyEquipment(id, worn) && writes == 2);
    r.port.kill(id);
    CHECK(!r.port.applyEquipment(id, worn) && writes == 2);                                                      // a dying actor is not dressed
    registry().equipRead = nullptr;
    registry().equipWrite = nullptr;
    CHECK(!r.port.sampleEquipment(id, got));                                                                     // no provider: nothing is synced
}

TEST(port, a_part_written_through_the_generic_bridge_reaches_the_equipment_provider) {
    PortRig r;
    const NetId id = r.npc(0x100);
    EquipList worn;
    registry().equipRead = [&](NetId, EquipList& out) { out = worn; return true; };
    registry().equipWrite = [&](NetId, const EquipList& w) { worn = w; return true; };
    Bytes b;
    CHECK(r.port.readPart(id, PartKind::Equipment, b) && b == encodeEquipment({}));
    CHECK(r.port.writePart(id, PartKind::Equipment, encodeEquipment({fnvcoop::FormRef("falloutnv.esm", 0x8ED04)})) && worn.size() == 1);
    CHECK(r.port.readPart(id, PartKind::Equipment, b) && b == encodeEquipment(worn));
    CHECK(!r.port.writePart(id, PartKind::Equipment, Bytes{0xFF, 0xFF, 0xFF, 0xFF, 0xFF}));                        // garbage is refused, nothing is written
    CHECK(!r.port.readPart(id, PartKind::Inventory, b));                                                         // no inventory provider in this rig
}

TEST(port, no_clone_when_the_real_actor_is_here_and_none_beyond_the_cap) {
    Config c = rigConfig();
    c.maxStandIns = 2;
    PortRig r(c);
    const NetId real = r.npc(0x100);
    r.eng.form("falloutnv.esm", 0x500);
    ActorMeta m = r.metaFor(0x100);
    live::ActorSnapshot at;
    at.cell = 1; at.pos = {1, 2, 3};
    CHECK(!r.port.spawnStandIn(m, at));                       // the real one is bound
    CHECK(r.eng.countOp("placeatme") == 0);
    (void)real;
    for (uint32_t i = 0; i < 3; ++i) {
        ActorMeta x;
        x.ref = fnvcoop::FormRef("falloutnv.esm", 0x900 + i);
        x.netId = deriveNetId(x.ref);
        x.base = fnvcoop::FormRef("falloutnv.esm", 0x500);
        const bool ok = r.port.spawnStandIn(x, at);
        CHECK(ok == (i < 2));
    }
    CHECK(r.port.standInCount() == 2 && r.port.stats().standInsRefused == 1);
}

TEST(port, a_stand_in_that_cannot_be_made_backs_off_instead_of_hammering_the_engine) {
    PortRig r;
    r.eng.noAnchor = true;
    r.eng.form("falloutnv.esm", 0x500);
    ActorMeta m;
    m.ref = fnvcoop::FormRef("falloutnv.esm", 0x777);
    m.netId = deriveNetId(m.ref);
    m.base = fnvcoop::FormRef("falloutnv.esm", 0x500);
    live::ActorSnapshot at;
    at.cell = 1;
    CHECK(!r.port.spawnStandIn(m, at));
    const int asked = r.eng.anchorCalls;
    r.frame(100);
    CHECK(!r.port.spawnStandIn(m, at));
    CHECK(r.eng.anchorCalls == asked);                       // inside the 500 ms back-off: the engine is not even asked
    r.eng.noAnchor = false;
    r.frame(600);
    CHECK(r.port.spawnStandIn(m, at));
    // a base form that is not loaded here is a refusal, not a crash
    ActorMeta bad = m;
    bad.ref = fnvcoop::FormRef("falloutnv.esm", 0x778);
    bad.netId = deriveNetId(bad.ref);
    bad.base = fnvcoop::FormRef("missing.esp", 5);
    CHECK(!r.port.spawnStandIn(bad, at));
}

// ---- leveled pick ----------------------------------------------------------------------------------------------------------------------------

TEST(port, a_different_pick_replaces_the_body_with_a_clone_and_disables_the_real_actor) {
    PortRig r;
    const NetId id = r.npc(0x100, {1000, 1000, 0}, 0x500);
    r.port.setAi(id, false);
    const uint32_t pickId = r.eng.form("falloutnv.esm", 0x600);
    r.port.setBase(id, fnvcoop::FormRef("falloutnv.esm", 0x600));
    const RefId c = r.port.activeRef(id);
    CHECK(c != 0x100 && c != 0 && r.eng.a[c].base == pickId);
    CHECK(r.eng.a[0x100].disabled && r.led.has(LedgerKind::Disabled, 0x100) && r.led.has(LedgerKind::StandIn, c));
    CHECK(r.eng.a[c].restrained && r.port.netOfRef(c) == id && r.port.isOurClone(c) && !r.port.isStandInRef(c));
    CHECK(std::fabs(r.eng.a[c].pos.x - 1000) < 0.01f && std::fabs(r.eng.a[c].pos.y - 1000) < 0.01f);
    CHECK(r.port.realRef(id) == 0x100 && r.port.stats().baseSwaps == 1);
    CHECK(r.tracked.size() == 1 && !r.tracked[0].standIn && r.tracked[0].ref == c);
    // the same pick again is a no-op
    const int placed = r.eng.countOp("placeatme");
    r.port.setBase(id, fnvcoop::FormRef("falloutnv.esm", 0x600));
    CHECK(r.eng.countOp("placeatme") == placed);
    // driving moves the clone, not the disabled original
    r.frame();
    r.port.drive(id, interpAt({1300, 1000, 0}));
    r.port.flush();
    CHECK(std::fabs(r.eng.a[c].pos.x - 1300) < 0.01f && std::fabs(r.eng.a[0x100].pos.x - 1000) < 0.01f);
}

TEST(port, a_pick_that_matches_the_local_roll_changes_nothing_and_a_story_npc_is_never_swapped) {
    PortRig r;
    const NetId id = r.npc(0x100, {1000, 1000, 0}, 0x500);
    r.port.setBase(id, fnvcoop::FormRef("falloutnv.esm", 0x500));
    CHECK(r.eng.countOp("placeatme") == 0 && r.eng.log.empty());
    const NetId nm = r.npc(0x200, {2000, 1000, 0}, 0x510, true);
    r.eng.form("falloutnv.esm", 0x620);
    r.port.setBase(nm, fnvcoop::FormRef("falloutnv.esm", 0x620));
    CHECK(r.eng.countOp("placeatme") == 0 && r.port.activeRef(nm) == 0x200);
    // an unknown pick form is counted
    r.port.setBase(id, fnvcoop::FormRef("missing.esp", 1));
    CHECK(r.port.stats().baseSwapFailed == 1 && r.port.activeRef(id) == 0x100);
}

TEST(port, losing_the_real_actor_takes_its_pick_clone_with_it) {
    PortRig r;
    const NetId id = r.npc(0x100, {1000, 1000, 0}, 0x500);
    r.eng.form("falloutnv.esm", 0x600);
    r.port.setBase(id, fnvcoop::FormRef("falloutnv.esm", 0x600));
    const RefId c = r.port.activeRef(id);
    r.port.unbindReal(id);
    CHECK(r.eng.a[c].deleted && !r.port.loaded(id) && !r.led.has(LedgerKind::StandIn, c));
    CHECK(!r.eng.a[0x100].disabled && !r.led.has(LedgerKind::Disabled, 0x100));       // the real actor is not left hidden
}

TEST(port, a_clone_the_engine_would_not_delete_is_swept_later) {
    PortRig r;
    const NetId id = r.npc(0x100, {1000, 1000, 0}, 0x500);
    r.eng.form("falloutnv.esm", 0x600);
    r.port.setBase(id, fnvcoop::FormRef("falloutnv.esm", 0x600));
    const RefId c = r.port.activeRef(id);
    r.eng.refuseOps.insert("delete");
    r.port.unbindReal(id);
    CHECK(!r.eng.a[c].deleted && r.led.has(LedgerKind::StandIn, c));     // still on record
    r.eng.refuseOps.clear();
    r.step(2500);                                                         // the sweep runs every 2 s
    CHECK(r.eng.a[c].deleted && !r.led.has(LedgerKind::StandIn, c));
}

// ---- death -------------------------------------------------------------------------------------------------------------------------------------

TEST(port, kill_kills_once_reports_it_and_retries_a_refused_line) {
    PortRig r;
    const NetId id = r.npc(0x100);
    r.port.kill(id);
    CHECK(r.eng.a[0x100].life == 2 && r.eng.count("kill", 0x100) == 1);
    CHECK(r.died.size() == 1 && r.died[0].fromNetwork && r.died[0].authorised && r.died[0].ref == 0x100);
    r.step(250);
    CHECK(r.eng.count("kill", 0x100) == 1);           // confirmed dead: no repeat
    const NetId id2 = r.npc(0x200);
    r.eng.refuseOps.insert("kill");
    r.port.kill(id2);
    for (int i = 0; i < 30; ++i) r.step(250);
    CHECK_MSG(r.eng.count("refused-kill", 0x200) == 5, "tries=" + std::to_string(r.eng.count("refused-kill", 0x200)));
    r.eng.refuseOps.clear();
    for (int i = 0; i < 5; ++i) r.step(250);
    CHECK(r.eng.a[0x200].life == 0);                  // gave up after five tries; nothing keeps hammering
}

// G04: a copy that dies because a REMOTE player's shot killed the actor on its owner's machine is killed as that player's puppet (the engine stores the killer
// before OnDeath, `IsKiller player` is true for the guest); a copy nobody remote shot is killed plainly. A refused first line is retried the same way.
TEST(port, a_copy_killed_by_a_remote_players_shot_is_credited_to_that_puppet) {
    PortRig r;
    const NetId id = r.npc(0x100);
    const NetId other = r.npc(0x200);
    registry().killerOf = [](RefId victim) -> RefId { return victim == 0x100 ? 0x2001u : 0u; };
    r.port.kill(id);
    CHECK(r.eng.a[0x100].life == 2 && r.eng.count("killby", 0x100) == 1 && r.eng.count("kill", 0x100) == 0);
    r.port.kill(other);
    CHECK(r.eng.a[0x200].life == 2 && r.eng.count("killby", 0x200) == 0 && r.eng.count("kill", 0x200) == 1);
    const NetId retry = r.npc(0x300);
    registry().killerOf = [](RefId) -> RefId { return 0x2002u; };
    r.eng.refuseOps.insert("killby");
    r.port.kill(retry);
    r.eng.refuseOps.clear();
    r.step(250);
    CHECK(r.eng.a[0x300].life == 2 && r.eng.count("killby", 0x300) == 1);   // the retry keeps the credit
}

TEST(port, a_killed_copy_is_not_driven_or_fed_values_and_revive_undoes_it) {
    PortRig r;
    const NetId id = r.npc(0x100, {1000, 1000, 0});
    r.port.setAi(id, false);
    r.port.kill(id);
    r.port.drive(id, interpAt({1500, 1000, 0}));
    r.port.applyAv(id, {{kAvHealth, 80}});
    r.step(250);
    CHECK(std::fabs(r.eng.a[0x100].pos.x - 1000) < 0.01f && r.eng.a[0x100].av[kAvHealth] == 0);
    r.port.revive(id);
    CHECK(r.eng.a[0x100].life == 0 && r.eng.count("resurrect", 0x100) == 1);
    r.port.drive(id, interpAt({1500, 1000, 0}));
    r.step(250);
    CHECK(std::fabs(r.eng.a[0x100].pos.x - 1500) < 0.01f);
}

// ---- ledger-driven clean-up ------------------------------------------------------------------------------------------------------------------

TEST(port, releasing_the_session_undoes_everything_the_port_did) {
    PortRig r;
    const NetId a = r.npc(0x100), b = r.npc(0x200, {2000, 1000, 0}, 0x510, true);
    r.port.setAi(a, false);
    r.port.setProtected(b, true);
    r.eng.form("falloutnv.esm", 0x600);
    r.port.setBase(a, fnvcoop::FormRef("falloutnv.esm", 0x600));
    const RefId clone = r.port.activeRef(a);
    r.port.releaseAll();
    CHECK(!r.eng.a[0x100].restrained && !r.eng.a[0x100].disabled && r.eng.a[clone].deleted);
    CHECK(!r.eng.a[0x200].essentialRef);
    CHECK(r.led.empty());
    CHECK(r.port.ids().empty());
    CHECK(r.store.has && r.store.text == "nvcoop-actors-ledger 1\n");      // the file says so too
}

TEST(port, entries_left_by_an_earlier_run_are_undone_and_this_runs_entries_are_not) {
    PortRig r;
    // the previous run (a save, a crash) left these in the file
    r.eng.addNpc(0x100, 0x500, {0, 0, 0}); r.eng.a[0x100].restrained = true;
    r.eng.addNpc(0x200, 0x500, {0, 0, 0}); r.eng.a[0x200].disabled = true;
    r.eng.addNpc(0x300, 0x500, {0, 0, 0}); r.eng.a[0x300].essentialRef = true;
    r.eng.addNpc(0xFF000009, 0x500, {0, 0, 0}); r.eng.a[0xFF000009].restrained = true;
    r.led.add(LedgerKind::Restrained, 0x100);
    r.led.add(LedgerKind::Disabled, 0x200);
    r.led.add(LedgerKind::Essential, 0x300);
    r.led.add(LedgerKind::StandIn, 0xFF000009);
    r.led.add(LedgerKind::Restrained, 0x7777);               // a ref that is not loaded now: stays on record
    // this run's own change
    const NetId mine = r.npc(0x400);
    r.port.setAi(mine, false);
    CHECK(r.port.staleCount() == 5);
    CHECK(r.port.undoStale() == 4);
    CHECK(!r.eng.a[0x100].restrained && !r.eng.a[0x200].disabled && !r.eng.a[0x300].essentialRef && r.eng.a[0xFF000009].deleted);
    CHECK(r.eng.a[0x400].restrained && r.led.has(LedgerKind::Restrained, 0x400));          // not touched
    CHECK(r.led.has(LedgerKind::Restrained, 0x7777) && r.port.staleCount() == 1);
    // the ref shows up later: the next sweep catches it
    r.eng.addNpc(0x7777, 0x500, {0, 0, 0}); r.eng.a[0x7777].restrained = true;
    CHECK(r.port.undoStale() == 1 && !r.eng.a[0x7777].restrained);
}

// After a load or in a new process the engine reuses created-ref ids: a stale stand-in / restrained entry for 0xFF...... may name an
// unrelated object now. It must be dropped, never deleted or un-restrained (deleting one in the middle of a load hung the host).
TEST(port, a_reused_created_ref_id_from_an_earlier_run_is_dropped_not_touched) {
    PortRig r;
    r.eng.addNpc(0xFF000011, 0x600, {0, 0, 0});                  // someone else's new actor took the id: not restrained
    r.eng.addNpc(0xFF000012, 0x600, {0, 0, 0});
    r.led.add(LedgerKind::StandIn, 0xFF000011);
    r.led.add(LedgerKind::Restrained, 0xFF000011);
    r.led.add(LedgerKind::Restrained, 0xFF000012);
    r.led.add(LedgerKind::StandIn, 0xFF000013);                  // the id is free now (nothing there): dropped as well
    const uint64_t linesBefore = r.eng.lines;
    CHECK(r.port.undoStale() == 0);
    CHECK(!r.eng.a[0xFF000011].deleted && !r.eng.a[0xFF000012].deleted);       // not touched
    CHECK(!r.led.has(LedgerKind::StandIn, 0xFF000011) && !r.led.has(LedgerKind::Restrained, 0xFF000011));
    CHECK(!r.led.has(LedgerKind::Restrained, 0xFF000012) && !r.led.has(LedgerKind::StandIn, 0xFF000013));
    CHECK(r.port.stats().ledgerDropped == 4);
    CHECK(r.eng.lines == linesBefore);                            // not a single console line ran
}

TEST(port, a_world_change_forgets_the_directory_without_touching_the_new_world) {
    PortRig r;
    const NetId a = r.npc(0x100);
    r.port.setAi(a, false);
    const uint64_t lines = r.eng.lines;
    r.port.forgetWorld();
    CHECK(r.port.ids().empty() && r.eng.lines == lines);
    CHECK(r.led.has(LedgerKind::Restrained, 0x100));          // the flag is still on record ...
    CHECK(r.port.staleCount() == 1);                          // ... and now counts as left behind
}

TEST(port, a_save_removes_pick_clones_and_blocks_new_ones_for_the_grace_period) {
    PortRig r;
    const NetId id = r.npc(0x100, {1000, 1000, 0}, 0x500);
    r.eng.form("falloutnv.esm", 0x600);
    r.port.setBase(id, fnvcoop::FormRef("falloutnv.esm", 0x600));
    const RefId c = r.port.activeRef(id);
    const std::vector<NetId> reverted = r.port.beforeSave();
    CHECK(reverted.size() == 1 && reverted[0] == id);
    CHECK(r.eng.a[c].deleted && !r.eng.a[0x100].disabled && r.port.activeRef(id) == 0x100);
    CHECK(r.led.empty() && r.store.text == "nvcoop-actors-ledger 1\n");
    r.port.setBase(id, fnvcoop::FormRef("falloutnv.esm", 0x600));             // inside the grace: refused
    CHECK(r.port.activeRef(id) == 0x100);
    ActorMeta m;
    m.ref = fnvcoop::FormRef("falloutnv.esm", 0x777); m.netId = deriveNetId(m.ref); m.base = fnvcoop::FormRef("falloutnv.esm", 0x500);
    live::ActorSnapshot at; at.cell = 1;
    CHECK(!r.port.spawnStandIn(m, at));
    r.frame(3500);
    CHECK(r.port.spawnStandIn(m, at));
}

TEST(port, the_ledger_file_follows_the_changes_when_the_frame_is_flushed) {
    PortRig r;
    const NetId a = r.npc(0x100);
    r.port.setAi(a, false);
    r.step(1200);
    CHECK(r.store.has && r.store.text.find("r 00000100") != std::string::npos);
    r.port.setAi(a, true);
    r.step(1200);
    CHECK(r.store.text.find("r 00000100") == std::string::npos);
    Config c = rigConfig();
    c.persistLedger = false;
    PortRig q(c);
    const NetId b = q.npc(0x100);
    q.port.setAi(b, false);
    q.step(1200);
    CHECK(!q.store.has);
}
