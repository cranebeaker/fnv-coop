// ActorFeed: what the engine has -> ActorSync. Scan, unload hysteresis, deaths, sessions, worlds, players, saves.
// Twins 41..57 live in actor_feed.cpp (FNV_MUT markers).
#include <string>

#include "feed_rig.h"
#include "minitest.h"

using namespace nvae_test;

namespace {
FeedRig& crowd(FeedRig& r, int n = 3) {
    for (int i = 0; i < n; ++i) r.eng.addNpc(0x100 + static_cast<RefId>(i), 0x500, {1000.0f + 200.0f * static_cast<float>(i), 1000, 0});
    return r;
}
}  // namespace

TEST(feed, loaded_npcs_are_tracked_and_the_player_is_not) {
    FeedRig r;
    crowd(r);
    r.eng.addNpc(0x103, 0x510, {1800, 1000, 0}).baseUnique = true;
    r.startSession();
    r.frame();
    // the sync tracks the 4 NPCs plus the host's own player record (since the entity rebuild every player is an ActorSync record; the host
    // pins its own each tick), while the feed itself never feeds the player
    CHECK_MSG(r.feed.fedCount() == 4 && r.sync->trackedCount() == 5,
              "fed " + std::to_string(r.feed.fedCount()) + ", tracked " + std::to_string(r.sync->trackedCount()));
    for (RefId ref : {0x100, 0x101, 0x102, 0x103}) CHECK(r.port.bound(r.id(static_cast<RefId>(ref))) && r.feed.fed(r.id(static_cast<RefId>(ref))));
    CHECK(!r.feed.fed(r.id(0x14)) && r.feed.netOfRef(0x100) == r.id(0x100));
    CHECK(r.tracked.size() == 4);
    int named = 0;
    for (auto& e : r.tracked) { CHECK(!e.standIn && !e.created); if (e.named) ++named; }
    CHECK(named == 1);
    CHECK(r.feed.stats().trackedNew == 4);
}

TEST(feed, corpses_disabled_deleted_and_player_like_refs_are_left_alone) {
    FeedRig r;
    r.eng.addNpc(0x100, 0x500, {1000, 1000, 0}).life = 2;                       // a corpse
    r.eng.addNpc(0x101, 0x500, {1100, 1000, 0}).disabled = true;
    r.eng.addNpc(0x102, 0x500, {1200, 1000, 0}).deleted = true;
    r.eng.addNpc(0x103, 0x500, {1300, 1000, 0}).player = true;                  // not the local player, but flagged as a player actor
    r.eng.addNpc(0x104, 0x500, {1400, 1000, 0});                                // the one real NPC
    r.startSession();
    r.frame();
    CHECK(r.feed.fedCount() == 1 && r.feed.fed(r.id(0x104)));
    // a disabled actor that gets enabled is picked up by the next scan
    r.eng.a[0x101].disabled = false;
    r.feed.markDirty();
    r.frame();
    CHECK(r.feed.fedCount() == 2 && r.feed.fed(r.id(0x101)));
}

TEST(feed, the_ports_own_clones_are_not_new_actors) {
    FeedRig r;
    crowd(r, 1);
    r.eng.form("falloutnv.esm", 0x500);
    r.startSession();
    // even if spawn-arb would give any created ref an identity, the port's own clone must not become an actor of its own
    registry().createdId = [](RefId ref, NetId& n, fnvcoop::FormRef& f) { n = ref; f = fnvcoop::FormRef("*created*", ref); return true; };
    ActorMeta m;
    m.ref = fnvcoop::FormRef("falloutnv.esm", 0x777);
    m.netId = deriveNetId(m.ref);
    m.base = fnvcoop::FormRef("falloutnv.esm", 0x500);
    live::ActorSnapshot at;
    at.cell = 1; at.pos = {1, 2, 3};
    r.frame();
    CHECK(r.port.spawnStandIn(m, at));
    r.feed.markDirty();
    r.frame();
    r.feed.markDirty();
    r.frame();
    CHECK_MSG(r.feed.fedCount() == 1, "fed " + std::to_string(r.feed.fedCount()));
}

TEST(feed, created_refs_need_a_host_issued_identity) {
    FeedRig r;
    r.eng.addNpc(0xFF000010, 0x500, {1000, 1000, 0});
    r.startSession();
    r.frame();
    CHECK(r.feed.fedCount() == 0 && r.feed.stats().createdSkipped >= 1);
    registry().createdId = [](RefId ref, NetId& n, fnvcoop::FormRef& f) {
        if (ref != 0xFF000010) return false;
        n = 777; f = fnvcoop::FormRef("*created*", 0xAA);
        return true;
    };
    r.feed.markDirty();
    r.frame();
    CHECK(r.feed.fedCount() == 1 && r.feed.fed(777) && r.port.bound(777));
    CHECK(r.tracked.size() == 1 && r.tracked[0].created && r.tracked[0].formRef.plugin == "*created*" && r.tracked[0].netId == 777);
}

TEST(feed, an_actor_sync_refuses_is_not_bound_or_driven) {
    FeedRig r;
    crowd(r, 1);
    r.startSession();
    ActorMeta other;                                   // the same placed ref already tracked under another id: a hash collision
    other.ref = r.eng.refForm(0x100);
    other.netId = 999;
    r.sync->track(other);
    r.frame();
    CHECK(r.feed.stats().trackFailed == 1 && r.feed.fedCount() == 0);
    CHECK(!r.port.bound(r.id(0x100)) && r.port.netOfRef(0x100) == 0);
}

TEST(feed, an_actor_missing_from_the_list_is_gone_only_when_the_engine_says_so) {
    FeedRig r;
    crowd(r, 2);
    r.startSession();
    r.frame();
    CHECK(r.feed.fedCount() == 2);
    // (a) the list was cut short, the actor is still there
    r.eng.hidden.insert(0x100);
    r.run(2000);
    CHECK(r.feed.fedCount() == 2 && r.lost.empty());
    r.eng.hidden.clear();
    // (b) one scan with the cell flipped away: not a loss
    r.eng.cells[1].loaded = false;
    r.feed.markDirty(); r.frame();
    r.eng.cells[1].loaded = true;
    r.feed.markDirty(); r.frame();
    CHECK(r.feed.fedCount() == 2 && r.lost.empty());
    // (c) really gone for two scans
    r.eng.cells[1].loaded = false;
    r.feed.markDirty(); r.frame();
    r.feed.markDirty(); r.frame();
    CHECK(r.feed.fedCount() == 0 && r.lost.size() == 2 && !r.port.bound(r.id(0x100)));
    CHECK(r.feed.stats().trackedGone == 2);
    // back again: tracked again
    r.eng.cells[1].loaded = true;
    r.feed.markDirty(); r.frame();
    CHECK(r.feed.fedCount() == 2 && r.tracked.size() == 4);
}

TEST(feed, a_deleted_actor_is_dropped_at_once) {
    FeedRig r;
    crowd(r, 1);
    r.startSession();
    r.frame();
    r.eng.a[0x100].deleted = true;
    r.feed.markDirty();
    r.frame();
    CHECK(r.feed.fedCount() == 0 && r.lost.size() == 1);
}

TEST(feed, scans_follow_the_interval_and_the_dirty_flag) {
    FeedRig r;
    crowd(r, 1);
    r.startSession();
    r.frame();
    const uint64_t base = r.feed.stats().scans;
    r.run(400);
    CHECK(r.feed.stats().scans == base);              // 500 ms interval
    r.run(200);
    CHECK(r.feed.stats().scans == base + 1);
    r.feed.markDirty();
    r.frame();
    CHECK(r.feed.stats().scans == base + 2);
}

TEST(feed, the_scan_cap_keeps_the_actors_nearest_the_player) {
    Config c = rigConfig();
    c.maxScanActors = 3;
    FeedRig r(true, c);
    r.eng.a[0x14].pos = {0, 0, 0};
    for (int i = 0; i < 6; ++i) r.eng.addNpc(0x100 + static_cast<RefId>(i), 0x500, {100.0f * static_cast<float>(6 - i), 0, 0});   // 0x105 is the nearest
    r.startSession();
    r.frame();
    CHECK(r.feed.fedCount() == 3);
    CHECK(r.feed.fed(r.id(0x105)) && r.feed.fed(r.id(0x104)) && r.feed.fed(r.id(0x103)) && !r.feed.fed(r.id(0x100)));
}

TEST(feed, the_local_player_is_fed_so_the_host_can_own_what_is_near) {
    FeedRig r;
    crowd(r, 2);
    r.startSession();
    r.run(1000);
    const OwnerInfo o = r.sync->ownerOf(r.id(0x100));
    CHECK(o.known && o.mine && o.owner == 0);
    CHECK(r.feed.stats().poseUpdates >= 10);
}

TEST(feed, adjacency_is_exterior_grid_distance_inside_one_worldspace) {
    FeedRig r;
    r.eng.cells[10] = FakeEngine::Cell{true, true, 3, 0, 0};
    r.eng.cells[11] = FakeEngine::Cell{true, true, 3, 2, -2};
    r.eng.cells[12] = FakeEngine::Cell{true, true, 3, 3, 0};
    r.eng.cells[13] = FakeEngine::Cell{true, true, 4, 0, 1};     // other worldspace
    r.eng.cells[14] = FakeEngine::Cell{true, false, 0, 0, 0};    // interior
    CHECK(r.feed.adjacent(10, 10) && r.feed.adjacent(10, 11) && r.feed.adjacent(11, 10));
    CHECK(!r.feed.adjacent(10, 12));
    CHECK(!r.feed.adjacent(10, 13));
    CHECK(!r.feed.adjacent(10, 14) && !r.feed.adjacent(14, 1) && r.feed.adjacent(14, 14));
    CHECK(!r.feed.adjacent(10, 999));
}

// ---- deaths --------------------------------------------------------------------------------------------------------------------------------

TEST(feed, an_owner_whose_actor_dies_in_the_engine_publishes_the_death) {
    FeedRig r;
    crowd(r, 2);
    r.startSession();
    r.run(1500);
    const NetId id = r.id(0x100);
    CHECK(r.sync->ownerOf(id).mine);
    r.eng.a[0x100].life = 2;
    r.eng.a[0x100].av[kAvHealth] = 0;
    r.run(300);
    CHECK(r.sync->isDead(id));
    CHECK(r.died.size() == 1 && r.died[0].mine && r.died[0].authorised && !r.died[0].fromNetwork && r.died[0].netId == id);
    CHECK(r.eng.count("resurrect", 0x100) == 0);
    CHECK(r.feed.stats().deathsReported == 1);
    r.run(500);
    CHECK(r.died.size() == 1);
}

// enemy_handoff, game 2026-10-22: the owner's copy of a ganger read dying (life 1) and half a second later restrained (5), which every reader
// counts as ALIVE, while the other machine had it dead. Publishing the death turns the AI off; SetRestrained on a corpse overwrote its life
// state. Twin 72 restrains the corpse like a living NPC.
TEST(feed, the_owners_copy_of_a_dead_actor_stays_dead_it_is_not_restrained_back_to_life) {
    FeedRig r;
    crowd(r, 2);
    r.startSession();
    r.run(1500);
    const NetId id = r.id(0x100);
    CHECK(r.sync->ownerOf(id).mine);
    const int restrainsBefore = r.eng.count("restrain", 0x100);
    r.eng.a[0x100].life = 1;                       // the engine's own death starts (dying)
    r.eng.a[0x100].av[kAvHealth] = 0;
    r.run(300);
    CHECK(r.sync->isDead(id) && r.feed.stats().deathsReported == 1);
    r.eng.a[0x100].life = 2;                       // the dying animation ends
    r.run(1500);
    CHECK(r.eng.a[0x100].life == 2);               // still dead: not 5
    CHECK(r.eng.count("restrain", 0x100) == restrainsBefore);
    CHECK(r.port.aiOff(id));                       // the port knows its AI is off
}

TEST(feed, a_story_npc_killed_by_the_engine_without_authority_is_revived) {
    FeedRig r;
    r.eng.addNpc(0x100, 0x500, {1000, 1000, 0}).baseUnique = true;
    r.startSession();
    r.run(1500);
    const NetId id = r.id(0x100);
    CHECK(r.sync->ownerOf(id).mine);
    r.eng.a[0x100].life = 2;
    r.run(300);
    CHECK(!r.sync->isDead(id) && r.eng.a[0x100].life == 0 && r.eng.count("resurrect", 0x100) >= 1);
    CHECK(r.feed.stats().deathsRefused >= 1 && r.died.empty());
}

TEST(feed, a_copy_that_died_locally_waits_for_the_owner_and_is_revived_when_no_death_comes) {
    FeedRig r(false);
    crowd(r, 1);
    r.startSession();
    r.run(300);
    const NetId id = r.id(0x100);
    CHECK(!r.sync->ownerOf(id).mine && r.feed.fed(id));
    r.eng.a[0x100].life = 2;
    r.run(1000);
    CHECK(r.eng.a[0x100].life == 2 && r.eng.count("resurrect", 0x100) == 0);     // inside the grace
    r.run(1000);
    CHECK(r.eng.a[0x100].life == 0 && r.eng.count("resurrect", 0x100) >= 1);
    CHECK(!r.sync->isDead(id) && r.feed.stats().deathsRefused == 1);
}

TEST(feed, a_copy_that_died_locally_stays_dead_when_the_owner_declares_the_death) {
    FeedRig r(false);
    crowd(r, 1);
    r.startSession();
    r.run(300);
    const NetId id = r.id(0x100);
    r.eng.a[0x100].life = 2;
    r.run(300);
    r.sync->onEffect(fnvcoop::KillActor{r.eng.refForm(0x100)});     // the authority speaks
    CHECK(r.sync->isDead(id));
    r.run(3000);
    CHECK(r.eng.a[0x100].life == 2 && r.eng.count("resurrect", 0x100) == 0 && r.feed.stats().deathsRefused == 0);
}

// ---- sessions, worlds, saves ----------------------------------------------------------------------------------------------------------------

TEST(feed, the_end_of_a_session_gives_every_npc_back_and_a_new_one_relearns_them_at_once) {
    FeedRig r(false);
    crowd(r, 3);
    r.startSession();
    r.run(300);
    CHECK(r.feed.fedCount() == 3);
    for (RefId ref : {0x100, 0x101, 0x102}) CHECK(r.eng.a[ref].restrained);          // not mine: the AI is off
    CHECK(!r.led.empty());
    r.endSession();
    r.frame();
    for (RefId ref : {0x100, 0x101, 0x102}) CHECK_MSG(!r.eng.a[ref].restrained, "actor " + std::to_string(ref) + " is still frozen");
    CHECK(r.led.empty() && r.feed.fedCount() == 0 && r.port.ids().empty());
    r.startSession();
    r.frame();                                                                          // one frame, not "after the next periodic scan"
    CHECK(r.feed.fedCount() == 3 && r.sync->trackedCount() == 3);
    CHECK(r.eng.a[0x100].restrained);
}

TEST(feed, nothing_is_scanned_without_a_session) {
    FeedRig r;
    crowd(r, 2);
    r.run(2000);
    CHECK(r.feed.stats().scans == 0 && r.feed.fedCount() == 0);
}

TEST(feed, an_actor_that_died_before_a_load_can_die_again_after_it) {
    // in game 2026-10-21: a gecko killed in one run was alive again after the reload, died again on the host, and stayed alive on the guest
    FeedRig r;
    crowd(r, 1);
    r.startSession();
    r.run(1500);
    NetId id = r.id(0x100);
    r.eng.a[0x100].life = 2;
    r.eng.a[0x100].av[kAvHealth] = 0;
    r.run(300);
    CHECK(r.sync->isDead(id) && r.feed.stats().deathsReported == 1);
    // the owner loads a save from before the death: the actor stands there alive
    r.eng.a[0x100].life = 0;
    r.eng.a[0x100].av[kAvHealth] = 100;
    r.eng.epoch = 2;
    r.frame();
    r.run(1500);
    id = r.id(0x100);
    CHECK_MSG(!r.sync->isDead(id), "the old world's death is still on record after the load");
    r.eng.a[0x100].life = 2;
    r.eng.a[0x100].av[kAvHealth] = 0;
    r.run(300);
    CHECK_MSG(r.feed.stats().deathsReported == 2, "the second death was not published: " + std::to_string(r.feed.stats().deathsReported));
    CHECK(r.sync->isDead(id));
}

TEST(feed, an_actor_a_script_resurrected_can_die_again_and_the_second_death_is_published) {
    // bench 2026-10-27 (real_fight): the harness resurrects the gangers with a console `Resurrect` before every fight; the feed kept "dead" on the
    // actor for the rest of the session, so the second death was never polled, never declared, and the other machines never heard of it
    FeedRig r;
    crowd(r, 1);
    r.startSession();
    r.run(1500);
    NetId id = r.id(0x100);
    r.eng.a[0x100].life = 2;
    r.eng.a[0x100].av[kAvHealth] = 0;
    r.run(300);
    CHECK(r.sync->isDead(id) && r.feed.stats().deathsReported == 1);
    // the story host resurrects it (ResurrectActor effect): ActorSync forgets the death, the port stands it up again
    r.sync->onEffect(fnvcoop::ResurrectActor{r.eng.refForm(0x100)});
    r.run(800);
    CHECK_MSG(r.eng.a[0x100].life == 0 && !r.sync->isDead(id), "the resurrect did not take");
    r.eng.a[0x100].life = 2;
    r.eng.a[0x100].av[kAvHealth] = 0;
    r.run(600);
    CHECK_MSG(r.feed.stats().deathsReported == 2, "the second death was not published: " + std::to_string(r.feed.stats().deathsReported));
    CHECK(r.sync->isDead(id));
}

TEST(feed, a_loaded_game_forgets_the_old_world_and_undoes_what_a_save_left_behind) {
    FeedRig r(false);
    crowd(r, 2);
    r.startSession();
    r.run(300);
    CHECK(r.eng.a[0x100].restrained);
    // the owner loads another game: new world, new refs. One restrained ref and one clone of the old run exist in the loaded save.
    r.eng.addNpc(0x300, 0x500, {500, 500, 0}).restrained = true;
    r.eng.addNpc(0xFF000009, 0x500, {600, 500, 0}).restrained = true;
    r.led.add(LedgerKind::Restrained, 0x300);
    r.led.add(LedgerKind::StandIn, 0xFF000009);
    r.eng.epoch = 2;
    r.frame();
    CHECK(r.feed.stats().worldChanges == 2);          // first sight of the world + the load
    CHECK(r.eng.count("unrestrain", 0x300) == 1 && r.eng.a[0xFF000009].deleted);       // undone (the scan of the same frame restrains 0x300 again as a tracked NPC)
    CHECK(!r.led.has(LedgerKind::StandIn, 0xFF000009) && r.feed.fedCount() >= 3);
    // menus: nothing happens while the world is not ready
    r.eng.ready = false;
    const uint64_t scans = r.feed.stats().scans;
    r.run(2000);
    CHECK(r.feed.stats().scans == scans);
}

TEST(feed, a_save_removes_clones_and_the_swap_is_redone_after_the_grace) {
    FeedRig r;
    crowd(r, 1);
    r.startSession();
    r.run(1500);
    const NetId id = r.id(0x100);
    r.eng.form("falloutnv.esm", 0x600);
    r.port.setBase(id, fnvcoop::FormRef("falloutnv.esm", 0x600));
    const RefId clone = r.port.activeRef(id);
    CHECK(clone != 0x100);
    r.sync->onNativeBase(id, fnvcoop::FormRef("falloutnv.esm", 0x600));              // ActorSync believes in the pick
    registry().beforeSave();
    CHECK(r.eng.a[clone].deleted && !r.eng.a[0x100].disabled && r.port.activeRef(id) == 0x100);
    r.run(1000);
    CHECK(r.feed.stats().rebased == 0);
    r.run(3000);
    CHECK(r.feed.stats().rebased == 1);
    CHECK(r.sync->baseOf(id) == fnvcoop::FormRef("falloutnv.esm", 0x500));           // ActorSync was told the native body is back
}

TEST(feed, refs_another_layer_drives_are_never_synced_as_npcs) {
    FeedRig r;
    crowd(r, 3);
    r.startSession();
    registry().exclude = [](RefId ref) { return ref == 0x101; };       // a player puppet made by the avatar layer
    r.frame();
    CHECK(r.feed.fedCount() == 2 && !r.feed.fed(r.id(0x101)) && r.feed.fed(r.id(0x100)));
    // an actor that becomes a puppet later is let go
    registry().exclude = [](RefId ref) { return ref == 0x101 || ref == 0x102; };
    r.feed.markDirty();
    r.frame();
    CHECK(r.feed.fedCount() == 1 && !r.feed.fed(r.id(0x102)) && r.lost.size() == 1 && !r.port.bound(r.id(0x102)));
}

TEST(feed, a_story_npcs_copy_that_is_dead_here_is_not_revived_it_can_only_be_a_host_kill) {
    FeedRig r(false);
    r.eng.addNpc(0x100, 0x500, {1000, 1000, 0}).baseUnique = true;
    r.startSession();
    r.run(300);
    r.eng.a[0x100].life = 2;
    r.run(4000);
    CHECK(r.eng.a[0x100].life == 2 && r.eng.count("resurrect", 0x100) == 0 && r.feed.stats().deathsRefused == 0);
}

TEST(feed, a_later_configure_call_reaches_the_running_feed_and_port) {
    FeedRig r;
    crowd(r, 1);
    r.startSession();
    r.frame();
    Config c = rigConfig();
    c.scanMs = 5000;
    c.maxLinesPerFrame = 3;
    configure(c);
    r.run(700);                                          // the scan that was already due under the old interval
    const uint64_t scans = r.feed.stats().scans;
    r.run(2000);
    CHECK(r.feed.stats().scans == scans);                // the new interval (5 s) is in force
    CHECK(r.feed.config().scanMs == 5000 && r.port.config().maxLinesPerFrame == 3);
}
