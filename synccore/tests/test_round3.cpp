// Vocabulary round 3 (game-wide census): created refs, followers, object state, ref script vars, ref inventories, journal, scripted saves,
// weather and time, control-lock release on disconnect, GuestLocalRun. Merge rules, routing, baseline, session behaviour, twins.
#include "fnvcoop/baseline.h"
#include "fnvcoop/mutations.h"
#include "fnvcoop/rules.h"
#include "fnvcoop/shadow.h"
#include "minitest.h"
#include "rejoin_rig.h"

using namespace fnvcoop;
using namespace rejoin;

namespace {
std::vector<Effect> applyAll(ShadowState& s, PlayerId origin, std::vector<Effect> effects) {
    EffectTxn t; t.seq = s.seq + 1; t.origin = origin; t.effects = std::move(effects);
    std::vector<Effect> out;
    s.applyTxn(t, kNoPlayer, &out);
    return out;
}

// ---- created refs ---------------------------------------------------------------------------------------------------------------
bool createdRefsHold() {
    ShadowState s;
    const uint32_t a = kCreatedRefPrefix | 5, b = kCreatedRefPrefix | 6;
    if (!isCreatedRef(createdRef(a))) return false;
    if (applyAll(s, 0, {SpawnRef{a, Q(1), Q(2), 1, 2, 3, false}}).size() != 1) return false;
    if (!applyAll(s, 0, {SpawnRef{a, Q(1), Q(2), 1, 2, 3, false}}).empty()) return false;          // identical respawn is a no-op
    if (s.createdCounter != 5 || s.spawned.size() != 1) return false;
    applyAll(s, 0, {DespawnRef{a}});
    if (!s.spawned.empty() || !s.despawned.count(createdRef(a))) return false;
    applyAll(s, 0, {SpawnRef{a, Q(1), Q(2), 9, 9, 9, true}});                                       // a late / replayed spawn must not resurrect it
    if (!s.spawned.empty()) return false;
    applyAll(s, 0, {DespawnRef{b}, SpawnRef{b, Q(1), Q(2), 0, 0, 0, false}});                      // despawn before spawn (reordered): stays gone
    if (!s.spawned.empty() || s.createdCounter != 6) return false;
    return true;
}

// ---- object state -----------------------------------------------------------------------------------------------------------------
bool objectStateHolds() {
    ShadowState s;
    ObjectState open; open.ref = Q(7); open.mask = kObjOpen; open.open = true;
    if (applyAll(s, 0, {open}).size() != 1 || !s.objects.count(Q(7))) return false;
    open.open = false;
    applyAll(s, 0, {open});
    if (!s.objects.empty()) return false;                                                           // closed == default: canonical, not stored
    ObjectState d; d.ref = Q(7); d.mask = kObjDestroyed | kObjActivated; d.destroyed = true; d.activatedOnce = true;
    applyAll(s, 0, {d});
    d.destroyed = false; d.activatedOnce = false;
    if (!applyAll(s, 0, {d}).empty()) return false;                                                 // sticky: a later "false" changes nothing
    if (!s.objects.at(Q(7)).destroyed || !s.objects.at(Q(7)).activated) return false;
    ObjectState lk; lk.ref = Q(8); lk.mask = kObjLock; lk.locked = true; lk.level = 75;
    applyAll(s, 0, {lk});
    if (s.locks.at(Q(8)) != std::make_pair(true, 75)) return false;
    ObjectState ul = lk; ul.locked = false;
    applyAll(s, 0, {ul});
    return s.locks.at(Q(8)) == std::make_pair(false, 0);
}

// ---- inventory on non-player refs -------------------------------------------------------------------------------------------------
bool refInventoryHolds() {
    ShadowState s;
    applyAll(s, 0, {RefAddItem{Q(1), Q(10), 3}, RefRemoveItem{Q(1), Q(10), 3}});
    if (!s.refInv.empty()) return false;                                                            // nets out
    applyAll(s, 0, {RefAddItem{Q(1), Q(10), 5}, RefAddItem{Q(1), Q(11), 2}});
    applyAll(s, 0, {RemoveAllItems{Q(1)}});
    if (!s.refInv.at(Q(1)).cleared || !s.refInv.at(Q(1)).delta.empty()) return false;
    applyAll(s, 0, {RefAddItem{Q(1), Q(12), 4}});
    const RefInv& i = s.refInv.at(Q(1));
    if (!i.cleared || i.delta.size() != 1 || i.delta.at(Q(12)) != 4) return false;
    // a joiner replays the same inventory through the diff
    ShadowState j;
    std::vector<Effect> d;
    ShadowState::diffEffects(ShadowState(), s, 1, d);
    EffectTxn t; t.seq = 1; t.effects = d;
    j.applyTxn(t);
    j.seq = s.seq;
    return j == s;
}

// ---- journal / weather / time / events --------------------------------------------------------------------------------------------
bool journalWeatherHolds() {
    ShadowState s;
    // topic, note 2, note 3, its removal (each a real state change, relayed in order), marker; the duplicate topic is dropped
    if (applyAll(s, 0, {AddTopic{Q(1)}, AddTopic{Q(1)}, AddNote{Q(2)}, AddNote{Q(3)}, RemoveNote{Q(3)}, SetMapMarker{Q(4), 3}}).size() != 5) return false;
    if (s.topics.size() != 1 || s.notes.size() != 1 || s.mapMarkers.at(Q(4)) != 3) return false;
    if (applyAll(s, 0, {SetWeather{Q(5), false}, SetTimeScale{30}}).size() != 2) return false;
    if (!applyAll(s, 0, {SetWeather{Q(5), false}, SetTimeScale{30}}).empty()) return false;
    if (applyAll(s, 0, {SetWeather{Q(5), true}}).size() != 1) return false;                          // force flag is part of the state
    // events: delivered every time, never stored
    ShadowState before = s;
    before.seq += 1;
    auto ev = applyAll(s, 0, {RequestSave{kSaveSystem}, AdvanceTime{480}, RequestSave{kSaveSystem}});
    return ev.size() == 3 && s == before;
}

// ---- baseline / digest for the new covered state -----------------------------------------------------------------------------------
bool baselineCoversRound3() {
    ShadowState host;
    ObjectState o; o.ref = Q(7); o.mask = kObjOpen | kObjDestroyed; o.open = true; o.destroyed = true;
    applyAll(host, 0, {o, RefScriptVar{Q(8), 2, 1.5}, AddTopic{Q(9)}, AddNote{Q(10)}, SetMapMarker{Q(11), 1}, SetWeather{Q(12), false}, SetTimeScale{20},
                       CompanionOwner{Q(13), 1, true, 1}});
    ShadowState client;                                                                             // an empty world: everything is missing
    BaselineDiff d = diffBaseline(host, summarizeWorld(client, 1), SharePolicy(), 1, false, 0, false, {});
    size_t found = 0;
    for (auto& e : d.effects) {
        found += std::holds_alternative<ObjectState>(e) + std::holds_alternative<RefScriptVar>(e) + std::holds_alternative<AddTopic>(e) +
                 std::holds_alternative<AddNote>(e) + std::holds_alternative<SetMapMarker>(e) + std::holds_alternative<SetWeather>(e) +
                 std::holds_alternative<SetTimeScale>(e) + std::holds_alternative<CompanionOwner>(e);
    }
    if (found != 8) return false;
    // applying the diff makes the digests equal, and the fast digest agrees with the entry-based one
    EffectTxn t; t.seq = 1; t.effects = d.effects;
    client.applyTxn(t);
    if (shardHashesOf(client) != shardHashesOf(host)) return false;
    if (shardHashesOf(host) != shardHashes(baselineEntries(host))) return false;
    // a different object state changes the digest
    ObjectState c; c.ref = Q(7); c.mask = kObjOpen; c.open = false;
    applyAll(client, 0, {c});
    return shardHashesOf(client) != shardHashesOf(host);
}

// ---- RefAngle: per (ref, axis) state, the committed (host) order wins ----------------------------------------------------------------
bool refAngleHolds() {
    ShadowState s;
    if (applyAll(s, 0, {RefAngle{Q(1), kAxisZ, 90.f}, RefAngle{Q(1), kAxisX, 10.f}}).size() != 2) return false;   // axes are independent
    if (!applyAll(s, 0, {RefAngle{Q(1), kAxisZ, 90.f}}).empty()) return false;                                     // same value is a no-op
    if (applyAll(s, 0, {RefAngle{Q(1), kAxisZ, 180.f}}).size() != 1 || s.angles.at({Q(1), kAxisZ}) != 180.f) return false;   // later commit wins
    if (applyAll(s, 1, {RefAngle{Q(1), kAxisZ, 270.f}}).size() != 1 || s.angles.at({Q(1), kAxisZ}) != 270.f) return false;
    if (s.angles.at({Q(1), kAxisX}) != 10.f || s.angles.size() != 2) return false;
    // pre-image: the first `before` is the original; writing it back removes the axis, and a diff restores an axis the target never rotated
    ShadowState p;
    applyAll(p, 0, {RefAngle{Q(3), kAxisZ, 90.f, true, 15.f}});
    ShadowState rotated = p;
    applyAll(rotated, 0, {RefAngle{Q(3), kAxisZ, 120.f, true, 90.f}});                                              // 90 is not the original
    if (rotated.angleOrig.at({Q(3), kAxisZ}) != 15.f) return false;
    std::vector<Effect> back;
    ShadowState::diffEffects(rotated, ShadowState(), 1, back);                                                      // rotated -> never rotated
    if (back.size() != 1 || std::get<RefAngle>(back[0]).degrees != 15.f) return false;
    EffectTxn bt; bt.seq = rotated.seq + 1; bt.effects = back;
    rotated.applyTxn(bt);
    if (!rotated.angles.empty() || !rotated.angleOrig.empty()) return false;
    if (!applyAll(rotated, 0, {RefAngle{Q(3), kAxisZ, 15.f, true, 15.f}}).empty()) return false;                   // rotating to where it is is a no-op
    // a joiner rebuilds the same angles through the diff
    ShadowState j;
    std::vector<Effect> d;
    ShadowState::diffEffects(ShadowState(), s, 1, d);
    EffectTxn t; t.seq = 1; t.effects = d;
    j.applyTxn(t);
    j.seq = s.seq;
    return j == s;
}
bool refAngleBaselineHolds() {
    ShadowState host;
    applyAll(host, 0, {RefAngle{Q(2), kAxisY, 45.f}, RefAngle{Q(2), kAxisZ, 5.f}});
    ShadowState client;
    BaselineDiff d = diffBaseline(host, summarizeWorld(client, 1), SharePolicy(), 1, false, 0, false, {});
    size_t found = 0;
    for (auto& e : d.effects) found += std::holds_alternative<RefAngle>(e);
    if (found != 2) return false;
    EffectTxn t; t.seq = 1; t.effects = d.effects;
    client.applyTxn(t);
    if (client.angles != host.angles || shardHashesOf(client) != shardHashesOf(host)) return false;
    if (shardHashesOf(host) != shardHashes(baselineEntries(host))) return false;
    applyAll(client, 0, {RefAngle{Q(2), kAxisY, 46.f}});                                                              // a different angle changes the digest
    BaselineDiff d2 = diffBaseline(host, summarizeWorld(client, 1), SharePolicy(), 1, false, 0, false, {});
    return shardHashesOf(client) != shardHashesOf(host) && d2.effects.size() == 1 && std::get<RefAngle>(d2.effects[0]).degrees == 45.f;
}

// ---- pre-images of weather, time scale, ref script vars, map markers, locks ----------------------------------------------------------
// A client changed each of them; the host loaded an older save that never did. The diff / baseline revert must put the originals back.
bool originsHold() {
    const FormRef w0 = Q(0x40), w1 = Q(0x41), r = Q(0x50), m = Q(0x60);
    ShadowState client;
    SetWeather sw{w1, true}; sw.hasBefore = true; sw.before = w0; sw.beforeForce = false;
    SetTimeScale ts{40.f}; ts.hasBefore = true; ts.before = 20.f;
    RefScriptVar rv{r, 2, 7.5}; rv.hasBefore = true; rv.before = 1.0;
    SetMapMarker mm{m, 3}; mm.hasBefore = true; mm.before = 0;
    SetLock lk{r, true, 60}; lk.hasBefore = true; lk.beforeLocked = false; lk.beforeLevel = 0;
    ObjectState os; os.ref = m; os.mask = kObjLock; os.locked = true; os.level = 25; os.hasBefore = true; os.beforeLocked = true; os.beforeLevel = 80;
    if (applyAll(client, 0, {sw, ts, rv, mm, lk, os}).size() != 6) return false;
    if (client.origins.size() != 6) return false;
    // diff to a host that never held them: every original comes back, exactly once each
    std::vector<Effect> back;
    ShadowState::diffEffects(client, ShadowState(), 1, back);
    size_t restored = 0;
    for (auto& e : back) {
        if (auto* x = std::get_if<SetWeather>(&e)) restored += x->weather == w0 && !x->force;
        else if (auto* x = std::get_if<SetTimeScale>(&e)) restored += x->scale == 20.f;
        else if (auto* x = std::get_if<RefScriptVar>(&e)) restored += x->value == 1.0 && x->index == 2;
        else if (auto* x = std::get_if<SetMapMarker>(&e)) restored += x->flags == 0 && x->marker == m;
        else if (auto* x = std::get_if<SetLock>(&e)) restored += x->ref == r && !x->locked;
    }
    if (restored != 5) return false;
    ShadowState c2 = client;
    EffectTxn bt; bt.seq = c2.seq + 1; bt.effects = back;
    c2.applyTxn(bt);
    if (c2.weatherSet || c2.timeScale != 0 || !c2.refVars.empty() || !c2.mapMarkers.empty()) return false;   // back to "never set"
    if (c2.locks.at(r) != std::make_pair(false, 0) || c2.locks.at(m) != std::make_pair(true, 80)) return false;   // locks keep an entry holding the original
    // the baseline path: the client's own world is what carries the pre-images
    BaselineDiff d = diffBaseline(ShadowState(), summarizeWorld(client, 1), SharePolicy(), 1, false, 0, false, {});
    if (d.revertKeys.size() < 6) return false;
    size_t reverts = 0;
    for (uint64_t k : d.revertKeys) { Effect e; reverts += revertEffectFor(client, k, e); }
    if (reverts != d.revertKeys.size()) return false;
    // origins survive the sidecar / snapshot encoding
    Welcome wl; wl.session = 1; wl.slot = 1; wl.headSeq = client.seq; wl.hasSnapshot = true; wl.snapshot = client;
    Message dec;
    if (decodeMessage(encodeMessage(wl), dec) != DecodeStatus::Ok) return false;
    auto* ow = std::get_if<Welcome>(&dec);
    return ow && ow->snapshot.sameOrigins(client) && ow->snapshot == client;
}

// ---- ref inventory: a clear carries its contents; a rebase undoes the clear and takes back the client's own deltas ---------------------
bool refInvPreimageHolds() {
    const FormRef c1 = Q(0x70), c2 = Q(0x71), x = Q(0x80), y = Q(0x81), z = Q(0x82);
    ShadowState client;
    RemoveAllItems clr; clr.ref = c1; clr.hasBefore = true; clr.items = {{x, 3}, {y, 2}};
    if (applyAll(client, 0, {clr, RefAddItem{c1, z, 4}, RefAddItem{c2, z, 4}}).size() != 3) return false;
    if (!client.refInv.at(c1).cleared || !client.refInv.at(c1).hasPre || client.refInv.at(c1).pre.at(x) != 3) return false;
    ShadowState host;
    applyAll(host, 0, {RefAddItem{c2, z, 7}});                                   // the loaded save: c1 untouched, c2 holds 7 not 4
    BaselineDiff d = diffBaseline(host, summarizeWorld(client, 1), SharePolicy(), 1, false, 0, false, {});
    // apply like the client does: all reverts (computed from its own world), then the host's effects
    ShadowState world = client;
    std::vector<Effect> fx;
    for (uint64_t k : d.revertKeys) { Effect e; if (revertEffectFor(client, k, e)) fx.push_back(e); }
    for (auto& e : d.effects) fx.push_back(e);
    EffectTxn t; t.seq = world.seq + 1; t.effects = fx;
    world.applyTxn(t);
    if (world.refInv.count(c1)) return false;                                    // c1 is back to untouched (cleared flag and delta gone)
    if (world.refInv.at(c2).delta.at(z) != 7 || world.refInv.at(c2).delta.size() != 1) return false;   // 4 taken back, 7 added: no double count
    // the shadow-to-shadow diff does the same
    std::vector<Effect> back;
    ShadowState::diffEffects(client, host, 1, back);
    size_t undos = 0;
    for (auto& e : back) if (auto* u = std::get_if<RemoveAllItems>(&e)) undos += u->undo && u->items.size() == 2;
    return undos == 1;
}

// ---- created refs: a rebase undoes a spawn the loaded save never had, and brings back a ref the client despawned -----------------------
bool spawnRebaseHolds() {
    const uint32_t a = kCreatedRefPrefix | 5, b = kCreatedRefPrefix | 6, c = kCreatedRefPrefix | 7;
    ShadowState client;
    applyAll(client, 0, {SpawnRef{a, Q(1), Q(2), 1, 2, 3, false}, SpawnRef{b, Q(1), Q(2), 4, 5, 6, true}, DespawnRef{b}, SpawnRef{c, Q(1), Q(2), 0, 0, 0, false}, DespawnRef{c}});
    ShadowState host;   // older save: b alive, a and c never existed
    applyAll(host, 0, {SpawnRef{b, Q(1), Q(2), 4, 5, 6, true}});
    BaselineDiff d = diffBaseline(host, summarizeWorld(client, 1), SharePolicy(), 1, false, 0, false, {});
    ShadowState world = client;
    std::vector<Effect> fx;
    for (uint64_t k : d.revertKeys) { Effect e; if (revertEffectFor(client, k, e)) fx.push_back(e); }
    for (auto& e : d.effects) fx.push_back(e);
    EffectTxn t; t.seq = world.seq + 1; t.effects = fx;
    world.applyTxn(t);
    if (world.spawned.size() != 1 || !world.spawned.count(createdRef(b)) || !world.despawned.empty()) return false;   // a gone, b back, no tombstones
    // the id counter was rewound on the host: the same id may be spawned again and must not be refused
    applyAll(world, 0, {SpawnRef{a, Q(7), Q(2), 9, 9, 9, false}});
    if (!world.spawned.count(createdRef(a))) return false;
    std::vector<Effect> back;
    ShadowState::diffEffects(client, host, 1, back);
    size_t forgets = 0;
    for (auto& e : back) if (auto* x = std::get_if<DespawnRef>(&e)) forgets += x->forget;
    return forgets == 3;   // a (spawned), c (tombstone) and b (tombstone) are dropped, then b is spawned again
}
}  // namespace

TEST(round3, ref_angle_is_per_axis_state_and_the_committed_order_wins) { CHECK(refAngleHolds()); }
TEST(round3, ref_angle_is_in_the_baseline_and_digest) { CHECK(refAngleBaselineHolds()); }
TEST(twin, round3_angle_preimage_ignored) {
    MutationScope m([] { Mutations x; x.anglePreimageIgnored = true; return x; }());
    CHECK(!refAngleHolds());
}
TEST(round3, preimages_restore_weather_time_vars_markers_and_locks) { CHECK(originsHold()); }
TEST(round3, ref_inventory_clear_is_undone_by_a_rebase_without_double_counting) { CHECK(refInvPreimageHolds()); }
TEST(twin, round3_ref_inv_preimage_ignored) {
    MutationScope m([] { Mutations x; x.originsIgnored = true; return x; }());
    CHECK(!refInvPreimageHolds());
}
TEST(round3, spawn_rebase_forgets_spawns_and_tombstones_the_loaded_save_never_had) { CHECK(spawnRebaseHolds()); }
TEST(twin, round3_spawn_rebase_ignored) {
    MutationScope m([] { Mutations x; x.originsIgnored = true; return x; }());
    CHECK(!spawnRebaseHolds());
}
TEST(twin, round3_origins_ignored) {
    MutationScope m([] { Mutations x; x.originsIgnored = true; return x; }());
    CHECK(!originsHold());
}
TEST(twin, round3_angle_keeps_first) {
    MutationScope m([] { Mutations x; x.angleKeepsFirst = true; return x; }());
    CHECK(!refAngleHolds());
}

TEST(round3, created_refs_are_tombstoned_and_never_resurrected) { CHECK(createdRefsHold()); }
TEST(round3, object_state_merges_sticky_flags_and_locks) { CHECK(objectStateHolds()); }
TEST(round3, ref_inventories_net_clear_and_replay_through_the_diff) { CHECK(refInventoryHolds()); }
TEST(round3, journal_weather_time_and_events) { CHECK(journalWeatherHolds()); }
TEST(round3, baseline_and_digest_cover_the_new_state) { CHECK(baselineCoversRound3()); }

TEST(twin, round3_spawn_resurrects) {
    MutationScope m([] { Mutations x; x.spawnResurrects = true; return x; }());
    CHECK(!createdRefsHold());
}
TEST(twin, round3_sticky_objects_reset) {
    MutationScope m([] { Mutations x; x.stickyObjectsReset = true; return x; }());
    CHECK(!objectStateHolds());
}
TEST(twin, round3_ref_inv_no_clear) {
    MutationScope m([] { Mutations x; x.refInvNoClear = true; return x; }());
    CHECK(!refInventoryHolds());
}

// ---- session level ----------------------------------------------------------------------------------------------------------------
namespace {
Cause cause(CauseKind k, uint32_t src = 1) { Cause c; c.kind = k; c.source = Q(src); return c; }

// Routing: UI focus is per player; quest-reward perks go to everyone, companion perks to the owner only; faction rank is shared;
// journal entries are shared.
bool routingHolds(std::string* why) {
    auto fail = [&](const char* w) { if (why) *why = w; return false; };
    Rig r(3);
    r.addClient(1); r.addClient(2);
    r.joinAll(); r.run(1500);
    if (!r.allLive()) return fail("not live");
    r.host.commit(2, cause(CauseKind::Script), {ForceActiveQuest{Q(50), Recipient::dflt()}});               // client slot 2 focuses a quest
    r.host.commit(0, cause(CauseKind::QuestResult), {SetPerk{Q(60), 1, Recipient::dflt()}});                // quest-reward perk: everybody
    r.host.commit(1, cause(CauseKind::Script, 9), {SetPerk{Q(61), 1, Recipient::dflt()}});                   // companion recruit perk: the owner only
    r.host.commit(2, cause(CauseKind::Script), {SetFactionRank{Q(70), Q(71), 2}, AddTopic{Q(72)}, AddNote{Q(73)}, SetMapMarker{Q(74), 3}});
    r.run(800);
    auto& a = r.cl[0]->sink; auto& b = r.cl[1]->sink;                                                       // slots 1 and 2
    if (b.countSeen(44) != 1 || a.countSeen(44) != 0) return fail("active quest not per player");
    if (r.host.state().ledger[2].activeQuest != Q(50) || r.host.state().ledger[1].hasActive) return fail("active quest ledger");
    if (a.countSeen(29) != 2) return fail("perk count");                                                    // reward + own companion perk
    if (b.countSeen(29) != 1) return fail("the companion perk leaked to a player who is not the owner");
    if (r.host.state().ledger[1].perks.count(Q(61)) == 0 || r.host.state().ledger[2].perks.count(Q(61)) != 0) return fail("companion perk ledger");
    if (r.host.state().ledger[1].perks.count(Q(60)) == 0 || r.host.state().ledger[2].perks.count(Q(60)) == 0) return fail("quest-reward perk not given to all");
    for (auto& c : r.cl) {
        if (c->cs->state() != r.host.state()) return fail("diverged");
        if (c->sink.countSeen(28) != 1 || c->sink.countSeen(40) != 1 || c->sink.countSeen(41) != 1 || c->sink.countSeen(43) != 1) return fail("shared faction/journal not delivered to everyone");
    }
    return true;
}

bool followerCapsHold(std::string* why) {
    auto fail = [&](const char* w) { if (why) *why = w; return false; };
    Rig r(4);
    r.addClient(1); r.addClient(2);
    r.joinAll(); r.run(1500);
    auto recruit = [&](PlayerId owner, uint32_t npc, uint8_t kind) {
        return r.host.commit(owner, cause(CauseKind::Script), {CompanionOwner{Q(npc), owner, true, kind}});
    };
    if (!recruit(1, 100, 0)) return fail("first humanoid refused");
    if (recruit(1, 101, 0)) return fail("second humanoid of one owner accepted");
    if (!recruit(1, 102, 1)) return fail("critter refused");
    if (recruit(1, 103, 1)) return fail("second critter of one owner accepted");
    if (!recruit(2, 104, 0) || !recruit(2, 105, 1)) return fail("second owner refused");
    if (recruit(0, 106, 0)) return fail("party total exceeded (default 4)");
    r.host.commit(1, cause(CauseKind::Script), {CompanionOwner{Q(100), 1, false, 0}});                      // dismiss frees the slot
    if (!recruit(0, 106, 0)) return fail("recruit after a dismiss refused");
    if (r.host.stats().followerCapDrops != 3) return fail("cap drops not counted");
    r.run(600);
    for (auto& c : r.cl) if (c->cs->state() != r.host.state()) return fail("diverged");
    if (r.host.state().companionOwner.size() != 4 || r.host.state().companionOwner.count(Q(100))) return fail("companion state");
    // a configurable party total
    HostSession::Options o; o.followers.partyTotal = 2;
    Rig r2(5, false, 40, o);
    r2.addClient(1); r2.joinAll(); r2.run(1500);
    r2.host.commit(1, cause(CauseKind::Script), {CompanionOwner{Q(1), 1, true, 0}, CompanionOwner{Q(2), 1, true, 1}});
    if (r2.host.commit(0, cause(CauseKind::Script), {CompanionOwner{Q(3), 0, true, 0}})) return fail("configured party total ignored");
    return true;
}

// W3a item 6: the companion vet runs inside the commit; what it takes out of a txn is neither logged nor sent, and the cap logic still sees the rest.
bool companionVetHolds(std::string* why) {
    auto fail = [&](const char* w) { if (why) *why = w; return false; };
    Rig r(7);
    r.addClient(1); r.addClient(2);
    r.joinAll(); r.run(1500);
    // stand-in for plugin/companions: a txn that recruits npc 201 for owner 2 and dismisses npc 200 (owner 1's) in the same breath is a swap: the dismissal goes
    r.host.companionVet = [](EffectTxn& t) {
        bool recruit = false;
        for (auto& e : t.effects) if (auto* c = std::get_if<CompanionOwner>(&e)) recruit = recruit || c->active;
        size_t before = t.effects.size();
        if (recruit)
            t.effects.erase(std::remove_if(t.effects.begin(), t.effects.end(), [](const Effect& e) { auto* c = std::get_if<CompanionOwner>(&e); return c && !c->active; }), t.effects.end());
        return before - t.effects.size();
    };
    if (!r.host.commit(1, cause(CauseKind::Script), {CompanionOwner{Q(200), 1, true, 0}})) return fail("first recruit refused");
    if (!r.host.commit(2, cause(CauseKind::Script), {CompanionOwner{Q(200), 1, false, 0}, CompanionOwner{Q(201), 2, true, 0}})) return fail("the swap txn vanished whole");
    if (!r.host.state().companionOwner.count(Q(200)) || r.host.state().companionOwner.at(Q(200)) != 1) return fail("the swap dismissed the other player's companion");
    if (!r.host.state().companionOwner.count(Q(201))) return fail("the recruit did not go through");
    if (r.host.stats().companionVetDrops != 1) return fail("the removed effect was not counted");
    r.run(600);
    for (auto& c : r.cl) if (c->cs->state() != r.host.state()) return fail("a machine applied what the vet removed");
    return true;
}

bool controlLockReleased(std::string* why, bool hostNotified) {
    auto fail = [&](const char* w) { if (why) *why = w; return false; };
    Rig r(6);
    r.addClient(1); r.addClient(2);
    r.joinAll(); r.run(1500);
    r.host.commit(0, cause(CauseKind::QuestResult), {SetPlayerControls{0x3F, false, Recipient::all()}});   // cinematic: everybody locked
    r.run(500);
    if (r.host.state().ledger[1].controlsOff != 0x3F) return fail("lock not taken");
    if (hostNotified) r.drop(0);                                                                           // transport reports the loss
    else { r.story.setConnected(r.cl[0]->peer, false); r.cl[0]->cs->disconnect(); }                        // only the client noticed
    r.run(300);
    if (r.cl[0]->sink.seen.empty() || std::get_if<SetPlayerControls>(&r.cl[0]->sink.seen.back().e) == nullptr ||
        !std::get<SetPlayerControls>(r.cl[0]->sink.seen.back().e).enable) return fail("the disconnecting client was not unlocked");
    if (hostNotified && r.host.state().ledger[1].controlsOff != 0) return fail("the host kept the lock of a player who left");
    if (r.host.state().ledger[2].controlsOff != 0x3F) return fail("the party lock of the others was released");
    r.reconnect(0);
    r.run(3000);
    if (!r.allLive()) return fail("not live after rejoin");
    if (r.host.state().ledger[1].controlsOff != 0 || r.cl[0]->cs->state().ledger[1].controlsOff != 0) return fail("returned frozen");
    if (r.cl[0]->cs->state() != r.host.state()) return fail("diverged");
    return true;
}

bool createdRefsSession(std::string* why) {
    auto fail = [&](const char* w) { if (why) *why = w; return false; };
    Rig r(7);
    r.addClient(1); r.joinAll(); r.run(1500);
    FormRef a = r.host.allocCreatedRef(), b = r.host.allocCreatedRef();
    if (!isCreatedRef(a) || a == b || (a.localId >> 24) != 0xFF) return fail("ids");
    r.host.commit(0, cause(CauseKind::Script), {SpawnRef{a.localId, Q(1000), Q(2000), 1, 2, 3, true}, SpawnRef{b.localId, Q(1001), Q(2000), 0, 0, 0, false}});
    r.run(300);
    r.host.commit(0, cause(CauseKind::Script), {DespawnRef{a.localId}});
    r.run(300);
    if (r.cl[0]->sink.countSeen(33) != 2 || r.cl[0]->sink.countSeen(34) != 1) return fail("spawn / despawn not delivered");
    Bytes side = r.host.saveSidecar();
    HostSession h2(*r.story.endpoint(77));
    if (!h2.loadSidecar(side)) return fail("sidecar");
    FormRef c = h2.allocCreatedRef();
    if (c == a || c == b || c.localId <= b.localId) return fail("a created ref id was reused after load");
    // a late joiner: replay or snapshot, never a resurrected despawned ref
    r.addClient(2); r.joinOnly(1); r.run(3000);
    if (r.cl[1]->cs->state() != r.host.state()) return fail("late joiner diverged");
    if (r.cl[1]->cs->state().spawned.count(a) || !r.cl[1]->cs->state().spawned.count(b)) return fail("late joiner spawn state");
    return true;
}

bool scriptedSaveEvent(std::string* why) {
    auto fail = [&](const char* w) { if (why) *why = w; return false; };
    Rig r(8);
    r.addClient(1); r.joinAll(); r.run(1500);
    r.host.commit(0, cause(CauseKind::Script), {RequestSave{kSaveAuto}});
    r.run(300);
    if (r.cl[0]->sink.countSeen(45) != 1) return fail("the client was not asked to write its sidecar exactly once");
    if (r.host.state().createdCounter != 0) return fail("event left state");
    r.addClient(2); r.joinOnly(1); r.run(3000);
    if (r.cl[1]->sink.countSeen(45, true) != 0) return fail("a late joiner is told to save for an old event (not marked catch-up)");
    return true;
}

const char* kLocalRules = R"({"schema":1,"quests":{
 "GLQ":{"tier":"T1","questline":"x","curated":true,"rules":[{"kind":"GuestLocalRun","params":{"scripts":["AchievementScript","RumbleScript"]},"origin":"curated"}]},
 "Other":{"tier":"T1","questline":"x","curated":true,"rules":[{"kind":"RewardAll","params":{},"origin":"curated"}]}}})";

bool guestLocalHolds(std::string* why) {
    auto fail = [&](const char* w) { if (why) *why = w; return false; };
    RulesDb db;
    if (!db.load(kLocalRules)) return fail("rules");
    Rig r(9);
    r.addClient(1); r.joinAll(); r.run(1500);
    RulesEngine eng(r.host, db, resolveEdid);
    if (!eng.guestLocalRun(QF("GLQ")) || eng.guestLocalRun(QF("Other"))) return fail("guestLocalRun lookup");
    if (eng.guestLocalQuests().size() != 1 || eng.guestLocalScripts().size() != 2) return fail("allow-list export");
    if (!eng.unsupportedKinds().empty()) return fail("GuestLocalRun reported unsupported");
    Cause c = cause(CauseKind::QuestResult);
    c.source = QF("GLQ");
    if (r.host.commit(0, c, {SetStage{Q(1), 5}, SetGlobal{Q(2), 1.f}})) return fail("a per-machine script was replicated");
    c.source = QF("Other");
    if (!r.host.commit(0, c, {SetStage{Q(1), 5}})) return fail("an ordinary quest was dropped");
    return eng.stats().guestLocalDropped == 1;
}
}  // namespace

TEST(round3, routing_active_quest_perks_faction_and_journal) {
    std::string why; CHECK_MSG(routingHolds(&why), why);
}
TEST(round3, follower_caps_per_owner_kind_and_party_total) {
    std::string why; CHECK_MSG(followerCapsHold(&why), why);
}
TEST(round3, the_companion_vet_runs_before_the_commit_and_its_removals_never_reach_a_machine) {
    std::string why; CHECK_MSG(companionVetHolds(&why), why);
}
TEST(round3, disconnect_always_releases_the_control_lock) {
    std::string why;
    CHECK_MSG(controlLockReleased(&why, true), "transport-notified: " + why);
    why.clear();
    CHECK_MSG(controlLockReleased(&why, false), "client-only: " + why);
}
TEST(round3, created_refs_are_allocated_by_the_host_and_survive_save_load_and_late_join) {
    std::string why; CHECK_MSG(createdRefsSession(&why), why);
}
TEST(round3, scripted_save_is_an_event_not_state) {
    std::string why; CHECK_MSG(scriptedSaveEvent(&why), why);
}
TEST(round3, guest_local_run_quests_are_not_replicated) {
    std::string why; CHECK_MSG(guestLocalHolds(&why), why);
}

TEST(twin, round3_active_quest_shared) {
    MutationScope m([] { Mutations x; x.activeQuestShared = true; return x; }());
    std::string why; CHECK(!routingHolds(&why));
}
TEST(twin, round3_ignore_follower_caps) {
    MutationScope m([] { Mutations x; x.ignoreFollowerCaps = true; return x; }());
    std::string why; CHECK(!followerCapsHold(&why));
}
TEST(twin, round3_companion_vet_ignored) {
    MutationScope m([] { Mutations x; x.companionVetIgnored = true; return x; }());
    std::string why; CHECK(!companionVetHolds(&why));
}
TEST(twin, round3_disconnect_keeps_lock) {
    MutationScope m([] { Mutations x; x.disconnectKeepsLock = true; return x; }());
    std::string why; CHECK(!controlLockReleased(&why, true));
}
TEST(twin, round3_guest_local_replicated) {
    MutationScope m([] { Mutations x; x.guestLocalReplicated = true; return x; }());
    std::string why; CHECK(!guestLocalHolds(&why));
}
