// Actor ownership service and view: unit tests. Scenario functions are shared with the mutation twins (test_twins.cpp).
#include "ownership_scenarios.h"
#include "fnvcoop/wire.h"
#include "minitest.h"

using namespace fnvcoop;
using namespace fnvcoop::testing;

TEST(own, a_pinned_player_entity_never_moves_and_a_rejoin_starts_above_the_old_session) {
    std::string why;
    CHECK_MSG(pinnedPlayersHold(&why), why);
}

namespace {
Presence P(PlayerId p, bool inCell, bool adj, float d, bool connected = true) {
    Presence x; x.player = p; x.inCell = inCell; x.adjacent = adj; x.distance = d; x.connected = connected; return x;
}
}  // namespace

TEST(own, pick_prefers_host_when_eligible) {
    OwnershipConfig c;
    CHECK(pickOwner({P(0, false, true, 5000), P(1, true, true, 10)}, kNoPlayer, false, c) == 0);
}
TEST(own, pick_nearest_when_host_far) {
    OwnershipConfig c;
    CHECK(pickOwner({P(0, false, false, 9000), P(2, true, true, 400), P(1, true, true, 100)}, kNoPlayer, false, c) == 1);
}
TEST(own, pick_nobody_eligible_keeps_dormant_owner_or_none) {
    OwnershipConfig c;
    CHECK(pickOwner({P(0, false, false, 9000), P(1, false, false, 9000)}, 1, false, c) == 1);
    CHECK(pickOwner({P(0, false, false, 9000), P(1, false, false, 9000, false)}, 1, false, c) == kNoPlayer);
    CHECK(pickOwner({P(0, false, false, 9000)}, kNoPlayer, false, c) == kNoPlayer);
}
TEST(own, pick_ties_go_to_lowest_slot) {
    OwnershipConfig c;
    CHECK(pickOwner({P(3, true, true, 100), P(2, true, true, 100)}, kNoPlayer, false, c) == 2);
}
TEST(own, pick_hysteresis_hold_and_margin) {
    OwnershipConfig c;   // margin 25%
    // challenger only slightly closer: keep
    CHECK(pickOwner({P(1, true, true, 1000), P(2, true, true, 900)}, 1, false, c) == 1);
    // clearly closer but still within the hold window: keep
    CHECK(pickOwner({P(1, true, true, 1000), P(2, true, true, 100)}, 1, true, c) == 1);
    // clearly closer and hold elapsed: switch
    CHECK(pickOwner({P(1, true, true, 1000), P(2, true, true, 100)}, 1, false, c) == 2);
    // the current owner left the area: switch at once, hold or not
    CHECK(pickOwner({P(1, false, false, 9000), P(2, true, true, 100)}, 1, true, c) == 2);
    // the current owner disconnected
    CHECK(pickOwner({P(1, true, true, 10, false), P(2, true, true, 100)}, 1, true, c) == 2);
}

TEST(own, track_grants_first_owner_with_epoch_1) {
    OwnershipService s; s.playerJoined(0); s.playerJoined(1);
    OwnershipService::Out out;
    s.track(7, {P(0, false, false, 9000), P(1, true, true, 50)}, 0, out);
    REQUIRE(out.size() == 1);
    CHECK(out[0].kind == OwnMsg::Grant);
    CHECK(out[0].owner == 1);
    CHECK(out[0].epoch == 1);
    CHECK(s.ownerOf(7) == 1);
    out.clear();
    s.evaluate(7, {P(0, false, false, 9000), P(1, true, true, 50)}, 100, out);
    CHECK(out.empty());   // idempotent
}

TEST(own, handoff_handshake_epochs_and_commit) {
    OwnershipService s; s.playerJoined(0); s.playerJoined(1);
    OwnershipService::Out out;
    s.track(7, {P(0, false, false, 9000), P(1, true, true, 50)}, 0, out);
    out.clear();
    // host walks next to the actor: host preferred once hold elapsed
    s.evaluate(7, {P(0, true, true, 10), P(1, true, true, 50)}, 2000, out);
    REQUIRE(out.size() == 1);
    CHECK(out[0].kind == OwnMsg::Transfer);
    CHECK(out[0].epoch == 2);
    CHECK(out[0].owner == 0);
    CHECK(out[0].prevOwner == 1);
    CHECK(s.ownerOf(7) == 1);            // not yet
    CHECK(s.handoffPending(7));
    OwnMsg ack; ack.kind = OwnMsg::Ack; ack.netId = 7; ack.epoch = 2; ack.owner = 0;
    out.clear();
    CHECK(!s.onAck(ack, 1, 2100, out));  // wrong sender
    CHECK(s.onAck(ack, 0, 2100, out));
    REQUIRE(out.size() == 1);
    CHECK(out[0].kind == OwnMsg::Commit);
    CHECK(out[0].epoch == 2);
    CHECK(s.ownerOf(7) == 0);
    CHECK(s.epochOf(7) == 2);
    out.clear();
    CHECK(!s.onAck(ack, 0, 2200, out));  // duplicate ack is ignored
    CHECK(out.empty());
}

TEST(own, handoff_timeout_forces_commit_or_aborts) {
    OwnershipService s; s.playerJoined(0); s.playerJoined(1);
    OwnershipService::Out out;
    s.track(7, {P(1, true, true, 50)}, 0, out);
    out.clear();
    s.evaluate(7, {P(0, true, true, 10), P(1, true, true, 50)}, 2000, out);
    out.clear();
    s.tick(2500, out);
    CHECK(out.empty());
    s.tick(3100, out);
    REQUIRE(out.size() == 1);
    CHECK(out[0].kind == OwnMsg::Commit);
    CHECK(s.stats().forcedCommits == 1);
}

TEST(own, new_owner_leaving_mid_handoff_cancels_it) {
    OwnershipService s; s.playerJoined(0); s.playerJoined(1);
    OwnershipService::Out out;
    s.track(7, {P(1, true, true, 50)}, 0, out);
    out.clear();
    s.evaluate(7, {P(0, true, true, 10), P(1, true, true, 50)}, 2000, out);
    out.clear();
    s.playerLeft(0, 2100, out);
    REQUIRE(out.size() == 1);
    CHECK(out[0].kind == OwnMsg::Grant);   // old owner keeps the actor, at a higher epoch than the cancelled transfer
    CHECK(out[0].owner == 1);
    CHECK(out[0].epoch == 3);
    CHECK(!s.handoffPending(7));
}

TEST(own, old_owner_leaving_mid_handoff_commits_to_new_owner) {
    OwnershipService s; s.playerJoined(0); s.playerJoined(1);
    OwnershipService::Out out;
    s.track(7, {P(1, true, true, 50)}, 0, out);
    out.clear();
    s.evaluate(7, {P(0, true, true, 10), P(1, true, true, 50)}, 2000, out);
    out.clear();
    s.playerLeft(1, 2100, out);
    REQUIRE(out.size() == 1);
    CHECK(out[0].kind == OwnMsg::Commit);
    CHECK(s.ownerOf(7) == 0);
}

TEST(own, owner_leaving_releases_and_regrants) {
    OwnershipService s; s.playerJoined(0); s.playerJoined(1);
    OwnershipService::Out out;
    s.track(7, {P(1, true, true, 50)}, 0, out);
    out.clear();
    auto orphans = s.playerLeft(1, 100, out);
    REQUIRE(orphans.size() == 1);
    REQUIRE(out.size() == 1);
    CHECK(out[0].kind == OwnMsg::Release);
    CHECK(out[0].epoch == 2);
    out.clear();
    s.evaluate(7, {P(0, true, true, 10)}, 200, out);
    REQUIRE(out.size() == 1);
    CHECK(out[0].kind == OwnMsg::Grant);
    CHECK(out[0].owner == 0);
    CHECK(out[0].epoch == 3);
}

TEST(own, follower_exception_follows_recruiter_everywhere) {
    OwnershipService s; s.playerJoined(0); s.playerJoined(1);
    OwnershipService::Out out;
    s.track(7, {P(0, true, true, 10), P(1, false, false, 9000)}, 0, out);
    CHECK(s.ownerOf(7) == 0);
    out.clear();
    // recruited by player 1, who is nowhere near: ownership goes to the recruiter (no hold, no eligibility needed)
    s.setRecruiter(7, 1, {P(0, true, true, 10), P(1, false, false, 9000)}, 5, out);
    REQUIRE(out.size() == 1);
    CHECK(out[0].kind == OwnMsg::Transfer);
    CHECK(out[0].owner == 1);
    // ack, then the actor stays with the recruiter though the host is closer
    OwnMsg ack; ack.kind = OwnMsg::Ack; ack.netId = 7; ack.epoch = out[0].epoch; ack.owner = 1;
    out.clear();
    CHECK(s.onAck(ack, 1, 10, out));
    out.clear();
    s.evaluate(7, {P(0, true, true, 10), P(1, false, false, 9000)}, 99999, out);
    CHECK(out.empty());
    CHECK(s.ownerOf(7) == 1);
    // recruiter disconnects: companion becomes a normal actor again
    s.playerLeft(1, 100000, out);
    out.clear();
    s.evaluate(7, {P(0, true, true, 10)}, 100001, out);
    CHECK(s.ownerOf(7) == 0);
}

TEST(own, snapshot_reports_epochs_for_joiners) {
    OwnershipService s; s.playerJoined(0); s.playerJoined(1);
    OwnershipService::Out out;
    s.track(7, {P(1, true, true, 50)}, 0, out);
    s.track(8, {P(0, true, true, 50)}, 0, out);
    s.playerLeft(1, 10, out);
    OwnershipService::Out snap;
    s.snapshot(snap);
    REQUIRE(snap.size() == 2);
    OwnershipView v(2);
    for (auto& m : snap) v.apply(m);
    CHECK(v.ownerOf(8) == 0);
    CHECK(v.ownerOf(7) == kNoPlayer);
    CHECK(v.epochOf(7) == 2);   // unowned but the epoch is remembered: old owner's late updates stay stale
}

TEST(own, view_apply_is_idempotent_and_order_independent) {
    OwnMsg g1; g1.kind = OwnMsg::Grant; g1.netId = 5; g1.epoch = 1; g1.owner = 1;
    OwnMsg t2; t2.kind = OwnMsg::Transfer; t2.netId = 5; t2.epoch = 2; t2.owner = 2; t2.prevOwner = 1;
    OwnMsg c2; c2.kind = OwnMsg::Commit; c2.netId = 5; c2.epoch = 2; c2.owner = 2; c2.prevOwner = 1;
    OwnershipView v(0);
    CHECK(v.apply(g1) == ApplyResult::Applied);
    CHECK(v.apply(g1) == ApplyResult::Duplicate);
    CHECK(v.apply(t2) == ApplyResult::Applied);
    CHECK(v.apply(t2) == ApplyResult::Duplicate);
    OwnerEpoch pe = 0;
    CHECK(!v.handoffIncoming(5, pe));
    CHECK(v.ownerOf(5) == 1);
    CHECK(v.apply(c2) == ApplyResult::Applied);
    CHECK(v.apply(c2) == ApplyResult::Duplicate);
    CHECK(v.apply(g1) == ApplyResult::Stale);   // replayed old grant (the echo of STR #906) cannot win
    CHECK(v.apply(t2) != ApplyResult::Applied);   // a replayed Transfer after its Commit is a harmless duplicate
    CHECK(v.ownerOf(5) == 2);
    CHECK(v.epochOf(5) == 2);
    // a Commit that arrives without its Transfer (lost) still applies
    OwnershipView w(0);
    w.apply(g1);
    CHECK(w.apply(c2) == ApplyResult::Applied);
    CHECK(w.ownerOf(5) == 2);
}

TEST(own, view_check_verdicts) {
    OwnershipView v(3);
    OwnMsg g; g.kind = OwnMsg::Grant; g.netId = 5; g.epoch = 4; g.owner = 1;
    v.apply(g);
    CHECK(v.check(5, 1, 4) == UpdateVerdict::Accept);
    CHECK(v.check(5, 2, 4) == UpdateVerdict::NotOwner);
    CHECK(v.check(5, 1, 3) == UpdateVerdict::StaleEpoch);
    CHECK(v.check(5, 1, 5) == UpdateVerdict::FutureEpoch);
    CHECK(v.check(5, 3, 4) == UpdateVerdict::EchoOfSelf);     // our own publish reflected back
    CHECK(v.check(99, 1, 1) == UpdateVerdict::UnknownActor);
    OwnMsg t; t.kind = OwnMsg::Transfer; t.netId = 5; t.epoch = 5; t.owner = 2; t.prevOwner = 1;
    v.apply(t);
    CHECK(v.check(5, 1, 4) == UpdateVerdict::Accept);          // old owner still authoritative during the handoff
    CHECK(v.check(5, 2, 5) == UpdateVerdict::NotYetActive);    // new owner must wait for the commit
    CHECK(v.stats().echoes == 1);
    OwnerEpoch e = 0;
    CHECK(!v.mayPublish(5, e));
    OwnershipView me(1);
    me.apply(g);
    CHECK(me.mayPublish(5, e));
    CHECK(e == 4);
}

TEST(own, view_handoff_incoming_for_new_owner) {
    OwnershipView v(2);
    OwnMsg g; g.kind = OwnMsg::Grant; g.netId = 5; g.epoch = 1; g.owner = 1;
    OwnMsg t; t.kind = OwnMsg::Transfer; t.netId = 5; t.epoch = 2; t.owner = 2; t.prevOwner = 1;
    v.apply(g); v.apply(t);
    OwnerEpoch pe = 0;
    CHECK(v.handoffIncoming(5, pe));
    CHECK(pe == 2);
}

TEST(own, wire_roundtrip_and_fuzz_safe) {
    OwnMsg m; m.kind = OwnMsg::Commit; m.netId = 123456; m.epoch = 77; m.owner = 3; m.prevOwner = 1;
    Archive w = Archive::writer(); io(w, m);
    Bytes b = w.finish();
    Archive r = Archive::reader(b.data(), b.size());
    OwnMsg back; io(r, back);
    CHECK(r.ok() && back == m);
    for (size_t i = 0; i < b.size(); ++i) {   // truncations never crash
        Archive t = Archive::reader(b.data(), i);
        OwnMsg x; io(t, x);
    }
    Bytes bad = b; bad[bad.size() - 5] = 0xFF;
    Archive q = Archive::reader(bad.data(), bad.size());
    OwnMsg y; io(q, y);
    CHECK(true);   // reaching here means no truncated or corrupt input crashed the reader
}

TEST(own, random_churn_keeps_one_owner_per_epoch_and_monotonic_epochs) { CHECK(churnHolds(1, 400)); CHECK(churnHolds(2, 400)); CHECK(churnHolds(3, 400)); }
TEST(own, stale_echo_injection_is_rejected) { CHECK(staleEchoRejected()); }
TEST(own, player_leave_and_rejoin_keeps_epochs_monotonic) { CHECK(rejoinEpochsMonotonic()); }

TEST(own, nothing_to_hand_over_commits_at_once_to_a_connected_new_owner) {
    std::string why;
    CHECK_MSG(commitNothingToHandOverHolds(&why), why);
}
