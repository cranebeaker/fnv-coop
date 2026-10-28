// Policy, merge rules, story log, diff. Includes the unit-level mutation twins (group "twin").
#include <type_traits>
#include "fnvcoop/mutations.h"
#include "fnvcoop/policy.h"
#include "fnvcoop/storylog.h"
#include "minitest.h"
#include "sim.h"

using namespace fnvcoop;

namespace {
template <class T, class = void> struct has_to : std::false_type {};
template <class T> struct has_to<T, std::void_t<decltype(std::declval<T>().to)>> : std::true_type {};

FormRef Q(uint32_t i) { return FormRef("falloutnv.esm", i); }

EffectTxn txn(PlayerId origin, CauseKind k, std::vector<Effect> e) {
    EffectTxn t; t.origin = origin; t.cause.kind = k; t.effects = std::move(e); return t;
}

// Shared by the real test and its twin: returns true iff all merge expectations hold.
bool mergeRulesHold() {
    ShadowState s;
    EffectTxn t1 = txn(0, CauseKind::QuestResult, {SetStage{Q(1), 40}, SetObjective{Q(1), 0, 1}, KillActor{Q(10)}});
    t1.seq = 1; s.applyTxn(t1);
    EffectTxn t2 = txn(0, CauseKind::QuestResult, {SetStage{Q(1), 10}, SetObjective{Q(1), 0, 2}, KillActor{Q(11)}});
    t2.seq = 2; s.applyTxn(t2);
    bool ok = true;
    ok &= s.stages[Q(1)] == 40;                                  // stage = max
    ok &= s.objectives[{Q(1), 0}] == 2;                          // host-origin flags = last writer (exact state)
    EffectTxn t3 = txn(2, CauseKind::Intent, {SetObjective{Q(1), 0, 1}});
    t3.seq = 3; s.applyTxn(t3);
    ok &= s.objectives[{Q(1), 0}] == 3;                          // guest-origin flags only add bits
    EffectTxn t4 = txn(0, CauseKind::QuestResult, {SetObjective{Q(1), 0, 0}});
    t4.seq = 4; s.applyTxn(t4);
    ok &= s.objectives[{Q(1), 0}] == 0;                          // the host's hide / reset survives
    ok &= s.kills.count(Q(10)) == 1 && s.kills.count(Q(11)) == 1; // kills = union
    return ok;
}

// An objective the host hid (displayed bit cleared) stays hidden after a resync diff and for a late joiner.
bool objectiveHideSurvives() {
    ShadowState host;
    EffectTxn a = txn(0, CauseKind::QuestResult, {SetObjective{Q(7), 2, 3}}); a.seq = 1; host.applyTxn(a);   // displayed | completed-ish
    ShadowState client = host;                                                                                   // client in sync, then the host hides it
    EffectTxn b = txn(0, CauseKind::QuestResult, {SetObjective{Q(7), 2, 2}}); b.seq = 2; host.applyTxn(b);
    bool ok = host.objectives[{Q(7), 2}] == 2;
    std::vector<Effect> d;                                                                                       // resync: client catches up by diff
    ShadowState::diffEffects(client, host, 1, d);
    EffectTxn r = txn(0, CauseKind::QuestResult, d); r.seq = 3; client.applyTxn(r);
    ok &= client.objectives[{Q(7), 2}] == 2;
    ShadowState late;                                                                                            // late join: diff from nothing
    d.clear();
    ShadowState::diffEffects(late, host, 2, d);
    EffectTxn l = txn(0, CauseKind::QuestResult, d); l.seq = 1; late.applyTxn(l);
    ok &= late.objectives[{Q(7), 2}] == 2;
    return ok;
}
}  // namespace

TEST(core, policy_defaults_match_owner_decisions) {
    SharePolicy p = SharePolicy::defaults();
    CHECK(p.karma); CHECK(p.reputation); CHECK(p.questRewardsToAll); CHECK(p.maxPlayers == 4);
    EffectTxn t = txn(2, CauseKind::QuestResult,
                      {AddItem{Q(5), 1, Recipient::dflt()}, ModActorValue{3, 100, Recipient::dflt()},
                       ModKarma{-10, Recipient::dflt()}, ModReputation{Q(9), 5, 0, Recipient::dflt()}});
    CHECK(normalizeTxn(t, p) == 0);
    for (auto& e : t.effects) {
        Recipient r = std::visit([](auto& x) -> Recipient {
            if constexpr (has_to<std::decay_t<decltype(x)>>::value) return x.to; else return {}; }, e);
        CHECK(r.mode == Recipient::Mask);
        CHECK(r.mask == 0b1111);  // every player gets the full reward / shared karma / shared rep
    }
}

TEST(core, policy_non_reward_causes_go_to_origin_only) {
    SharePolicy p;
    EffectTxn t = txn(2, CauseKind::Loot, {AddItem{Q(5), 1, Recipient::dflt()}, ModActorValue{3, 5, Recipient::dflt()}});
    normalizeTxn(t, p);
    CHECK(std::get<AddItem>(t.effects[0]).to == Recipient::slots(1u << 2));
    CHECK(std::get<ModActorValue>(t.effects[1]).to == Recipient::slots(1u << 2));
    // karma/rep stay shared regardless of cause
    EffectTxn k = txn(2, CauseKind::Combat, {ModKarma{-5, Recipient::dflt()}});
    normalizeTxn(k, p);
    CHECK(std::get<ModKarma>(k.effects[0]).to == Recipient::slots(0b1111));
}

TEST(core, policy_flags_change_routing_and_drop_categories) {
    SharePolicy p;
    p.karma = false; p.reputation = false; p.questRewardsToAll = false; p.globals = false; p.kills = false; p.journal = false;
    EffectTxn t = txn(1, CauseKind::QuestResult,
                      {AddItem{Q(5), 1, Recipient::dflt()}, ModKarma{3, Recipient::dflt()}, ModReputation{Q(2), 1, 1, Recipient::dflt()},
                       SetGlobal{Q(3), 1}, KillActor{Q(4)}, SetStage{Q(5), 1}, EnableRef{Q(6)}});
    size_t dropped = normalizeTxn(t, p);
    CHECK(dropped == 3);  // global, kill, stage
    REQUIRE(t.effects.size() == 4);
    CHECK(std::get<AddItem>(t.effects[0]).to == Recipient::slots(0b10));
    CHECK(std::get<ModKarma>(t.effects[1]).to == Recipient::slots(0b10));
    CHECK(std::get<ModReputation>(t.effects[2]).to == Recipient::slots(0b10));
    CHECK(std::holds_alternative<EnableRef>(t.effects[3]));  // world facts not covered by a flag always flow
}

TEST(core, policy_explicit_recipients_clamped_and_origin_validated) {
    SharePolicy p; p.maxPlayers = 3;
    EffectTxn t = txn(1, CauseKind::Script, {AddItem{Q(1), 1, Recipient::slots(0b11111000)}, AddItem{Q(1), 1, Recipient::slots(0b110)}});
    CHECK(normalizeTxn(t, p) == 1);  // first targets only slots outside maxPlayers -> dropped
    CHECK(std::get<AddItem>(t.effects[0]).to == Recipient::slots(0b110));
    EffectTxn bad = txn(5, CauseKind::Script, {AddItem{Q(1), 1, Recipient::dflt()}});
    CHECK(normalizeTxn(bad, p) == kNormalizeInvalid);
}

TEST(core, shadow_merge_rules) {
    CHECK(mergeRulesHold());
    ShadowState s;
    EffectTxn t = txn(0, CauseKind::QuestResult, {SetGlobal{Q(1), 1.f}, SetGlobal{Q(1), 2.f}, EnableRef{Q(2)}, DisableRef{Q(2)},
                                                  KillActor{Q(3)}, ResurrectActor{Q(3)}});
    t.seq = 1; s.applyTxn(t);
    CHECK(s.globals[Q(1)] == 2.f);        // globals: host order, last wins
    CHECK(s.enabled[Q(2)] == false);       // enable/disable: host order
    CHECK(s.kills.count(Q(3)) == 0);       // explicit resurrect removes
}

TEST(core, shadow_output_is_post_merge) {
    ShadowState s;
    std::vector<Effect> out;
    EffectTxn a = txn(0, CauseKind::QuestResult, {SetStage{Q(1), 40}}); a.seq = 1;
    s.applyTxn(a, 1, &out);
    CHECK(out.size() == 1);
    out.clear();
    EffectTxn b = txn(0, CauseKind::QuestResult, {SetStage{Q(1), 20}, AddItem{Q(9), 2, Recipient::slots(0b100)}, AddItem{Q(9), 3, Recipient::slots(0b10)}}); b.seq = 2;
    s.applyTxn(b, 1, &out);
    REQUIRE(out.size() == 2);  // a lower stage the host ran for the first time is delivered (its done flag), item for slot 2 not for me, item for slot 1 is
    CHECK(std::get<SetStage>(out[0]).stage == 20);
    CHECK(std::get<AddItem>(out[1]).count == 3);
    out.clear();
    EffectTxn c = txn(0, CauseKind::QuestResult, {SetStage{Q(1), 20}}); c.seq = 3;
    s.applyTxn(c, 1, &out);
    CHECK(out.empty());        // the same lower stage again is a repeat, not a second delivery
}

// Story fix 2026-10-28 (docs/tools/story.md, "completed-stage bits"): VCG01 ran its stages 76 and 79 from a script before the host walked 9..75, so the client's
// stage was already 79 when the earlier stages arrived. The max merge swallowed all of them: the client had stage 80 but not one done flag in between
// (GetStageDone false). Shared by the test and its twin: true iff every stage the host ran comes out exactly once, in the host's order.
bool everyRunStageIsDelivered() {
    ShadowState client;
    std::vector<Effect> out;
    uint64_t seq = 0;
    auto run = [&](uint32_t stage) { EffectTxn t = txn(0, CauseKind::Console, {SetStage{Q(5), stage}}); t.seq = ++seq; client.applyTxn(t, 1, &out); };
    {   // the quest's own script jumps ahead (a stage run from another stage's result)
        EffectTxn t = txn(0, CauseKind::Script, {SetStage{Q(5), 8}, SetStage{Q(5), 76}, SetStage{Q(5), 79}}); t.seq = ++seq; client.applyTxn(t, 1, &out);
    }
    for (uint32_t stage : {9u, 10u, 15u, 17u, 75u}) run(stage);
    run(80);
    run(10);    // a repeat
    run(75);    // a repeat
    std::vector<uint32_t> got;
    for (auto& e : out) got.push_back(std::get<SetStage>(e).stage);
    const std::vector<uint32_t> want = {8, 76, 79, 9, 10, 15, 17, 75, 80};
    return got == want && client.stages[Q(5)] == 80;
}
TEST(core, every_stage_the_host_ran_is_delivered_even_below_the_current_one) { CHECK(everyRunStageIsDelivered()); }
TEST(twin, lower_stage_swallowed) {
    Mutations m; m.lowerStageSwallowed = true;
    { MutationScope s(m); CHECK(!everyRunStageIsDelivered()); }
    CHECK(everyRunStageIsDelivered());   // and restored afterwards
}

TEST(core, diff_effects_reconstruct_state) {
    Rng r(2024);
    for (int round = 0; round < 200; ++round) {
        ShadowState early, late;
        uint64_t n1 = r.range(0, 30), n2 = n1 + r.range(0, 60);
        std::vector<EffectTxn> txns;
        SharePolicy p;
        for (uint64_t i = 1; i <= n2; ++i) {
            EffectTxn t = txn(PlayerId(r.range(0, 3)), sim::randomCause(r).kind, {});
            for (uint32_t k = 0, m = r.range(1, 5); k < m; ++k) t.effects.push_back(sim::randomEffect(r, 4));
            normalizeTxn(t, p);
            t.seq = i;
            txns.push_back(t);
            late.applyTxn(t);
            if (i == n1) early = late;
        }
        PlayerId me = PlayerId(r.range(0, 3));
        std::vector<Effect> d;
        ShadowState::diffEffects(early, late, me, d);
        // Applying the diff as a txn (self-addressed) to `early` must reproduce `late` for shared state and my ledger.
        ShadowState rebuilt = early;
        // Objective flags in a diff are the exact host state, so they are applied host-origin; everything else is self-addressed.
        std::vector<Effect> objs, rest;
        for (auto& e : d) (std::holds_alternative<SetObjective>(e) ? objs : rest).push_back(e);
        EffectTxn ho = txn(kHostPlayer, CauseKind::Intent, objs); ho.seq = late.seq;
        rebuilt.applyTxn(ho);
        EffectTxn dt = txn(me, CauseKind::Intent, rest); dt.seq = late.seq;
        rebuilt.applyTxn(dt);
        for (unsigned s2 = 0; s2 < kMaxSlots; ++s2) if (s2 != me) { rebuilt.ledger[s2] = late.ledger[s2]; }
        // Barrier arrival masks are not recoverable from a diff (only the release is world-visible).
        for (auto& kv : late.barriers) CHECK(rebuilt.barriers[kv.first].released == kv.second.released);
        rebuilt.barriers = late.barriers;
        CHECK_MSG(rebuilt == late, rebuilt.describeDifference(late));
    }
}

TEST(core, storylog_append_range_and_compact) {
    StoryLog log;
    for (int i = 1; i <= 10; ++i) {
        uint64_t s = log.append(txn(0, CauseKind::QuestResult, {SetStage{Q(1), uint32_t(i)}}));
        CHECK(s == uint64_t(i));
    }
    CHECK(log.head() == 10); CHECK(log.firstSeq() == 1);
    CHECK(log.range(3, 4).size() == 4);
    CHECK(log.range(3, 4)[0].seq == 4);
    CHECK(log.get(10) != nullptr); CHECK(log.get(11) == nullptr); CHECK(log.get(0) == nullptr);
    ShadowState before = log.headState();
    log.compact(3);
    CHECK(log.size() == 3); CHECK(log.firstSeq() == 8); CHECK(log.head() == 10);
    CHECK(log.headState() == before);          // compaction never changes the head state
    CHECK(!log.canReplayFrom(5)); CHECK(log.canReplayFrom(7)); CHECK(log.canReplayFrom(10)); CHECK(!log.canReplayFrom(11));
    CHECK(log.range(5, 4).empty());
    CHECK(log.baseState().seq == 7);
}

TEST(core, storylog_save_load_roundtrip_and_garbage) {
    Rng r(8);
    StoryLog log;
    for (int i = 0; i < 100; ++i) {
        EffectTxn t = txn(PlayerId(r.range(0, 3)), CauseKind::QuestResult, {});
        for (uint32_t k = 0; k < 4; ++k) t.effects.push_back(sim::randomEffect(r, 4));
        normalizeTxn(t, SharePolicy());
        log.append(t);
    }
    log.compact(40);
    auto blob = log.save();
    StoryLog back;
    REQUIRE(back.load(blob.data(), blob.size()));
    CHECK(back.head() == log.head()); CHECK(back.firstSeq() == log.firstSeq());
    CHECK(back.headState() == log.headState());
    CHECK(back.save() == blob);
    // garbage / truncation must fail cleanly and leave the target untouched
    StoryLog target = back;
    for (size_t cut = 0; cut < blob.size(); cut += 7) {
        StoryLog t2;
        CHECK(!t2.load(blob.data(), cut));
    }
    for (int i = 0; i < 2000; ++i) {
        std::vector<uint8_t> g = blob;
        g[r.u32() % g.size()] ^= uint8_t(1u << (r.u32() % 8));
        StoryLog t2;
        t2.load(g.data(), g.size());  // may or may not parse; must not crash
    }
    CHECK(target.headState() == back.headState());
}

TEST(core, script_vars_barrier_and_round2_merges) {
    ShadowState s;
    std::vector<Effect> out;
    EffectTxn a = txn(1, CauseKind::DialogueResult, {SetScriptVar{Q(1), 2, 5.0}, SetScriptVar{Q(1), 2, 7.0}, SetScriptVar{Q(1), 3, 1.0}});
    a.seq = 1; s.applyTxn(a, 2, &out);
    double v2 = s.scriptVars[std::make_pair(Q(1), uint16_t(2))];
    CHECK(v2 == 7.0);  // host order, last wins; separate index is independent
    CHECK(s.scriptVars.size() == 2);
    // barrier: needs slots 0,1,2; releases exactly once when the last one arrives, whatever the arrival order
    out.clear();
    EffectTxn b0 = txn(2, CauseKind::Script, {BarrierArrive{Q(9), 0b111}}); b0.seq = 2; s.applyTxn(b0, 1, &out);
    EffectTxn b1 = txn(0, CauseKind::Script, {BarrierArrive{Q(9), 0b111}}); b1.seq = 3; s.applyTxn(b1, 1, &out);
    CHECK(!s.barriers[Q(9)].released);
    for (auto& e : out) CHECK(!std::holds_alternative<BarrierRelease>(e));
    EffectTxn b2 = txn(1, CauseKind::Script, {BarrierArrive{Q(9), 0b111}, BarrierArrive{Q(9), 0b111}}); b2.seq = 4; s.applyTxn(b2, 1, &out);
    CHECK(s.barriers[Q(9)].released);
    size_t rel = 0; for (auto& e : out) rel += std::holds_alternative<BarrierRelease>(e);
    CHECK(rel == 1);
    // controls are per player and routed to the initiator
    EffectTxn c = txn(1, CauseKind::Script, {SetPlayerControls{0b11, false, Recipient::dflt()}});
    normalizeTxn(c, SharePolicy()); c.seq = 5;
    std::vector<Effect> o2; s.applyTxn(c, 2, &o2);
    CHECK(o2.empty());
    CHECK((s.ledger[1].controlsOff == 0b11 && s.ledger[2].controlsOff == 0));
    // perks as rewards go to everyone; sticky one-shots are unions
    EffectTxn p = txn(0, CauseKind::QuestResult, {SetPerk{Q(3), 2, Recipient::dflt()}, SayOnceFlag{Q(4)}, SayOnceFlag{Q(4)}});
    normalizeTxn(p, SharePolicy()); p.seq = 6; s.applyTxn(p);
    CHECK((s.ledger[1].perks[Q(3)] == 2 && s.ledger[3].perks[Q(3)] == 2));
    CHECK(s.sayOnce.size() == 1);
}

// ---- mutation twins (unit level): the same assertions must FAIL under the bug ----------------

TEST(twin, unit_merge) {
    Mutations m; m.stageLastWins = true;
    { MutationScope s(m); CHECK(!mergeRulesHold()); }
    m = Mutations(); m.flagsAnd = true;
    { MutationScope s(m); CHECK(!mergeRulesHold()); }
    m = Mutations(); m.objectiveOr = true;
    { MutationScope s(m); CHECK(!mergeRulesHold()); }
    m = Mutations(); m.killsReplace = true;
    { MutationScope s(m); CHECK(!mergeRulesHold()); }
    CHECK(mergeRulesHold());  // and restored afterwards
}

TEST(core, host_objective_hide_survives_resync_and_late_join) { CHECK(objectiveHideSurvives()); }
TEST(twin, unit_objective_or) {
    Mutations m; m.objectiveOr = true;
    { MutationScope s(m); CHECK(!objectiveHideSurvives()); }
    CHECK(objectiveHideSurvives());
}
