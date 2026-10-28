// Runtime desync detection + targeted self-heal.
#include <cstdio>
#include <chrono>
#include <memory>
#include <thread>
#include "fnvcoop/session.h"
#include "minitest.h"

using namespace fnvcoop;

namespace {
FormRef Q(uint32_t i) { return FormRef("falloutnv.esm", i); }

// A stand-in for the game engine of one client: applies effects (force-set for baseline/repair) and can be dumped.
struct Engine : IEffectSink {
    ShadowState w;
    size_t repairs = 0;
    void apply(const Effect& e, const Context& c) override {
        if (c.repair) ++repairs;
        if (c.baseline) {
            std::visit([&](const auto& x) {
                using T = std::decay_t<decltype(x)>;
                if constexpr (std::is_same_v<T, SetStage>) w.stages[x.quest] = x.stage;
                else if constexpr (std::is_same_v<T, SetScriptVar>) w.scriptVars[{x.owner, x.index}] = x.value;
                else if constexpr (std::is_same_v<T, SetGlobal>) w.globals[x.global] = x.value;
                else if constexpr (std::is_same_v<T, KillActor>) w.kills.insert(x.actor);
                else if constexpr (std::is_same_v<T, ResurrectActor>) w.kills.erase(x.actor);
                else if constexpr (std::is_same_v<T, EnableRef>) w.enabled[x.ref] = true;
                else if constexpr (std::is_same_v<T, DisableRef>) w.enabled[x.ref] = false;
                else if constexpr (std::is_same_v<T, SetObjective>) w.objectives[{x.quest, x.objective}] = x.flags;
                else { EffectTxn t; t.effects = {e}; w.applyTxn(t); }
            }, e);
        } else {
            EffectTxn t; t.effects = {e}; w.applyTxn(t);
        }
    }
};

struct Rig {
    InMemoryNetwork net{12};
    HostSession host;
    std::vector<std::unique_ptr<Engine>> eng;
    std::vector<std::unique_ptr<EffectCapture>> caps;
    std::vector<std::unique_ptr<ClientSession>> cs;
    uint64_t now = 0;
    IWorker* worker = nullptr;
    explicit Rig(int clients = 1, uint32_t digestMs = 500, IWorker* w = nullptr) : host(*net.endpoint(0)), worker(w) {
        host.setWorker(w);
        for (int i = 0; i < clients; ++i) {
            eng.push_back(std::make_unique<Engine>());
            caps.push_back(std::make_unique<EffectCapture>());
            cs.push_back(std::make_unique<ClientSession>(*net.endpoint(PeerId(i + 1)), 0, uint64_t(10 + i), *eng.back(), caps.back().get()));
            Engine* e = eng.back().get();
            cs.back()->setBaselineProvider([e](ShadowState& w, uint64_t& h) { w = e->w; h = 1; return true; });
            cs.back()->setDigestInterval(digestMs);
            cs.back()->setWorker(w);
        }
        HostSession::DesyncOptions o;
        o.minDigestIntervalMs = 100;
        host.setDesyncOptions(o);
    }
    void connectAll() { for (auto& c : cs) c->connect(now, c->lastSession() + 1); }
    template <class F> void run(uint32_t ms, F&& each) {
        for (uint32_t t = 0; t < ms; t += 5) {
            now += 5; net.advance(now); host.tick(now);
            for (auto& c : cs) c->tick(now);
            each(now);
            if (worker) std::this_thread::sleep_for(std::chrono::microseconds(300));
            net.advance(now);
        }
    }
    void run(uint32_t ms) { run(ms, [](uint64_t) {}); }
    bool inSync(size_t i = 0) {
        // the client's engine world equals the host state in every shared entry
        return shardHashes(baselineEntries(eng[i]->w)) == shardHashes(baselineEntries(host.state()));
    }
    void traffic(int n) {
        for (int i = 0; i < n; ++i) {
            Cause c; c.kind = CauseKind::Script; c.source = Q(1);
            host.commit(0, c, {SetStage{Q(0x100 + i), uint32_t(i)}, SetScriptVar{Q(0x100 + i), uint16_t(i % 4), double(i)},
                               SetGlobal{Q(0x200 + i % 5), float(i)}, EnableRef{Q(0x300 + i % 7)}});
            run(20);
        }
    }
};

// Shared by the real test and its twin.
struct HealResult { bool healed = false; uint64_t ms = 0; size_t reports = 0; bool reportHasValues = false; };
HealResult corruptAndHeal(IWorker* w = nullptr) {
    Rig r(1, 500, w);
    r.connectAll(); r.run(400);
    r.traffic(60);
    r.run(1500);
    HealResult out;
    if (!r.inSync()) return out;   // baseline sanity: no false positives so far
    // silent corruption: a missed engine apply (stage) and a stray script write (kill)
    r.eng[0]->w.stages[Q(0x103)] += 7;
    r.eng[0]->w.kills.insert(Q(0x9999));
    uint64_t t0 = r.now;
    for (int i = 0; i < 1000 && !out.healed; ++i) {
        r.run(10);
        out.healed = r.inSync();
    }
    out.ms = r.now - t0;
    out.reports = r.host.desyncs().size();
    if (!r.host.desyncs().empty()) {
        for (auto& rep : r.host.desyncs())
            for (auto& e : rep.entries)
                if (e.client.find("(missing)") == std::string::npos && !e.host.empty() && !e.client.empty()) out.reportHasValues = true;
    }
    return out;
}
}  // namespace

TEST(desync, silent_corruption_is_detected_reported_and_healed_within_k_seconds) {
    HealResult h = corruptAndHeal();
    std::printf("    healed=%d in %llu ms, %zu report(s)\n", int(h.healed), (unsigned long long)h.ms, h.reports);
    CHECK(h.healed);
    CHECK(h.ms <= 3000);
    CHECK(h.reports >= 1);
    CHECK(h.reportHasValues);
}

TEST(desync, healing_works_with_a_worker_thread_too) {
    ThreadWorker worker;
    HealResult h = corruptAndHeal(&worker);
    worker.waitIdle();
    CHECK(h.healed);
    CHECK(h.reports >= 1);
    CHECK(h.reportHasValues);
}

TEST(desync, report_names_keys_values_and_last_txns_and_json_is_exposed) {
    Rig r(1, 500);
    r.connectAll(); r.run(400);
    r.traffic(30);
    r.run(1500);
    r.eng[0]->w.stages[Q(0x105)] = 4242;
    r.eng[0]->w.kills.insert(Q(0x9999));
    r.run(3000);
    REQUIRE(!r.host.desyncs().empty());
    bool sawStage = false, sawKill = false;
    for (auto& rep : r.host.desyncs()) {
        CHECK(rep.slot == 1);
        for (auto& e : rep.entries) {
            if (e.host.find("SetStage") != std::string::npos && e.client.find("4242") != std::string::npos) {
                sawStage = true;
                CHECK(!e.lastSeqs.empty());                 // which txns last touched the drifted key
            }
            if (e.clientOnly && e.client.find("KillActor") != std::string::npos) sawKill = true;
        }
    }
    CHECK(sawStage);
    CHECK(sawKill);
    std::string js = r.host.desyncsJson();
    CHECK(js.find("\"desyncs\":[{") != std::string::npos);
    CHECK(js.find("4242") != std::string::npos);
    CHECK(js.find("\"repairs\":") != std::string::npos);
    CHECK(r.inSync());
}

namespace {
// combat_proof 2026-10-22: the host polls object / NPC script vars (RefScriptVar: window timers, a ganger's death count). The real client dump
// (plugin/snapshot) has no ref vars, so a digest that compares them reports every one as "missing" and repairs it again every round.
struct RefVarRun { uint64_t ok = 0, mismatch = 0, repairs = 0, effectsRepaired = 0; size_t reports = 0; bool inSyncOtherwise = false; };
RefVarRun refVarsRun() {
    Rig r(1, 300);
    Engine* e = r.eng[0].get();
    r.cs[0]->setBaselineProvider([e](ShadowState& w, uint64_t& h) { w = e->w; w.refVars.clear(); h = 1; return true; });   // what the dump can carry
    r.connectAll(); r.run(400);
    r.traffic(20);
    for (int i = 0; i < 30; ++i) {   // the poll keeps sending a moving float timer for three window refs and one NPC counter
        Cause c; c.kind = CauseKind::Script; c.source = Q(2);
        r.host.commit(0, c, {RefScriptVar{Q(0x171CDB), 1, 8.0 + i * 0.08}, RefScriptVar{Q(0x1771EA), 1, 8.0 + i * 0.08},
                             RefScriptVar{Q(0x1771EB), 1, 8.0 + i * 0.08}, RefScriptVar{Q(0x104C75), 1, double(i % 4)}});
        r.run(100);
    }
    r.run(3000);
    RefVarRun o;
    o.ok = r.host.stats().digestsOk; o.mismatch = r.host.stats().digestsMismatch; o.repairs = r.host.stats().desyncRepairs;
    o.effectsRepaired = r.host.stats().desyncEffectsRepaired; o.reports = r.host.desyncs().size();
    o.inSyncOtherwise = r.inSync();
    return o;
}
}  // namespace

TEST(desync, ref_script_vars_are_not_digest_state_the_client_dump_cannot_carry_them) {
    RefVarRun o = refVarsRun();
    std::printf("    digests ok=%llu mismatch=%llu repairs=%llu effects=%llu\n", (unsigned long long)o.ok, (unsigned long long)o.mismatch,
                (unsigned long long)o.repairs, (unsigned long long)o.effectsRepaired);
    CHECK(o.ok > 5);
    CHECK(o.mismatch == 0);
    CHECK(o.repairs == 0 && o.effectsRepaired == 0);
    CHECK(o.reports == 0);
}

TEST(desync, twin_a_digest_that_compares_ref_script_vars_never_converges) {   // twin: the pre-fix digest
    MutationScope m([] { Mutations x; x.digestKeepsRefVars = true; x.repairForever = true; return x; }());   // the pre-fix digest, and no give-up
    RefVarRun o = refVarsRun();
    CHECK(o.mismatch > 5);
    CHECK(o.repairs > 5);
    CHECK(o.reports > 5);   // the same four keys, round after round
}

namespace {
// npc_sync / combat_proof / enemy_fight 2026-10-22: the host shadow holds the last SetWeather and SetTimeScale; the real client dump (plugin/snapshot)
// reads neither, so each digest called both "missing" and repaired them again, and every test step saw a new mismatch.
RefVarRun worldEnvRun() {
    Rig r(1, 301);
    Engine* e = r.eng[0].get();
    r.cs[0]->setBaselineProvider([e](ShadowState& w, uint64_t& h) { w = e->w; w.weatherSet = false; w.weatherForced = false; w.weather = FormRef(); w.timeScale = 0; h = 1; return true; });
    r.connectAll(); r.run(400);
    r.traffic(20);
    for (int i = 0; i < 6; ++i) {
        Cause c; c.kind = CauseKind::Script; c.source = Q(2);
        r.host.commit(0, c, {SetWeather{Q(0x1237D7 + i), false}, SetTimeScale{30.0f + float(i)}});
        r.run(500);
    }
    r.run(3000);
    RefVarRun o;
    o.ok = r.host.stats().digestsOk; o.mismatch = r.host.stats().digestsMismatch; o.repairs = r.host.stats().desyncRepairs;
    o.effectsRepaired = r.host.stats().desyncEffectsRepaired; o.reports = r.host.desyncs().size();
    o.inSyncOtherwise = r.inSync();
    return o;
}
}  // namespace

TEST(desync, weather_and_time_scale_are_not_digest_state_the_client_dump_cannot_read_them) {
    RefVarRun o = worldEnvRun();
    std::printf("    digests ok=%llu mismatch=%llu repairs=%llu effects=%llu\n", (unsigned long long)o.ok, (unsigned long long)o.mismatch,
                (unsigned long long)o.repairs, (unsigned long long)o.effectsRepaired);
    CHECK(o.ok > 3);
    CHECK(o.mismatch == 0);
    CHECK(o.repairs == 0 && o.effectsRepaired == 0);
    CHECK(o.reports == 0);
}

TEST(desync, twin_a_digest_that_compares_weather_and_time_scale_repairs_them_every_round) {   // twin: the pre-fix digest
    MutationScope m([] { Mutations x; x.digestKeepsWorldEnv = true; return x; }());
    RefVarRun o = worldEnvRun();
    CHECK(o.mismatch > 3);
    CHECK(o.repairs > 3);
    CHECK(o.reports > 3);
}

namespace {
// Goodsprings story run 2026-10-28: the host shadow holds every committed SetMapMarker (ShowMap from a stage result: Honest Hearts' intro reveals the
// Northern Passage); the real client dump (plugin/snapshot) has no marker reader, so each digest called the markers "missing" and repaired them again.
RefVarRun mapMarkerRun() {
    Rig r(1, 302);
    Engine* e = r.eng[0].get();
    r.cs[0]->setBaselineProvider([e](ShadowState& w, uint64_t& h) { w = e->w; w.mapMarkers.clear(); h = 1; return true; });   // what the dump can carry
    r.connectAll(); r.run(400);
    r.traffic(20);
    for (int i = 0; i < 6; ++i) {
        Cause c; c.kind = CauseKind::QuestResult; c.source = Q(2);
        r.host.commit(0, c, {SetMapMarker{Q(0x157F7D + i), kMarkerVisible}});
        r.run(500);
    }
    r.run(3000);
    RefVarRun o;
    o.ok = r.host.stats().digestsOk; o.mismatch = r.host.stats().digestsMismatch; o.repairs = r.host.stats().desyncRepairs;
    o.effectsRepaired = r.host.stats().desyncEffectsRepaired; o.reports = r.host.desyncs().size();
    o.inSyncOtherwise = r.inSync();
    return o;
}
}  // namespace

TEST(desync, map_markers_are_not_digest_state_the_client_dump_cannot_read_them) {
    RefVarRun o = mapMarkerRun();
    std::printf("    digests ok=%llu mismatch=%llu repairs=%llu effects=%llu\n", (unsigned long long)o.ok, (unsigned long long)o.mismatch,
                (unsigned long long)o.repairs, (unsigned long long)o.effectsRepaired);
    CHECK(o.ok > 3);
    CHECK(o.mismatch == 0);
    CHECK(o.repairs == 0 && o.effectsRepaired == 0);
    CHECK(o.reports == 0);
}

TEST(desync, twin_a_digest_that_compares_map_markers_repairs_them_every_round) {   // twin: the pre-fix digest
    MutationScope m([] { Mutations x; x.digestKeepsMapMarkers = true; return x; }());
    RefVarRun o = mapMarkerRun();
    CHECK(o.mismatch > 3);
    CHECK(o.repairs > 3);
    CHECK(o.reports > 3);
}

namespace {
// Goodsprings story run 2026-10-28 (S02): Doc's greeting adds VMQ01CourierNote; the host shadow holds the note, the real client dump has no note reader,
// so the digest called it "missing" and repaired it.
RefVarRun noteRun() {
    Rig r(1, 303);
    Engine* e = r.eng[0].get();
    r.cs[0]->setBaselineProvider([e](ShadowState& w, uint64_t& h) { w = e->w; w.notes.clear(); h = 1; return true; });   // what the dump can carry
    r.connectAll(); r.run(400);
    r.traffic(20);
    for (int i = 0; i < 6; ++i) {
        Cause c; c.kind = CauseKind::QuestResult; c.source = Q(2);
        r.host.commit(0, c, {AddNote{Q(0x105BCD + i)}});
        r.run(500);
    }
    r.run(3000);
    RefVarRun o;
    o.ok = r.host.stats().digestsOk; o.mismatch = r.host.stats().digestsMismatch; o.repairs = r.host.stats().desyncRepairs;
    o.effectsRepaired = r.host.stats().desyncEffectsRepaired; o.reports = r.host.desyncs().size();
    o.inSyncOtherwise = r.inSync();
    return o;
}
}  // namespace

TEST(desync, journal_notes_are_not_digest_state_the_client_dump_cannot_read_them) {
    RefVarRun o = noteRun();
    CHECK(o.ok > 3);
    CHECK(o.mismatch == 0);
    CHECK(o.repairs == 0 && o.effectsRepaired == 0);
    CHECK(o.reports == 0);
}

TEST(desync, twin_a_digest_that_compares_notes_repairs_them_every_round) {   // twin: the pre-fix digest
    MutationScope m([] { Mutations x; x.digestKeepsNotes = true; return x; }());
    RefVarRun o = noteRun();
    CHECK(o.mismatch > 3);
    CHECK(o.repairs > 3);
    CHECK(o.reports > 3);
}

TEST(desync, no_false_positives_under_loss_and_reordering_with_constant_traffic) {
    Rig r(2, 200);
    LinkParams lp; lp.minLatencyMs = 5; lp.maxLatencyMs = 90; lp.loss = 0.15; lp.duplicate = 0.1; lp.corrupt = 0.02;
    r.net.setLink(lp);
    r.connectAll(); r.run(1000);
    r.traffic(150);   // txns keep flowing while digests are taken
    r.net.setLink(LinkParams());
    r.run(3000);
    CHECK(r.host.stats().digestsReceived > 10);
    CHECK(r.host.stats().digestsOk > 5);
    CHECK(r.host.stats().desyncRepairs == 0);       // digests mid-traffic never reported a drift that was only lag
    CHECK(r.host.desyncs().empty());
    CHECK(r.inSync(0));
    CHECK(r.inSync(1));
}

TEST(desync, digests_are_rate_limited_per_client) {
    Rig r(1, 50);   // client digests every 50 ms ...
    HostSession::DesyncOptions o; o.minDigestIntervalMs = 1000;
    r.host.setDesyncOptions(o);          // ... host accepts one per second
    r.connectAll(); r.run(300);
    r.traffic(5);
    r.run(5000);
    CHECK(r.host.stats().digestsReceived > 40);
    CHECK(r.host.stats().digestsRateLimited > 30);
    CHECK(r.host.stats().digestsOk + r.host.stats().digestsMismatch + r.host.stats().digestsStale <= 8);
}

TEST(desync, repair_bandwidth_is_budgeted_and_still_converges) {
    Rig r(1, 200);
    HostSession::DesyncOptions o; o.minDigestIntervalMs = 100; o.maxRepairEffectsPerMin = 120; o.maxShardsPerRound = 8; o.maxRepairEffects = 256;
    r.host.setDesyncOptions(o);
    r.connectAll(); r.run(400);
    r.traffic(100);
    r.run(500);
    for (int i = 0; i < 100; ++i) r.eng[0]->w.stages[Q(0x100 + i)] += 3;
    for (int i = 0; i < 100; ++i) r.eng[0]->w.scriptVars[{Q(0x100 + i), uint16_t(i % 4)}] += 1.0;
    uint64_t t0 = r.now;
    r.run(4000);
    size_t early = r.eng[0]->repairs;
    CHECK(early > 0);
    CHECK(early <= 130);                               // budget (120/min + a little refill) caps the first seconds
    CHECK(r.host.stats().repairsDeferredBudget > 0);
    bool healed = false;
    for (int i = 0; i < 400 && !healed; ++i) { r.run(500); healed = r.inSync(); }   // up to ~200 virtual seconds
    CHECK(healed);
    CHECK(r.now - t0 > 4000);
}

TEST(desync, volatile_keys_never_trigger_reports) {
    volatileKeys().clear();
    volatileKeys().addVar(Q(0x105), 1);
    Rig r(1, 300);
    r.connectAll(); r.run(400);
    r.traffic(30);
    r.eng[0]->w.scriptVars[{Q(0x105), 1}] = 123456.0;      // a timer the client ticks locally
    r.run(3000);
    CHECK(r.host.desyncs().empty());
    CHECK(r.host.stats().desyncRepairs == 0);
    volatileKeys().clear();
}

TEST(desync, host_can_ask_for_an_immediate_digest_after_a_scene) {
    Rig r(1, 3600 * 1000);   // periodic digests effectively off
    r.connectAll(); r.run(400);
    r.traffic(20);
    r.run(500);
    r.eng[0]->w.stages[Q(0x104)] += 9;
    r.run(2000);
    CHECK(!r.inSync());                                     // nothing noticed it yet
    r.host.requestDigest();
    r.run(1500);
    CHECK(r.inSync());
}

TEST(desync, wire_roundtrip_of_sync_messages) {
    SyncDigest d; d.session = 1; d.epoch = 2; d.seq = 3; d.digestId = 4; d.shards.assign(kSyncShards, 0xABCD);
    ShardRequest rq{1, 4, 0x8000000000000001ull, 0xFF};
    ShardEntries se; se.session = 1; se.digestId = 4; se.truncated = true; se.entries = {SetStage{Q(1), 2}, SetScriptVar{Q(2), 3, 1.5}};
    SyncRepair rp; rp.session = 1; rp.epoch = 2; rp.digestId = 4; rp.effects = {KillActor{Q(9)}}; rp.revertKeys = {1, 2, 3};
    DigestRequest dr{1, 2};
    for (Message m : {Message(d), Message(rq), Message(se), Message(rp), Message(dr)}) {
        Bytes b = encodeMessage(m);
        REQUIRE(!b.empty());
        Message out;
        REQUIRE(decodeMessage(b, out) == DecodeStatus::Ok);
        CHECK(out.index() == m.index());
        CHECK(encodeMessage(out) == b);
    }
    d.shards.resize(5);   // a digest with the wrong shard count is invalid on the wire
    Message out;
    CHECK(decodeMessage(encodeMessage(d), out) != DecodeStatus::Ok);
}

TEST(twin, unit_heal_disabled) {
    HealResult ok = corruptAndHeal();
    CHECK(ok.healed);
    Mutations m; m.healDisabled = true;
    MutationScope s(m);
    HealResult bad = corruptAndHeal();
    CHECK(!bad.healed);          // the drift persists ...
    CHECK(bad.reports >= 1);     // ... but it is detected and reported
}

namespace {
// A world bigger than one shard reply (kMaxShardEntries): 6000 script vars spread over the 16 script-var shards, ~375 each, and a
// drifted value in every shard. When the host asked for all 8 bad shards at once the guest's reply was cut at 1000 entries and every
// key cut off counted as "(missing)" and was resent each round, eating the repair budget (two-copy fight test 2026-10-21: 24 digests
// in a row, 1244 effects each, never settled).
struct BigDriftResult { bool healed = false; uint64_t falseMissing = 0; };
BigDriftResult bigWorldDrift() {
    Rig r(1, 500);
    r.connectAll(); r.run(400);
    for (int b = 0; b < 60; ++b) {
        Cause c; c.kind = CauseKind::Script; c.source = Q(1);
        std::vector<Effect> fx;
        for (int i = 0; i < 100; ++i) fx.push_back(SetScriptVar{Q(0x1000 + b * 100 + i), 0, double(b * 100 + i)});
        r.host.commit(0, c, fx);
        r.run(20);
    }
    r.run(2000);
    BigDriftResult out;
    if (!r.inSync()) return out;
    for (int i = 0; i < 6000; i += 50) r.eng[0]->w.scriptVars[{Q(0x1000 + i), 0}] = -1.0;   // 120 drifted values, spread over every shard
    for (int i = 0; i < 1500 && !out.healed; ++i) {
        r.run(20);
        out.healed = r.inSync();
    }
    for (auto& rep : r.host.desyncs())
        for (auto& e : rep.entries)
            if (e.client == "(missing)") ++out.falseMissing;
    return out;
}
}  // namespace

TEST(desync, drift_in_a_world_bigger_than_one_shard_reply_is_healed_without_false_missing_keys) {
    BigDriftResult h = bigWorldDrift();
    std::printf("    healed=%d falseMissing=%llu\n", int(h.healed), (unsigned long long)h.falseMissing);
    CHECK(h.healed);
    CHECK(h.falseMissing == 0);
}

TEST(desync, twin_asking_for_every_bad_shard_at_once_resends_the_cut_off_keys_as_missing) {
    Mutations m; m.overfullShardRequest = true;
    MutationScope s(m);
    BigDriftResult bad = bigWorldDrift();
    std::printf("    twin: healed=%d falseMissing=%llu\n", int(bad.healed), (unsigned long long)bad.falseMissing);
    CHECK(bad.falseMissing > 0);
}
