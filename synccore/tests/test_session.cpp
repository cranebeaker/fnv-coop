// Session-level behaviour over the in-memory network (perfect link unless a test degrades it).
#include <memory>
#include <tuple>
#include "fnvcoop/authority.h"
#include "fnvcoop/replicator.h"
#include "fnvcoop/session.h"
#include "minitest.h"

using namespace fnvcoop;

namespace {
FormRef Q(uint32_t i) { return FormRef("falloutnv.esm", i); }

struct RecSink : IEffectSink {
    std::vector<Effect> got;
    std::vector<uint64_t> seqs;
    EffectCapture* cap = nullptr;
    size_t reemitted = 0;
    PlayerId joinedSlot = kNoPlayer;
    void apply(const Effect& e, const Context& c) override {
        got.push_back(e);
        seqs.push_back(c.seq);
        if (cap) cap->record(e);  // the engine hook firing on our own apply
    }
    void onJoined(PlayerId s, const SharePolicy&) override { joinedSlot = s; }
    size_t count(size_t index) const { size_t n = 0; for (auto& e : got) if (e.index() == index) ++n; return n; }
};

struct Rig {
    InMemoryNetwork net{1};
    HostSession host;
    struct C { RecSink sink; EffectCapture cap; std::unique_ptr<ClientSession> s; uint32_t sess = 0; };
    std::vector<std::unique_ptr<C>> clients;
    uint64_t now = 0;

    explicit Rig(SharePolicy p = {}) : host(*net.endpoint(0), p) {}
    C& add(uint64_t token) {
        auto c = std::make_unique<C>();
        c->sink.cap = &c->cap;
        c->s = std::make_unique<ClientSession>(*net.endpoint(PeerId(clients.size() + 1)), 0, token, c->sink, &c->cap);
        clients.push_back(std::move(c));
        return *clients.back();
    }
    void connect(C& c) {
        c.s->connect(now, ++c.sess);
    }
    void run(uint32_t ms) {
        for (uint32_t t = 0; t < ms; t += 5) {
            now += 5;
            net.advance(now);
            host.tick(now);
            for (auto& c : clients) c->s->tick(now);
            net.advance(now);
        }
    }
};
Cause quest(uint32_t q = 1) { Cause c; c.kind = CauseKind::QuestResult; c.source = Q(q); return c; }
}  // namespace

TEST(session, join_replays_history_and_stays_in_sync) {
    Rig r;
    r.host.commit(0, quest(), {SetStage{Q(1), 10}, AddItem{Q(5), 2, Recipient::dflt()}});
    r.host.commit(0, quest(), {SetStage{Q(1), 20}});
    auto& a = r.add(111);
    r.connect(a);
    r.run(200);
    CHECK(a.s->joined());
    CHECK(a.sink.joinedSlot == 1);
    CHECK(a.s->applied() == 2);
    CHECK(a.s->state() == r.host.state());
    CHECK(a.sink.count(5) == 1);  // quest reward item delivered to the joiner by replay
    r.host.commit(0, quest(), {SetStage{Q(1), 30}});
    r.run(100);
    CHECK(a.s->applied() == 3);
    CHECK(r.host.allSynced());
}

TEST(session, quest_reward_reaches_every_player_exactly_once) {
    Rig r;
    auto& a = r.add(1); auto& b = r.add(2); auto& c = r.add(3);
    for (auto* x : {&a, &b, &c}) r.connect(*x);
    r.run(100);
    r.host.commit(0, quest(), {AddItem{Q(7), 1, Recipient::dflt()}, ModActorValue{2, 500, Recipient::dflt()}, ModKarma{-20, Recipient::dflt()}});
    r.run(200);
    for (auto* x : {&a, &b, &c}) {
        CHECK(x->sink.count(5) == 1);   // AddItem
        CHECK(x->sink.count(7) == 1);   // ModActorValue (caps/XP)
        CHECK(x->sink.count(9) == 1);   // ModKarma shared
    }
}

TEST(session, loot_goes_only_to_the_looter) {
    Rig r;
    auto& a = r.add(1); auto& b = r.add(2);
    r.connect(a); r.connect(b);
    r.run(100);
    Cause loot; loot.kind = CauseKind::Loot;
    r.host.commit(2, loot, {AddItem{Q(7), 1, Recipient::dflt()}});   // slot 2 == client b
    r.run(100);
    CHECK(a.sink.count(5) == 0);
    CHECK(b.sink.count(5) == 1);
}

TEST(session, duplicate_and_reordered_delivery_applies_once_in_order) {
    Rig r;
    auto& a = r.add(1);
    r.connect(a);
    r.run(50);
    // Hand-deliver txns out of order and duplicated, straight to the client's transport.
    for (int i = 1; i <= 5; ++i) r.host.commit(0, quest(), {AddItem{Q(5), 1, Recipient::all()}, SetGlobal{Q(2), float(i)}});
    r.run(100);
    CHECK(a.s->applied() == 5);
    size_t items = a.sink.count(5);
    CHECK(items == 5);
    // Replay the same batches again, reversed and doubled.
    TxnBatch b; b.session = 1;
    for (uint64_t s = 5; s >= 1; --s) { b.txns.push_back(*r.host.log().get(s)); b.txns.push_back(*r.host.log().get(s)); }
    r.net.endpoint(0)->send(1, encodeMessage(b));
    r.run(50);
    CHECK(a.sink.count(5) == items);
    CHECK(a.s->applied() == 5);
    CHECK(a.s->stats().duplicatesDropped >= 10);
    CHECK(a.s->state() == r.host.state());
}

TEST(session, out_of_order_arrival_is_buffered_until_gap_fills) {
    Rig r;
    auto& a = r.add(1);
    r.connect(a);
    r.run(50);
    // Block host->client traffic by taking the client down while committing, then deliver 3 and 2 before 1.
    for (int i = 1; i <= 3; ++i) r.host.commit(0, quest(), {SetGlobal{Q(2), float(i)}});
    r.run(10);
    CHECK(a.s->applied() == 3);  // normal path sanity
    // New session txns 4..6 delivered 6,5,4 manually
    r.net.setConnected(1, false);
    for (int i = 4; i <= 6; ++i) r.host.commit(0, quest(), {SetStage{Q(1), uint32_t(i)}});
    r.net.setConnected(1, true);
    TxnBatch b; b.session = 1;
    b.txns.push_back(*r.host.log().get(6));
    r.net.endpoint(0)->send(1, encodeMessage(b));
    b.txns[0] = *r.host.log().get(5);
    r.net.endpoint(0)->send(1, encodeMessage(b));
    r.net.advance(r.now);
    a.s->tick(r.now);
    CHECK(a.s->applied() == 3);  // 5 and 6 buffered, 4 missing
    b.txns[0] = *r.host.log().get(4);
    r.net.endpoint(0)->send(1, encodeMessage(b));
    r.net.advance(r.now);
    a.s->tick(r.now);
    CHECK(a.s->applied() == 6);
    CHECK(a.s->state() == r.host.state());
}

TEST(session, rejoin_resumes_by_replay_and_after_compaction_by_snapshot) {
    Rig r;
    auto& a = r.add(1); auto& b = r.add(2);
    r.connect(a); r.connect(b);
    r.run(100);
    r.host.commit(0, quest(), {AddItem{Q(5), 1, Recipient::dflt()}, SetStage{Q(1), 10}});
    r.run(100);
    CHECK(a.s->applied() == 1);

    // a drops; the story moves on and the host compacts its log.
    a.s->disconnect(); r.net.setConnected(1, false);
    for (int i = 2; i <= 6; ++i) r.host.commit(0, quest(), {AddItem{Q(5), 1, Recipient::dflt()}, SetStage{Q(1), uint32_t(i * 10)}});
    r.host.mutableLog().compact(0);
    r.run(50);

    // b meanwhile never dropped.
    CHECK(b.s->applied() == 6);

    // a rejoins with the same token: its lastSeq (1) is below the log base -> snapshot, applied as a diff.
    size_t itemsBefore = a.sink.count(5);
    r.net.setConnected(1, true); r.connect(a);
    r.run(300);
    CHECK(a.s->joined());
    CHECK(a.s->applied() == 6);
    CHECK(a.s->stats().snapshots == 1);
    CHECK(a.sink.count(5) - itemsBefore == 1);  // 5 missed rewards arrive as ONE net delta, not duplicated
    CHECK(a.s->state() == r.host.state());
    CHECK(a.sink.joinedSlot == 1);
    CHECK(r.host.stats().snapshots >= 1);

    // And a plain replay rejoin (log intact) resumes without a snapshot.
    a.s->disconnect(); r.net.setConnected(1, false);
    r.host.commit(0, quest(), {SetStage{Q(1), 99}});
    r.net.setConnected(1, true); r.connect(a);
    r.run(300);
    CHECK(a.s->applied() == 7);
    CHECK(a.s->stats().snapshots == 1);
    CHECK(a.s->stats().replays >= 1);
}

TEST(session, snapshot_for_a_lagging_client_is_paced_not_sent_every_tick) {
    // Real host 2026-11-17: a joined guest's Acks stopped while the log compacted past its cursor; the host then sent a whole-snapshot
    // Welcome on every tick (126 KB per frame), ENet queued them all and the game ran out of memory.
    Rig r;
    auto& a = r.add(1);
    r.connect(a);
    r.run(100);
    r.host.commit(0, quest(), {SetStage{Q(1), 10}});
    r.run(100);
    CHECK(a.s->applied() == 1);

    r.net.setConnected(1, false);   // the link stalls; the host still counts the guest as joined
    for (int i = 2; i <= 6; ++i) r.host.commit(0, quest(), {SetStage{Q(1), uint32_t(i * 10)}});
    r.host.mutableLog().compact(0);
    const uint64_t before = r.host.stats().snapshots;
    r.run(3000);   // 600 ticks
    const uint64_t sent = r.host.stats().snapshots - before;
    CHECK_MSG(sent >= 1 && sent <= 6, "snapshots in 3 s of stall: " + std::to_string(sent));

    r.net.setConnected(1, true);    // the link comes back: the next paced snapshot brings the guest up to date
    r.run(3000);
    CHECK(a.s->applied() == 6);
    CHECK(a.s->state() == r.host.state());
}

TEST(session, same_player_gets_same_slot_and_server_full_is_rejected) {
    SharePolicy p; p.maxPlayers = 2;  // host + 1
    Rig r(p);
    auto& a = r.add(1); auto& b = r.add(2);
    r.connect(a); r.connect(b);
    r.run(200);
    CHECK(a.s->joined()); CHECK(a.s->slot() == 1);
    CHECK(!b.s->joined()); CHECK(b.s->rejected());
    a.s->disconnect(); a.s->connect(r.now, 99);
    r.run(200);
    CHECK(a.s->slot() == 1);
}

// Shared by the real test and its twin. Returns true iff nothing was re-emitted.
static bool echoGuardHolds() {
    Rig r;
    auto& a = r.add(1);
    r.connect(a);
    r.run(100);
    r.host.commit(0, quest(), {SetStage{Q(1), 10}, AddItem{Q(5), 1, Recipient::dflt()}});
    r.run(100);
    return a.cap.empty() && a.sink.got.size() == 2 && !a.s->intentsPending();
}

TEST(session, echo_guard_prevents_reemit_of_remote_applies) {
    CHECK(echoGuardHolds());
    // Capture records only OUTSIDE the guard:
    EffectCapture cap;
    cap.record(SetStage{Q(1), 1});
    { EchoGuard g(&cap); cap.record(SetStage{Q(1), 2}); { EchoGuard g2(&cap); cap.record(SetStage{Q(1), 3}); } cap.record(SetStage{Q(1), 4}); }
    cap.record(SetStage{Q(1), 5});
    CHECK(cap.take().size() == 2);
    CHECK(cap.suppressedCount() == 3);
}

TEST(session, intent_is_committed_once_and_origin_does_not_reapply) {
    Rig r;
    auto& a = r.add(1); auto& b = r.add(2);
    r.connect(a); r.connect(b);
    r.run(100);
    Cause loot; loot.kind = CauseKind::Loot;
    uint32_t id = a.s->submitIntent(loot, {AddItem{Q(5), 1, Recipient::dflt()}});
    CHECK(id == 1);
    r.run(400);
    CHECK(!a.s->intentsPending());
    CHECK(r.host.stats().intentsCommitted == 1);
    CHECK(r.host.log().head() == 1);
    CHECK(a.sink.count(5) == 0);   // origin already did it locally
    CHECK(b.sink.count(5) == 0);   // loot is looter-only
    // a client claiming a quest-result cause is downgraded: no free rewards for everyone
    Cause fake = quest();
    a.s->submitIntent(fake, {AddItem{Q(6), 9, Recipient::dflt()}});
    r.run(400);
    CHECK(b.sink.count(5) == 0);
    CHECK(r.host.state().ledger[2].items.empty());
    // retransmitted intent (lost ack) is not committed twice
    TxnBatch unused; (void)unused;
    r.net.endpoint(1)->send(0, encodeMessage(Intent{1, 1, 1, loot, {AddItem{Q(5), 1, Recipient::dflt()}}}));
    r.run(100);
    CHECK(r.host.log().head() == 2);
    CHECK(r.host.stats().intentsDuplicate >= 1);
}

TEST(session, app_channel_carries_opaque_payloads_both_ways) {
    Rig r;
    auto& a = r.add(1); auto& b = r.add(2);
    r.connect(a); r.connect(b);
    r.run(100);
    std::vector<std::pair<int, Bytes>> hostGot, aGot, bGot;
    r.host.onApp = [&](PlayerId slot, uint16_t ch, const Bytes& p) { hostGot.push_back({int(slot) * 1000 + ch, p}); r.host.relayApp(slot, ch, p); };
    a.s->onApp = [&](PlayerId from, uint16_t ch, const Bytes& p) { aGot.push_back({int(from) * 1000 + ch, p}); };
    b.s->onApp = [&](PlayerId from, uint16_t ch, const Bytes& p) { bGot.push_back({int(from) * 1000 + ch, p}); };
    a.s->sendApp(7, Bytes{1, 2, 3});
    r.run(100);
    REQUIRE(hostGot.size() == 1);
    CHECK(hostGot[0].first == 1007);
    CHECK(aGot.empty());
    REQUIRE(bGot.size() == 1);
    CHECK((bGot[0].second == Bytes{1, 2, 3} && bGot[0].first == 1007));
    r.host.sendApp(kNoPlayer, 9, Bytes{});
    r.run(100);
    CHECK(aGot.size() == 1); CHECK(bGot.size() == 2);
}

TEST(session, guest_action_requests_run_on_host_exactly_once) {
    Rig r;
    auto& a = r.add(1); auto& b = r.add(2);
    r.connect(a); r.connect(b);
    r.run(100);
    std::vector<std::tuple<int, int, uint32_t>> ran;
    r.host.onAction = [&](PlayerId slot, IntentKind k, const FormRef& ref, uint32_t, uint32_t) {
        ran.push_back({int(slot), int(k), ref.localId});
        Cause c; c.kind = CauseKind::Script; c.source = ref;
        r.host.commit(slot, c, {SetStage{Q(77), 10}});  // the real activation's captured effects
    };
    uint32_t id1 = a.s->submitAction(IntentKind::ActivateRef, Q(500));
    uint32_t id2 = a.s->submitAction(IntentKind::PickUp, Q(501));
    CHECK(id1 == 1); CHECK(id2 == 2);
    CHECK(a.s->submitAction(IntentKind::AppliedEffects, Q(1)) == 0);
    r.run(600);
    REQUIRE(ran.size() == 2);
    CHECK((std::get<0>(ran[0]) == 1 && std::get<1>(ran[0]) == int(IntentKind::ActivateRef) && std::get<2>(ran[0]) == 500u));
    CHECK(std::get<1>(ran[1]) == int(IntentKind::PickUp));
    CHECK(!a.s->intentsPending());
    CHECK(b.s->state() == r.host.state());                  // everyone sees the host's result, nobody applied anything locally
    CHECK(a.sink.count(0) == 1);                            // the requester gets the stage from the host txn like anyone else
    // a duplicated request datagram must not execute twice
    r.net.endpoint(1)->send(0, encodeMessage(Intent{1, 1, 1, Cause(), {}, IntentKind::ActivateRef, Q(500)}));
    r.run(100);
    CHECK(ran.size() == 2);
}

TEST(session, state_replicator_coalesces_skips_unchanged_and_obeys_budget) {
    Rig r;
    auto& a = r.add(1);
    r.connect(a);
    r.run(100);
    StateReplicator::Options o; o.maxEffectsPerFlush = 8; o.minIntervalMs = 10; o.maxEffectsPerSec = 100;
    StateReplicator rep(r.host, o);
    uint64_t t = 0;
    // 1000 writes to ONE var within a frame -> exactly one effect with the newest value
    for (int i = 0; i < 1000; ++i) rep.setScriptVar(Q(1), 0, double(i));
    rep.flush(t += 20);
    CHECK(r.host.state().scriptVars.at({Q(1), 0}) == 999.0);
    CHECK(rep.stats().effectsCommitted == 1);
    CHECK(rep.stats().coalesced == 999);
    // unchanged value is not re-sent
    rep.setScriptVar(Q(1), 0, 999.0);
    CHECK(rep.flush(t += 20) == 0);
    CHECK(rep.stats().skippedUnchanged == 1);
    // 200 distinct vars at 100 effects/s: bounded rate, nothing starves, newest value wins
    for (uint16_t i = 0; i < 200; ++i) rep.setScriptVar(Q(2), i, 1.0);
    uint64_t start = t;
    while (rep.pending() && t < start + 10000) {
        if (t < start + 300) for (uint16_t i = 0; i < 200; i += 50) rep.setScriptVar(Q(2), i, double(t));  // churn while waiting
        rep.flush(t += 10);
    }
    CHECK(rep.pending() == 0);
    uint64_t elapsed = t - start;
    CHECK(elapsed >= 1500);          // ~200 effects at 100/s cannot go out faster than the budget (minus the bucket)
    CHECK(elapsed < 5000);
    r.run(300);
    CHECK(a.s->state() == r.host.state());
    CHECK(r.host.state().scriptVars.size() == 201);
}

TEST(session, guest_never_runs_quest_scripts_but_host_does) {
    CHECK(questScriptsMayRun(Role::Host));
    CHECK(!questScriptsMayRun(Role::Guest));
    Mutations m; m.guestRunsQuestScripts = true;
    MutationScope s(m);
    CHECK(questScriptsMayRun(Role::Guest));
}

TEST(session, garbage_datagrams_are_ignored) {
    Rig r;
    auto& a = r.add(1);
    r.connect(a);
    Rng rng(3);
    for (int i = 0; i < 500; ++i) {
        Bytes g(rng.range(0, 64));
        for (auto& x : g) x = uint8_t(rng.u32());
        r.net.endpoint(1)->send(0, g);
        r.net.endpoint(0)->send(1, g);
    }
    r.run(500);
    CHECK(a.s->joined());
    CHECK(r.host.stats().badMessages > 0);
}

// Ghost-kill rule: on a guest, scripts must not author shared state; genuine local actions may.
static bool guestSuppressionHolds() {
    Rig r;
    auto& a = r.add(1);  // constructing the session marks the capture as a guest
    EffectCapture& cap = a.cap;
    bool blocked = !cap.record(SetStage{Q(1), 50});           // OnDeath script of a ghost NPC
    { EchoGuard g(&cap); blocked &= !cap.record(KillActor{Q(2)}); }  // remote/ghost event scope
    bool allowed;
    { LocalAction la(&cap); allowed = cap.record(AddItem{Q(5), 1, Recipient::dflt()}); }
    EffectCapture host;                                         // the host records everything
    bool hostOk = host.record(SetStage{Q(1), 50});
    return blocked && allowed && hostOk && cap.take().size() == 1;
}

TEST(session, guest_blocks_script_authored_shared_state) { CHECK(guestSuppressionHolds()); }

// ---- mutation twins (unit level) ------------------------------------------------------------

TEST(twin, unit_ghost) {
    Mutations m; m.noGuestSuppression = true;
    MutationScope s(m);
    CHECK(!guestSuppressionHolds());
}

TEST(twin, unit_echo) {
    // Defense in depth: the guest script-block is a second layer, so BOTH must be broken for an echo to leak.
    Mutations m; m.noEchoGuard = true; m.noGuestSuppression = true;
    MutationScope s(m);
    CHECK(!echoGuardHolds());
}

static bool idempotentDelivery() {
    Rig r;
    auto& a = r.add(1);
    r.connect(a);
    r.run(50);
    r.host.commit(0, quest(), {AddItem{Q(5), 1, Recipient::all()}});
    r.run(50);
    TxnBatch b; b.session = 1; b.txns.push_back(*r.host.log().get(1));
    r.net.endpoint(0)->send(1, encodeMessage(b));
    r.run(50);
    return a.sink.count(5) == 1;
}

TEST(session, redelivered_txn_is_idempotent) { CHECK(idempotentDelivery()); }

TEST(twin, unit_idempotency) {
    Mutations m; m.noIdempotency = true;
    MutationScope s(m);
    CHECK(!idempotentDelivery());
}
