// Rejoin / late-join hardening: a client that drops mid-transaction and comes back after the host saved+loaded (epoch change), and a late
// joiner arriving during an open barrier. Full HostSession + ClientSession + RulesEngine over a lossy link.
#include <map>
#include <memory>
#include "fnvcoop/rules.h"
#include "fnvcoop/session.h"
#include "minitest.h"
#include "rejoin_rig.h"

using namespace fnvcoop;
using namespace rejoin;

namespace {

// Scenario: host saves, commits R1, client 2 is cut off right after R2, R3 happens, host loads the save (epoch change), R4 in the new
// epoch, client 2 comes back with a baseline. Every reward item must arrive at most once per player and everybody converges.
bool dropThenReloadScenario(uint64_t seed, std::string* why) {
    auto fail = [&](const char* w) { if (why) *why = w; return false; };
    Rig r(seed);
    r.addClient(1); r.addClient(2);
    r.joinAll();
    r.run(1200);
    if (!r.allLive()) return fail("not live after join");
    Bytes save = r.host.saveSidecar();
    r.reward(0x100); r.run(400);                                  // both clients get R1
    r.reward(0x101);
    r.drop(1);                                                    // client 2 vanishes with R2 still in flight
    r.reward(0x102);                                              // R3: only client 1 sees it
    r.run(400);
    uint32_t oldEpoch = r.host.epoch();
    if (!r.host.loadSidecar(save)) return fail("loadSidecar failed");
    if (r.host.epoch() != oldEpoch + 1) return fail("epoch not bumped");
    r.run(1500);                                                  // client 1 rebases onto the older timeline
    r.reward(0x103);                                              // R4 in the new epoch while client 2 is away
    r.run(400);
    r.reconnect(1);
    r.run(3000);
    if (!r.allLive()) return fail("not live after rejoin");
    for (auto& c : r.cl) {
        if (c->cs->state() != r.host.state()) return fail("client state != host state");
        if (c->cs->worldEpoch() != r.host.epoch()) return fail("client on another epoch");
        for (auto& kv : c->sink.items) if (kv.second > 1) return fail("duplicate reward");
    }
    if (r.cl[1]->sink.items.count(0x101) || r.cl[1]->sink.items.count(0x102)) return fail("old-epoch reward replayed to the returning client");
    if (r.cl[1]->sink.items[0x100] != 1) return fail("client 2 lost R1");
    r.reward(0x104); r.run(600);                                  // the new epoch works for both, exactly once
    for (auto& c : r.cl) if (c->sink.items[0x104] != 1) return fail("new-epoch reward not delivered exactly once");
    if (r.cl[0]->cs->state() != r.host.state() || r.cl[1]->cs->state() != r.host.state()) return fail("diverged after new reward");
    return true;
}

// Scenario: a client disconnects while txns are in flight (no host reload) and rejoins; replay must not duplicate rewards.
bool dropMidBatchScenario(uint64_t seed, std::string* why) {
    auto fail = [&](const char* w) { if (why) *why = w; return false; };
    Rig r(seed);
    r.addClient(1); r.addClient(2);
    r.joinAll(); r.run(1200);
    if (!r.allLive()) return fail("not live");
    for (uint32_t i = 0; i < 6; ++i) r.reward(0x200 + i);
    r.run(20);
    r.drop(1);
    for (uint32_t i = 6; i < 10; ++i) r.reward(0x200 + i);
    r.run(500);
    r.reconnect(1);
    r.run(3000);
    if (!r.allLive()) return fail("not live after rejoin");
    for (size_t ci = 0; ci < r.cl.size(); ++ci) {
        auto& c = r.cl[ci];
        if (c->cs->state() != r.host.state()) return fail("state diverged");
        for (auto& kv : c->sink.items) if (kv.second > 1) { if (why) *why = "duplicate reward item " + std::to_string(kv.first) + " x" + std::to_string(kv.second) + " on client " + std::to_string(ci) + " trail:"; for (auto& t : c->sink.trail) *why += " " + t; return false; }
    }
    // everything committed while client 2 was away is delivered by replay (same epoch): exactly once each
    for (uint32_t i = 6; i < 10; ++i) if (r.cl[1]->sink.items[0x200 + i] > 1) return fail("duplicate replay");
    return true;
}

const char* kVcg01 = R"({"schema":1,"quests":{
 "VCG01":{"tier":"T4","questline":"Main: Prologue","curated":true,"rules":[
   {"kind":"Barrier","params":{"step":"all-players-pass"},"origin":"curated"}]}}})";

struct BarrierResult { bool ok = true; std::string why; int applied = 0; };

// Late joiner arrives while the VCG01 chargen barrier is open (host passed, client 1 has not).
BarrierResult lateJoinBarrier(uint64_t seed, int variant) {
    BarrierResult res;
    auto fail = [&](const char* w) { res.ok = false; res.why = w; return res; };
    Rig r(seed);
    r.addClient(1); r.addClient(2);
    r.joinOnly(0); r.run(1200);                                   // only client 1 is in the session
    RulesDb db;
    if (!db.load(kVcg01)) return fail("rules");
    RulesEngine eng(r.host, db, resolveEdid);
    eng.applyStage = [&](const FormRef&, uint32_t) { ++res.applied; };
    r.host.onAction = [&](PlayerId s, IntentKind k, const FormRef& ref, uint32_t, uint32_t) { eng.handleAction(s, k, ref); };
    r.engine = &eng;
    FormRef q = QF("VCG01");
    if (eng.onStageRequested(q, 30, 0) != StageDecision::Deferred) return fail("not deferred");
    eng.onPlayerPassed(0, q);                                     // the host finished chargen; client 1 is still creating a character
    r.run(200);
    r.joinOnly(1); r.run(2500);                                   // client 2 joins late, mid-barrier
    if (res.applied != 0) return fail("released while two players were still creating characters");
    if (!r.cl[1]->cs->joined()) return fail("late joiner did not join");
    if (r.cl[1]->cs->state() != r.host.state()) return fail("late joiner has a different shadow state");
    if (variant == 0) {                                           // everybody passes
        r.cl[0]->cs->submitAction(IntentKind::ScenePass, q); r.run(300);
        if (res.applied != 0) return fail("released before the late joiner passed");
        r.cl[1]->cs->submitAction(IntentKind::ScenePass, q); r.run(600);
    } else if (variant == 1) {                                    // the late joiner leaves without passing
        r.cl[0]->cs->submitAction(IntentKind::ScenePass, q); r.run(300);
        if (res.applied != 0) return fail("released before the late joiner left");
        r.drop(1); r.run(600);
    } else {                                                      // the late joiner passes first, the old client later
        r.cl[1]->cs->submitAction(IntentKind::ScenePass, q); r.run(300);
        if (res.applied != 0) return fail("released with client 1 still pending");
        r.cl[0]->cs->submitAction(IntentKind::ScenePass, q); r.run(600);
    }
    if (res.applied != 1) return fail("stage not applied exactly once");
    if (eng.pendingDeferrals() != 0) return fail("barrier still pending");
    if (!r.host.state().barriers.at(RulesEngine::barrierId(q, 30)).released) return fail("barrier not released in shadow state");
    for (size_t i = 0; i < r.cl.size(); ++i) {
        if (variant == 1 && i == 1) continue;
        if (r.cl[i]->cs->state() != r.host.state()) return fail("client diverged");
        if (r.cl[i]->sink.count(23) > 1) return fail("BarrierRelease delivered twice");
    }
    return res;
}
}  // namespace

TEST(rejoin, client_dropped_mid_txn_rejoins_after_host_reload_without_duplicates) {
    for (uint64_t seed = 1; seed <= 6; ++seed) {
        std::string why;
        bool ok = dropThenReloadScenario(seed, &why);
        CHECK_MSG(ok, "seed " + std::to_string(seed) + ": " + why);
    }
}

TEST(rejoin, client_dropped_mid_batch_replays_missed_txns_exactly_once) {
    for (uint64_t seed = 1; seed <= 6; ++seed) {
        std::string why;
        bool ok = dropMidBatchScenario(seed, &why);
        CHECK_MSG(ok, "seed " + std::to_string(seed) + ": " + why);
    }
}

TEST(rejoin, late_joiner_during_open_barrier_everyone_passes) {
    for (uint64_t seed = 1; seed <= 4; ++seed) { auto b = lateJoinBarrier(seed, 0); CHECK_MSG(b.ok, b.why); }
}
TEST(rejoin, late_joiner_during_open_barrier_leaves_without_passing_does_not_stick_it) {
    for (uint64_t seed = 1; seed <= 4; ++seed) { auto b = lateJoinBarrier(seed, 1); CHECK_MSG(b.ok, b.why); }
}
TEST(rejoin, late_joiner_during_open_barrier_passes_first) {
    for (uint64_t seed = 1; seed <= 4; ++seed) { auto b = lateJoinBarrier(seed, 2); CHECK_MSG(b.ok, b.why); }
}

// ---- mutation twins: each re-introduces the bug and the scenario must notice ----
// A guest's DialogueResult action carries the speaker and the phase in its cause; the host sees them inside onAction, and only there.
TEST(rejoin, action_cause_reaches_the_host_inside_on_action) {
    Rig r(7);
    r.addClient(1);
    r.joinOnly(0); r.run(1200);
    CHECK(r.cl[0]->cs->joined());
    std::vector<std::pair<IntentKind, Cause>> seen;
    r.host.onAction = [&](PlayerId, IntentKind k, const FormRef&, uint32_t, uint32_t) { seen.push_back({k, r.host.actionCause()}); };
    Cause c;
    c.kind = CauseKind::DialogueResult;
    c.source = FormRef("fallout3.esm", 0x14E4C);   // the speaker
    c.aux = 1;                                       // end phase
    CHECK(r.cl[0]->cs->submitAction(IntentKind::DialogueResult, FormRef("fallout3.esm", 0x2B1F3), c) != 0);
    r.run(300);
    CHECK(r.cl[0]->cs->submitAction(IntentKind::ActivateRef, FormRef("fallout3.esm", 0x1234)) != 0);
    r.run(300);
    CHECK(seen.size() == 2);
    if (seen.size() == 2) {
        CHECK(seen[0].first == IntentKind::DialogueResult && seen[0].second == c);
        CHECK(seen[1].first == IntentKind::ActivateRef && seen[1].second == Cause());
    }
    CHECK(r.host.actionCause() == Cause());   // cleared after the call
}

TEST(twin, rejoin_no_epoch_bump) {
    MutationScope m([] { Mutations x; x.noEpochBump = true; return x; }());
    std::string why;
    bool ok = true;
    for (uint64_t seed = 1; seed <= 6 && ok; ++seed) ok = dropThenReloadScenario(seed, &why);
    CHECK(!ok);   // the twin must be caught
}
TEST(twin, rejoin_no_idempotency) {
    MutationScope m([] { Mutations x; x.noIdempotency = true; return x; }());
    bool ok = true;
    std::string why;
    for (uint64_t seed = 1; seed <= 6 && ok; ++seed) ok = dropMidBatchScenario(seed, &why) && dropThenReloadScenario(seed, &why);
    CHECK(!ok);
}
TEST(twin, rejoin_departed_hold_barrier) {
    MutationScope m([] { Mutations x; x.departedHoldBarrier = true; return x; }());
    auto b = lateJoinBarrier(1, 1);
    CHECK(!b.ok);   // a leaver holds the barrier hostage
}
TEST(twin, rejoin_barrier_release_early) {
    MutationScope m([] { Mutations x; x.barrierReleaseEarly = true; return x; }());
    auto b = lateJoinBarrier(1, 0);
    CHECK(!b.ok);
}

// =====================================================================================================================
// Process restart: the player quits, loads their own save (with the client sidecar stored next to it) and rejoins.
// Rewards already delivered before that save must not be delivered again; rewards after it arrive exactly once.
// =====================================================================================================================
namespace {
bool restartWithSaveNoDuplicates(uint64_t seed, bool hostReloadsMeanwhile, std::string* why) {
    auto fail = [&](const char* w) { if (why) *why = w; return false; };
    Rig r(seed);
    r.addClient(1); r.addClient(2);
    r.joinAll(); r.run(1500);
    if (!r.allLive()) return fail("not live");
    for (uint32_t i = 0; i < 3; ++i) r.reward(0x300 + i);
    r.run(500);
    Bytes hostSave = r.host.saveSidecar();
    Bytes mySave = r.cl[1]->cs->saveState();                        // written next to the player's own save game
    if (mySave.empty()) return fail("no client sidecar");
    r.reward(0x303); r.reward(0x304);                               // after the save: these are NOT in the player's save
    r.run(500);
    r.drop(1);                                                      // quit
    if (hostReloadsMeanwhile) { r.host.loadSidecar(hostSave); r.run(800); r.reward(0x305); r.run(400); }
    r.cl[1]->sink.items.clear();                                    // fresh process: its engine holds exactly what its save held
    r.cl[1]->sink.trail.clear();
    r.cl[1]->dump = world(20, 9);
    r.story.setConnected(r.cl[1]->peer, true);
    if (!r.cl[1]->cs->onLocalSaveLoaded(r.now, &mySave)) return fail("load refused");
    r.run(4000);
    if (!r.allLive()) return fail("not live after restart");
    for (auto& c : r.cl) if (c->cs->state() != r.host.state()) return fail("diverged");
    auto& it = r.cl[1]->sink.items;
    for (uint32_t i = 0; i < 3; ++i) if (it[0x300 + i] != 0) return fail("a reward from before the save was delivered again");
    for (auto& kv : it) if (kv.second > 1) return fail("duplicate reward after restart");
    if (!hostReloadsMeanwhile && (it[0x303] != 1 || it[0x304] != 1)) return fail("a reward after the save was not delivered");
    r.reward(0x306); r.run(600);
    if (it[0x306] != 1) return fail("new reward not delivered exactly once");
    return true;
}
}  // namespace

TEST(rejoin, restart_with_own_save_gets_only_the_rewards_it_is_missing) {
    for (uint64_t seed = 1; seed <= 5; ++seed) {
        std::string why;
        CHECK_MSG(restartWithSaveNoDuplicates(seed, false, &why), "seed " + std::to_string(seed) + ": " + why);
        why.clear();
        CHECK_MSG(restartWithSaveNoDuplicates(seed, true, &why), "reload seed " + std::to_string(seed) + ": " + why);
    }
}
TEST(twin, rejoin_sidecar_loses_ledger) {
    MutationScope m([] { Mutations x; x.sidecarLosesLedger = true; return x; }());
    bool ok = true;
    std::string why;
    for (uint64_t seed = 1; seed <= 3 && ok; ++seed) ok = restartWithSaveNoDuplicates(seed, false, &why);
    CHECK(!ok);
}

// =====================================================================================================================
// Restart whose sidecar cannot be resumed (saved with an intent in flight): the shadow is discarded and the client rejoins with a baseline
// against a host whose ledger is intact. The reward ids stored in the sidecar keep the rewards its save already holds from coming again.
// =====================================================================================================================
namespace {
bool restartBaselineRejoinNoDuplicates(uint64_t seed, std::string* why) {
    auto fail = [&](const char* w) { if (why) *why = w; return false; };
    Rig r(seed);
    r.addClient(1); r.addClient(2);
    r.joinAll(); r.run(1500);
    if (!r.allLive()) return fail("not live");
    for (uint32_t i = 0; i < 3; ++i) r.reward(0x400 + i);
    r.run(500);
    Cause c; c.kind = CauseKind::Intent;
    r.cl[1]->cs->submitIntent(c, {SetGlobal{Q(0x3001), 7.f}});     // in flight at save time: the sidecar is not resumable
    Bytes mySave = r.cl[1]->cs->saveState();
    if (r.cl[1]->cs->canSaveState()) return fail("test setup: the sidecar should be unclean");
    r.reward(0x403); r.reward(0x404);                               // after the save: not in the player's save game
    r.run(500);
    r.drop(1);
    r.cl[1]->sink.items.clear();
    r.cl[1]->sink.trail.clear();
    r.cl[1]->dump = world(20, 9);
    r.story.setConnected(r.cl[1]->peer, true);
    if (!r.cl[1]->cs->onLocalSaveLoaded(r.now, &mySave)) return fail("load refused");
    r.run(4000);
    if (!r.allLive()) return fail("not live after restart");
    auto& it = r.cl[1]->sink.items;
    for (uint32_t i = 0; i < 3; ++i) if (it[0x400 + i] != 0) return fail("a reward the save already holds was delivered again");
    if (it[0x403] != 1 || it[0x404] != 1) return fail("a reward after the save was not delivered exactly once");
    r.reward(0x405); r.run(600);
    if (it[0x405] != 1) return fail("new reward not delivered exactly once");
    // a second restart from the same sidecar after those: only what is newer than the save comes again, still once
    r.drop(1);
    r.cl[1]->sink.items.clear();
    r.story.setConnected(r.cl[1]->peer, true);
    if (!r.cl[1]->cs->onLocalSaveLoaded(r.now, &mySave)) return fail("second load refused");
    r.run(4000);
    for (uint32_t i = 0; i < 3; ++i) if (r.cl[1]->sink.items[0x400 + i] != 0) return fail("second restart: old reward again");
    for (auto& kv : r.cl[1]->sink.items) if (kv.second > 1) return fail("second restart: duplicate");
    return true;
}
}  // namespace

TEST(rejoin, baseline_rejoin_after_an_unresumable_sidecar_delivers_only_missing_rewards) {
    for (uint64_t seed = 1; seed <= 5; ++seed) {
        std::string why;
        CHECK_MSG(restartBaselineRejoinNoDuplicates(seed, &why), "seed " + std::to_string(seed) + ": " + why);
    }
}
TEST(twin, rejoin_no_reward_ids) {
    MutationScope m([] { Mutations x; x.noRewardIds = true; return x; }());
    std::string why;
    CHECK(!restartBaselineRejoinNoDuplicates(1, &why));
}

// =====================================================================================================================
// Same restart against a host running without a world baseline (plugin baseline=0), on a client without a world dumper: the
// session asks for a rebase and the app rejoins through the plain snapshot path with an empty shadow. The snapshot's ledger must
// not hand over the rewards the save already holds (was 207 vs 107 caps).
// =====================================================================================================================
namespace {
bool restartPlainRejoinNoDuplicates(uint64_t seed, std::string* why) {
    auto fail = [&](const char* w) { if (why) *why = w; return false; };
    Rig r(seed, false, 40, HostSession::Options(), /*withHostBaseline=*/false);
    r.addClient(1); r.addClient(2);
    r.joinAll(); r.run(1500);
    if (!r.allLive()) return fail("not live");
    for (uint32_t i = 0; i < 3; ++i) r.reward(0x500 + i);
    r.run(500);
    Cause c; c.kind = CauseKind::Intent;
    r.cl[1]->cs->submitIntent(c, {SetGlobal{Q(0x3002), 7.f}});     // in flight at save time: the sidecar is not resumable
    Bytes mySave = r.cl[1]->cs->saveState();
    if (r.cl[1]->cs->canSaveState()) return fail("test setup: the sidecar should be unclean");
    r.reward(0x503); r.reward(0x504);                               // after the save: not in the player's save game
    r.run(500);
    r.drop(1);
    r.cl[1]->sink.items.clear();
    r.cl[1]->sink.trail.clear();
    r.cl[1]->cs->setBaselineProvider(nullptr);                      // this app cannot dump its world
    r.story.setConnected(r.cl[1]->peer, true);
    if (!r.cl[1]->cs->onLocalSaveLoaded(r.now, &mySave)) return fail("load refused");
    if (!r.cl[1]->cs->rebaseRequired()) return fail("test setup: expected a rebase request without a provider");
    r.cl[1]->cs->connect(r.now, r.cl[1]->cs->lastSession() + 1);   // the app restarts the join itself
    r.run(4000);
    if (!r.allLive()) {
        auto& cs = *r.cl[1]->cs;
        if (why) *why = "not live after restart: joined=" + std::to_string(cs.joined()) + " rejected=" + std::to_string(cs.rejected()) +
                        " reason=" + std::to_string(cs.rejectReason()) + " baselineActive=" + std::to_string(cs.baselineActive()) +
                        " applied=" + std::to_string(cs.applied()) + " head=" + std::to_string(r.host.log().head());
        return false;
    }
    if (r.cl[1]->cs->state() != r.host.state()) return fail("diverged");
    auto& it = r.cl[1]->sink.items;
    for (uint32_t i = 0; i < 3; ++i) if (it[0x500 + i] != 0) return fail("a reward the save already holds was delivered again");
    if (it[0x503] != 1 || it[0x504] != 1) return fail("a reward after the save was not delivered exactly once");
    for (auto& kv : it) if (kv.second > 1) return fail("duplicate reward after plain rejoin");
    r.reward(0x505); r.run(600);
    if (it[0x505] != 1) return fail("new reward not delivered exactly once");
    return true;
}
}  // namespace

TEST(rejoin, plain_rejoin_after_an_unresumable_sidecar_delivers_only_missing_rewards) {
    for (uint64_t seed = 1; seed <= 5; ++seed) {
        std::string why;
        CHECK_MSG(restartPlainRejoinNoDuplicates(seed, &why), "seed " + std::to_string(seed) + ": " + why);
    }
}
TEST(twin, plain_rejoin_no_reward_ids) {
    MutationScope m([] { Mutations x; x.noRewardIds = true; return x; }());
    std::string why;
    CHECK(!restartPlainRejoinNoDuplicates(1, &why));
    CHECK_MSG(why.find("reward") != std::string::npos, "the twin must be caught by the reward check, not by the test setup: " + why);
}
