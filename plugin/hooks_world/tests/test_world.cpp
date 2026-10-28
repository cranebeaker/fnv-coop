// L12 world-state unit tests. Built once (FNVCOOP_RUNTIME_MUT); run once for real and once per mutation twin with FNVCOOP_MUT=<n> (each twin MUST fail).
#include <cstdio>
#include <cstring>
#include <map>
#include <set>
#include <string>
#include <vector>

#include "effect_json.h"
#include "hook_patch.h"
#include "world_addrs.h"
#include "world_owner_addrs.h"
#include "apply_lines.h"
#include "world_core.h"
#include "world_plan.h"
#include "world_query.h"
#include "world_refvars.h"
#include "engine_addrs.h"

using namespace nvworld;
using fnvcoop::FormRef;

static int g_fail = 0, g_checks = 0;
#define CHECK(c)                                                       \
    do {                                                               \
        ++g_checks;                                                    \
        if (!(c)) {                                                    \
            ++g_fail;                                                  \
            std::printf("  FAIL %s:%d  %s\n", __FILE__, __LINE__, #c); \
        }                                                              \
    } while (0)
#define TEST(name) static void name()
static void run(const char* n, void (*f)()) {
    int before = g_fail;
    f();
    std::printf("[%s] %s\n", g_fail == before ? " ok " : "FAIL", n);
}

static FormRef esm(uint32_t id) { return FormRef("falloutnv.esm", id); }
static FormRef created(uint32_t id) { return FormRef(fnvcoop::createdRefPlugin(), 0xFF000000u | id); }

// ---------------------------------------------------------------------------------------------- fake engine resolver
struct FakeResolver : nvhooks::FormResolver {
    std::set<uint32_t> unloaded;  // forms that exist in the esm but are not loaded right now
    bool toRef(uint32_t id, FormRef* out) override {
        if ((id >> 24) == 0xFF || (id >> 24) != 0) return false;  // engine resolver cannot name created refs
        *out = esm(id & 0xFFFFFF);
        return true;
    }
    uint32_t fromRef(const FormRef& r) override { return r.plugin == "falloutnv.esm" && !unloaded.count(r.localId) ? r.localId : 0; }
    void* lookup(uint32_t id) override { return id ? reinterpret_cast<void*>(uintptr_t(id)) : nullptr; }
};

TEST(created_ref_map) {
    CreatedRefMap m;
    CHECK(m.local(created(1)) == 0);
    m.bind(created(1), 0x1000);
    CHECK(m.local(created(1)) == 0x1000);
    FormRef h;
    CHECK(m.hostOf(0x1000, &h) && h == created(1));
    m.bind(created(1), 0x2000);  // the host ref now maps to a different local object
    CHECK(m.local(created(1)) == 0x2000);
    CHECK(!m.hostOf(0x1000, &h));
    CHECK(m.hostOf(0x2000, &h));
    CHECK(m.size() == 1);
    CHECK(m.forget(created(1)));
    CHECK(!m.hostOf(0x2000, &h));
    CHECK(m.local(created(1)) == 0);
    CHECK(!m.forget(created(1)));
}

TEST(world_resolver_host) {
    FakeResolver inner;
    CreatedRefMap map;
    WorldResolver r(inner, map, true);
    FormRef f;
    CHECK(r.toRef(0xFF001234u, &f) && f == FormRef(fnvcoop::createdRefPlugin(), 0xFF001234u));
    CHECK(r.fromRef(f) == 0xFF001234u);
    CHECK(r.toRef(0x00104C79u, &f) && f == esm(0x104C79));
    CHECK(r.fromRef(esm(0x104C79)) == 0x104C79u);
}

TEST(world_resolver_client) {
    FakeResolver inner;
    CreatedRefMap map;
    WorldResolver r(inner, map, false);
    FormRef f;
    CHECK(r.fromRef(created(7)) == 0);  // not created here yet
    CHECK(!r.toRef(0xFF000055u, &f));
    map.bind(created(7), 0xFF000055u);
    CHECK(r.fromRef(created(7)) == 0xFF000055u);
    CHECK(r.toRef(0xFF000055u, &f) && f == created(7));  // local id back to the HOST's name
    CHECK(r.toRef(0x00104C79u, &f) && f == esm(0x104C79));
}

static fnvcoop::SpawnRef spawn(uint32_t id, uint32_t cell = 0x10) {
    fnvcoop::SpawnRef s;
    s.id = 0xFF000000u | id;
    s.base = esm(0x500 + id);
    s.cell = esm(cell);
    s.x = 1;
    s.y = 2;
    s.z = 3;
    return s;
}

TEST(spawn_ledger_create_and_despawn) {
    SpawnLedger l;
    CHECK(l.onSpawn(spawn(1), true) == SpawnAction::Create);
    l.onCreated(created(1), 0xFF000A00u);
    CHECK(l.onSpawn(spawn(1), true) == SpawnAction::Duplicate);  // replayed txn: never create twice
    auto d = l.onDespawn(fnvcoop::DespawnRef{0xFF000001u});
    CHECK(d.action == DespawnAction::DeleteLocal && d.localRefID == 0xFF000A00u);
    CHECK(l.map().size() == 0);
    CHECK(l.onDespawn(fnvcoop::DespawnRef{0xFF000001u}).action == DespawnAction::Unknown);  // second despawn is a no-op
}

TEST(spawn_ledger_queue) {
    SpawnLedger l;
    CHECK(l.onSpawn(spawn(1, 0x10), false) == SpawnAction::Queued);
    CHECK(l.onSpawn(spawn(2, 0x20), false) == SpawnAction::Queued);
    CHECK(l.onSpawn(spawn(1, 0x10), false) == SpawnAction::Duplicate);
    CHECK(l.pending() == 2);
    auto got = l.takeReady([](const fnvcoop::SpawnRef& s) { return s.cell == esm(0x10); });
    CHECK(got.size() == 1 && got[0].ref() == created(1));
    CHECK(l.pending() == 1);
    got = l.takeReady([](const fnvcoop::SpawnRef& s) { return s.cell == esm(0x10); });
    CHECK(got.empty());  // already taken: no second creation
    // despawn before the cell loaded: the queued spawn is cancelled, nothing is ever created
    CHECK(l.onDespawn(fnvcoop::DespawnRef{0xFF000002u}).action == DespawnAction::CancelledPending);
    CHECK(l.pending() == 0);
    got = l.takeReady([](const fnvcoop::SpawnRef&) { return true; });
    CHECK(got.empty());
}

TEST(spawn_ledger_clear) {
    SpawnLedger l;
    l.onSpawn(spawn(1), true);
    l.onCreated(created(1), 0xFF000A00u);
    l.onSpawn(spawn(2, 0x99), false);
    CHECK(l.map().size() == 1 && l.pending() == 1);
    l.clear();  // new world: nothing of the old one survives
    CHECK(l.map().size() == 0 && l.pending() == 0);
    CHECK(l.onSpawn(spawn(1), true) == SpawnAction::Create);  // and the same host ref can be created again
}

TEST(ref_var_tracker) {
    RefVarTracker t;
    t.beginPoll(100);
    CHECK(!t.observe(0x100, 3, 0.0));  // baseline is silent
    CHECK(!t.observe(0x100, 3, 0.0));  // unchanged
    CHECK(t.observe(0x100, 3, 1.0));   // change
    CHECK(!t.observe(0x100, 3, 1.0));  // not repeated
    CHECK(t.observe(0x100, 3, 0.0));   // changes back
    CHECK(t.reportedThisPoll() == 2);
    CHECK(t.size() == 1);
}

TEST(ref_var_budget) {
    RefVarTracker t;
    t.beginPoll(2);
    for (uint16_t i = 0; i < 5; ++i) t.observe(0x200, i, 0);
    t.beginPoll(2);
    int sent = 0;
    for (uint16_t i = 0; i < 5; ++i) sent += t.observe(0x200, i, 1.0) ? 1 : 0;
    CHECK(sent == 2);                     // capped
    t.beginPoll(2);
    for (uint16_t i = 0; i < 5; ++i) sent += t.observe(0x200, i, 1.0) ? 1 : 0;
    CHECK(sent == 4);                     // the deferred ones are reported by the next poll, never lost
    t.beginPoll(2);
    for (uint16_t i = 0; i < 5; ++i) sent += t.observe(0x200, i, 1.0) ? 1 : 0;
    CHECK(sent == 5);
    t.beginPoll(2);
    for (uint16_t i = 0; i < 5; ++i) sent += t.observe(0x200, i, 1.0) ? 1 : 0;
    CHECK(sent == 5);
}

TEST(ref_var_forget) {
    RefVarTracker t;
    t.beginPoll(10);
    t.observe(0x300, 1, 5);
    t.observe(0x301, 1, 5);
    t.forgetRef(0x300);
    CHECK(t.size() == 1);
    CHECK(!t.observe(0x300, 1, 9));  // re-baselined after unload, the 9 is not reported
    CHECK(t.observe(0x300, 1, 10));
}

TEST(weather_time_sync) {
    WeatherTimeSync s;
    auto w1 = esm(0x10), w2 = esm(0x20);
    auto e = s.poll(w1, true, 1.0f);
    CHECK(e.size() == 2);  // first sight of both
    CHECK(std::get<fnvcoop::SetWeather>(e[0]).weather == w1);
    CHECK(std::get<fnvcoop::SetTimeScale>(e[1]).scale == 1.0f);
    CHECK(s.poll(w1, true, 1.0f).empty());  // unchanged: nothing
    e = s.poll(w2, true, 1.0f);
    CHECK(e.size() == 1 && std::get<fnvcoop::SetWeather>(e[0]).weather == w2);
    CHECK(s.poll(w2, false, 1.0f).empty());  // interior: no weather
    e = s.poll(w2, true, 30.0f);
    CHECK(e.size() == 1 && std::get<fnvcoop::SetTimeScale>(e[0]).scale == 30.0f);
    // a script already sent SetWeather(w1) through the capture hub: the next poll must not repeat it
    s.noteSent(fnvcoop::SetWeather{w1, true});
    CHECK(s.poll(w1, true, 30.0f).empty());
    s.noteSent(fnvcoop::SetTimeScale{5.0f});
    CHECK(s.poll(w1, true, 5.0f).empty());
}

TEST(weather_block_local) {
    CHECK(WeatherTimeSync::blockLocal(true, false));
    CHECK(!WeatherTimeSync::blockLocal(true, true));    // a host txn being applied may change it
    CHECK(!WeatherTimeSync::blockLocal(false, false));  // the host plays normally
}

TEST(av_effects) {
    auto a = esm(0x104C79);
    CHECK(!makeAvEffect(a, 16, AvCmdKind::Mod, 100, 100));  // nothing changed
    auto e = makeAvEffect(a, 16, AvCmdKind::Set, 100, 250);
    CHECK(e && e->mode == AvMode::Set && e->value == 250 && e->av == 16);
    e = makeAvEffect(a, 16, AvCmdKind::Force, 100, 250);
    CHECK(e && e->mode == AvMode::Set && e->value == 250);
    e = makeAvEffect(a, 16, AvCmdKind::Mod, 100, 125);
    CHECK(e && e->mode == AvMode::Mod && e->value == 25);
    e = makeAvEffect(a, 16, AvCmdKind::Damage, 100, 70);
    CHECK(e && e->mode == AvMode::Damage && e->value == 30);  // positive damage amount
    e = makeAvEffect(a, 16, AvCmdKind::Restore, 40, 100);
    CHECK(e && e->mode == AvMode::Restore && e->value == 60);
}

TEST(av_names_and_lines) {
    CHECK(std::strcmp(avName(16), "Health") == 0);
    CHECK(std::strcmp(avName(12), "ActionPoints") == 0);
    CHECK(std::strcmp(avName(0), "Aggression") == 0);
    CHECK(std::strcmp(avName(1), "Confidence") == 0);
    CHECK(std::strcmp(avName(5), "Strength") == 0);
    CHECK(std::strcmp(avName(41), "Guns") == 0);
    CHECK(std::strcmp(avName(45), "Unarmed") == 0);
    CHECK(std::strcmp(avName(999), "") == 0);
    SetActorValueRef x{esm(1), 16, 250, AvMode::Set};
    CHECK(avLine(x) == "SetActorValue Health 250");
    x.mode = AvMode::Mod; x.value = -5;
    CHECK(avLine(x) == "ModActorValue Health -5");
    x.mode = AvMode::Damage; x.value = 30;
    CHECK(avLine(x) == "DamageActorValue Health 30");
    x.mode = AvMode::Restore; x.value = 60;
    CHECK(avLine(x) == "RestoreActorValue Health 60");
    x.av = 999;
    CHECK(avLine(x).empty());
}

TEST(console_lines) {
    CHECK(markForDeleteLine() == "MarkForDelete");
    SetEssential e{esm(0x104C79), true, false};
    CHECK(setEssentialLine(e, 0x104C79) == "SetEssential 00104C79 1");
    e.essential = false;
    CHECK(setEssentialLine(e, 0x104C79) == "SetEssential 00104C79 0");
    e.refOnly = true; e.essential = true;
    CHECK(setEssentialLine(e, 0x104C79) == "SetActorRefEssential 1");
    CHECK(setRestrainedLine(true) == "SetRestrained 1");
    CHECK(setRestrainedLine(false) == "SetRestrained 0");
    CHECK(setRelationLine(Relation::Enemy, 0x11, 0x22) == "SetEnemy 00000011 00000022");
    CHECK(setRelationLine(Relation::Ally, 0x11, 0x22) == "SetAlly 00000011 00000022");
    CHECK(scriptPackageLine(true, 0xAB) == "AddScriptPackage 000000AB");
    CHECK(scriptPackageLine(false, 0xAB) == "RemoveScriptPackage 000000AB");
    CHECK(placeAtMeLine(0x1234, 3) == "PlaceAtMe 00001234 3");
    CHECK(setPosLine('X', 1.5f) == "SetPos X 1.5");
}

TEST(world_log_coalesce) {
    WorldLog l;
    l.push(SetEssential{esm(1), true, false});
    l.push(SetEssential{esm(1), false, false});  // last write wins
    l.push(SetEssential{esm(2), true, false});
    l.push(SetActorValueRef{esm(1), 16, 50, AvMode::Set});
    l.push(SetActorValueRef{esm(1), 16, 75, AvMode::Set});
    l.push(SetActorValueRef{esm(1), 16, 5, AvMode::Damage});  // deltas are ordered and never coalesced
    l.push(SetActorValueRef{esm(1), 16, 5, AvMode::Damage});
    l.push(SetRestrained{esm(1), true});
    l.push(SetRestrained{esm(1), false});
    CHECK(l.size() == 6);
    auto v = l.drain();
    CHECK(v.size() == 6 && l.size() == 0);
    if (v.size() != 6) return;
    CHECK(std::get<SetEssential>(v[0]).essential == false);
    CHECK(std::get<SetActorValueRef>(v[2]).value == 75);
    CHECK(std::get<SetRestrained>(v[5]).on == false);
}

TEST(world_effect_json) {
    std::vector<WorldEffect> all = {
        SetActorValueRef{esm(0x104C79), 16, 12.5f, AvMode::Damage},
        SetEssential{esm(0x104C79), true, true},
        SetRestrained{esm(0x104C79), true},
        SetRelation{esm(0x11), esm(0x22), Relation::Ally},
        SetScriptPackage{esm(0x1), esm(0x2), false},
    };
    for (auto& e : all) {
        WorldEffect back;
        std::string err;
        std::string js = worldEffectToJson(e);
        CHECK(worldEffectFromJson(js, &back, &err));
        CHECK(back == e);
    }
    WorldEffect x;
    std::string err;
    CHECK(!worldEffectFromJson("{\"type\":\"Nope\"}", &x, &err));
    CHECK(!worldEffectFromJson("not json", &x, &err));
}


// ---------------------------------------------------------------------------------------------- apply planning
struct PlanFx {
    FakeResolver inner;
    CreatedRefMap* mapPtr = nullptr;
    SpawnLedger ledger;
    std::set<uint32_t> loadedCells;
    PlanEnv env;
    WorldResolver res;
    PlanFx() : res(inner, ledger.map(), false), env{res, ledger, nullptr} {
        env.cellReady = [this](const FormRef& c) { return loadedCells.count(c.localId) > 0; };
    }
};

TEST(plan_spawn) {
    PlanFx f;
    f.loadedCells.insert(0x10);
    auto p = planEffect(spawn(1, 0x10), f.env);
    CHECK(p.status == PlanStatus::Ok && p.steps.size() == 1);
    CHECK(p.steps[0].kind == StepKind::CreateSpawn && p.steps[0].baseID == 0x501 && p.steps[0].cellID == 0x10);
    f.ledger.onCreated(created(1), 0xFF000B00u);
    p = planEffect(spawn(1, 0x10), f.env);  // replayed
    CHECK(p.status == PlanStatus::Ok && p.steps.empty());
    // cell not loaded -> queued, nothing to run yet
    p = planEffect(spawn(2, 0x20), f.env);
    CHECK(p.status == PlanStatus::Queued && p.steps.empty());
    CHECK(planQueuedSpawns(f.env).empty());
    f.loadedCells.insert(0x20);
    auto q = planQueuedSpawns(f.env);
    CHECK(q.size() == 1 && q[0].steps.size() == 1 && q[0].steps[0].spawn.ref() == created(2));
    CHECK(planQueuedSpawns(f.env).empty());
}

TEST(plan_spawn_missing_base) {
    PlanFx f;
    f.loadedCells.insert(0x10);
    fnvcoop::SpawnRef s = spawn(3, 0x10);
    s.base = FormRef("nosuchmod.esp", 5);
    CHECK(planEffect(s, f.env).status == PlanStatus::FormMissing);
}

TEST(plan_despawn) {
    PlanFx f;
    f.loadedCells.insert(0x10);
    planEffect(spawn(1, 0x10), f.env);
    f.ledger.onCreated(created(1), 0xFF000C00u);
    auto p = planEffect(fnvcoop::DespawnRef{0xFF000001u}, f.env);
    CHECK(p.status == PlanStatus::Ok && p.steps.size() == 1);
    CHECK(p.steps[0].kind == StepKind::ConsoleOnRef && p.steps[0].refID == 0xFF000C00u && p.steps[0].line == "MarkForDelete");
    p = planEffect(fnvcoop::DespawnRef{0xFF000001u}, f.env);  // already gone
    CHECK(p.status == PlanStatus::Ok && p.steps.empty());
    planEffect(spawn(9, 0x99), f.env);  // queued
    p = planEffect(fnvcoop::DespawnRef{0xFF000009u}, f.env);
    CHECK(p.status == PlanStatus::Ok && p.steps.empty() && f.ledger.pending() == 0);
}

TEST(plan_ref_script_var) {
    PlanFx f;
    auto p = planEffect(fnvcoop::RefScriptVar{esm(0x777), 4, 2.5}, f.env);
    CHECK(p.status == PlanStatus::Ok && p.steps.size() == 1);
    CHECK(p.steps[0].kind == StepKind::SetRefVar && p.steps[0].refID == 0x777 && p.steps[0].varIndex == 4 && p.steps[0].varValue == 2.5);
    p = planEffect(fnvcoop::RefScriptVar{FormRef("nosuchmod.esp", 1), 4, 2.5}, f.env);
    CHECK(p.status == PlanStatus::FormMissing);
    // a var on a created ref goes through the host->local map
    f.ledger.onCreated(created(5), 0xFF000D00u);
    p = planEffect(fnvcoop::RefScriptVar{created(5), 1, 9}, f.env);
    CHECK(p.status == PlanStatus::Ok && p.steps.size() == 1 && p.steps[0].refID == 0xFF000D00u);
    CHECK(planEffect(fnvcoop::SetStage{esm(1), 10}, f.env).status == PlanStatus::Unsupported);
    CHECK(handlesEffect(fnvcoop::SpawnRef{}) && handlesEffect(fnvcoop::DespawnRef{}) && handlesEffect(fnvcoop::RefScriptVar{}));
    CHECK(!handlesEffect(fnvcoop::SetStage{}));
}

TEST(plan_world_effects) {
    PlanFx f;
    auto p = planWorldEffect(SetActorValueRef{esm(0x100), 16, 50, AvMode::Damage}, f.env);
    CHECK(p.status == PlanStatus::Ok && p.steps.size() == 1 && p.steps[0].refID == 0x100 && p.steps[0].line == "DamageActorValue Health 50");
    p = planWorldEffect(SetActorValueRef{esm(0x100), 9999, 1, AvMode::Set}, f.env);
    CHECK(p.status == PlanStatus::Unsupported);
    p = planWorldEffect(SetEssential{esm(0x200), true, true}, f.env);  // ref-only: on the ref
    CHECK(p.steps.size() == 1 && p.steps[0].kind == StepKind::ConsoleOnRef && p.steps[0].refID == 0x200 && p.steps[0].line == "SetActorRefEssential 1");
    p = planWorldEffect(SetEssential{esm(0x200), false, false}, f.env);  // base form: global command
    CHECK(p.steps.size() == 1 && p.steps[0].kind == StepKind::ConsoleGlobal && p.steps[0].line == "SetEssential 00000200 0");
    p = planWorldEffect(SetRestrained{esm(0x300), true}, f.env);
    CHECK(p.steps.size() == 1 && p.steps[0].refID == 0x300 && p.steps[0].line == "SetRestrained 1");
    p = planWorldEffect(SetRelation{esm(0x11), esm(0x22), Relation::Enemy}, f.env);
    CHECK(p.steps.size() == 1 && p.steps[0].kind == StepKind::ConsoleGlobal && p.steps[0].line == "SetEnemy 00000011 00000022");
    p = planWorldEffect(SetRelation{esm(0x11), FormRef("nosuchmod.esp", 2), Relation::Enemy}, f.env);
    CHECK(p.status == PlanStatus::FormMissing);
    p = planWorldEffect(SetScriptPackage{esm(0x300), esm(0x400), true}, f.env);
    CHECK(p.steps.size() == 1 && p.steps[0].refID == 0x300 && p.steps[0].line == "AddScriptPackage 00000400");
}

// ---------------------------------------------------------------------------------------------- IgnoreCrime / faction player-enemy
TEST(new_world_effects_lines_json_log) {
    CHECK(ignoreCrimeLine(true) == "IgnoreCrime 1");
    CHECK(ignoreCrimeLine(false) == "IgnoreCrime 0");
    CHECK(setPCEnemyOfFactionLine(0x1234, true) == "SetPCEnemyOfFaction 00001234 1");
    CHECK(setPCEnemyOfFactionLine(0x1234, false) == "SetPCEnemyOfFaction 00001234 0");
    for (WorldEffect e : {WorldEffect(SetIgnoreCrime{esm(0x104C79), true}), WorldEffect(SetFactionPlayerEnemy{esm(0x77), false})}) {
        WorldEffect back;
        std::string err;
        CHECK(worldEffectFromJson(worldEffectToJson(e), &back, &err));
        CHECK(back == e);
    }
    CHECK(std::string(worldEffectName(SetIgnoreCrime{})) == "SetIgnoreCrime");
    CHECK(std::string(worldEffectName(SetFactionPlayerEnemy{})) == "SetFactionPlayerEnemy");
    WorldLog l;
    l.push(SetIgnoreCrime{esm(1), true});
    l.push(SetIgnoreCrime{esm(1), false});
    l.push(SetIgnoreCrime{esm(2), true});
    l.push(SetFactionPlayerEnemy{esm(9), true});
    l.push(SetFactionPlayerEnemy{esm(9), false});
    auto v = l.drain();
    CHECK(v.size() == 3);
    if (v.size() == 3) {
        CHECK(std::get<SetIgnoreCrime>(v[0]).on == false);
        CHECK(std::get<SetFactionPlayerEnemy>(v[2]).on == false);
    }
}

TEST(plan_new_world_effects) {
    PlanFx f;
    auto p = planWorldEffect(SetIgnoreCrime{esm(0x300), true}, f.env);
    CHECK(p.status == PlanStatus::Ok && p.steps.size() == 1 && p.steps[0].kind == StepKind::ConsoleOnRef && p.steps[0].refID == 0x300 &&
          p.steps[0].line == "IgnoreCrime 1");
    p = planWorldEffect(SetFactionPlayerEnemy{esm(0x55), true}, f.env);
    CHECK(p.steps.size() == 1 && p.steps[0].kind == StepKind::ConsoleGlobal && p.steps[0].line == "SetPCEnemyOfFaction 00000055 1");
    p = planWorldEffect(SetFactionPlayerEnemy{esm(0x55), false}, f.env);
    CHECK(p.steps.size() == 1 && p.steps[0].line == "SetPCEnemyOfFaction 00000055 0");
    p = planWorldEffect(SetFactionPlayerEnemy{FormRef("nosuchmod.esp", 1), true}, f.env);
    CHECK(p.status == PlanStatus::FormMissing);
    p = planWorldEffect(SetIgnoreCrime{FormRef("nosuchmod.esp", 1), true}, f.env);
    CHECK(p.status == PlanStatus::FormMissing);
}

// ---------------------------------------------------------------------------------------------- world-time lane: ownership, ObjectState, clock, saves
TEST(ownership_lines_json_log) {
    CHECK(setOwnershipLine(0x55) == "SetOwnership 00000055");
    CHECK(setOwnershipLine(0) == "SetOwnership");   // no argument: the player owns it
    CHECK(clearOwnershipLine() == "ClearOwnership");
    CHECK(setCellOwnershipLine(0x10, 0x20) == "SetCellOwnership 00000010 00000020");
    CHECK(setCellPublicFlagLine(0x10, true) == "SetCellPublicFlag 00000010 1");
    CHECK(setCellPublicFlagLine(0x10, false) == "SetCellPublicFlag 00000010 0");
    SetOwner withOwner{esm(0x700), esm(0x55), false};
    SetOwner player{esm(0x700), FormRef(), false};
    SetOwner cleared{esm(0x700), FormRef(), true};
    for (WorldEffect e : {WorldEffect(withOwner), WorldEffect(player), WorldEffect(cleared), WorldEffect(SetCellOwner{esm(0x10), esm(0x20)}),
                          WorldEffect(SetCellPublic{esm(0x10), true})}) {
        WorldEffect back;
        std::string err;
        CHECK(worldEffectFromJson(worldEffectToJson(e), &back, &err));
        CHECK(back == e);
    }
    CHECK(std::string(worldEffectName(SetOwner{})) == "SetOwner");
    CHECK(std::string(worldEffectName(SetCellOwner{})) == "SetCellOwner");
    CHECK(std::string(worldEffectName(SetCellPublic{})) == "SetCellPublic");
    WorldLog l;
    l.push(withOwner);
    l.push(cleared);   // same ref: the later word replaces the earlier
    l.push(SetOwner{esm(0x701), esm(0x55), false});
    l.push(SetCellPublic{esm(0x10), true});
    l.push(SetCellPublic{esm(0x10), false});
    l.push(SetCellOwner{esm(0x10), esm(0x20)});
    l.push(SetCellOwner{esm(0x10), esm(0x21)});
    auto v = l.drain();
    CHECK(v.size() == 4);
    if (v.size() == 4) {
        CHECK(std::get<SetOwner>(v[0]).clear);
        CHECK(!std::get<SetCellPublic>(v[2]).on);
        CHECK(std::get<SetCellOwner>(v[3]).owner == esm(0x21));
    }
    FormRef t;
    CHECK(createdTargetOf(WorldEffect(SetOwner{created(4), esm(1), false}), &t) && t == created(4));
    WorldBaseline b;
    b.onWorld(withOwner);
    b.onWorld(cleared);
    b.onWorld(SetCellPublic{esm(0x10), true});
    auto snap = b.snapshot();
    CHECK(snap.world.size() == 2);
}

TEST(plan_ownership) {
    PlanFx f;
    auto p = planWorldEffect(SetOwner{esm(0x700), esm(0x55), false}, f.env);
    CHECK(p.status == PlanStatus::Ok && p.steps.size() == 1 && p.steps[0].kind == StepKind::ConsoleOnRef && p.steps[0].refID == 0x700 &&
          p.steps[0].line == "SetOwnership 00000055");
    p = planWorldEffect(SetOwner{esm(0x700), FormRef(), false}, f.env);
    CHECK(p.steps.size() == 1 && p.steps[0].line == "SetOwnership");
    p = planWorldEffect(SetOwner{esm(0x700), FormRef(), true}, f.env);
    CHECK(p.steps.size() == 1 && p.steps[0].line == "ClearOwnership" && p.steps[0].refID == 0x700);
    p = planWorldEffect(SetOwner{esm(0x700), FormRef("nosuchmod.esp", 1), false}, f.env);
    CHECK(p.status == PlanStatus::FormMissing);
    p = planWorldEffect(SetCellOwner{esm(0x10), esm(0x20)}, f.env);
    CHECK(p.steps.size() == 1 && p.steps[0].kind == StepKind::ConsoleGlobal && p.steps[0].line == "SetCellOwnership 00000010 00000020");
    p = planWorldEffect(SetCellPublic{esm(0x10), true}, f.env);
    CHECK(p.steps.size() == 1 && p.steps[0].kind == StepKind::ConsoleGlobal && p.steps[0].line == "SetCellPublicFlag 00000010 1");
    p = planWorldEffect(SetCellPublic{FormRef("nosuchmod.esp", 1), true}, f.env);
    CHECK(p.status == PlanStatus::FormMissing);
}

TEST(console_steps_are_scripts_and_run_outside_the_script_gate) {
    // lab run 2026-10-27: WorldApplier::applyAny held a ScriptGate around run(); the gate answers every Script::Execute with "pretend it ran" and a console line
    // is a script, so an ObjectState (the world-time poll's native door change, a client's heal) never changed anything on the client (status Ok, door unchanged)
    CHECK(stepRunsScripts(StepKind::ConsoleOnRef) && stepRunsScripts(StepKind::ConsoleGlobal));
    CHECK(!stepRunsScripts(StepKind::CreateSpawn) && !stepRunsScripts(StepKind::SetRefVar) && !stepRunsScripts(StepKind::AdvanceClock));
    PlanFx f;
    fnvcoop::ObjectState os;
    os.ref = esm(0x500);
    os.mask = fnvcoop::kObjOpen;
    os.open = true;
    auto p = planEffect(os, f.env);
    CHECK(p.steps.size() == 1 && stepRunsScripts(p.steps[0].kind));   // every step the ObjectState plan has is a console line: none of it may run gated
}

TEST(plan_object_state_clock_and_scripted_saves) {
    PlanFx f;
    CHECK(handlesEffect(fnvcoop::ObjectState{}) && handlesEffect(fnvcoop::AdvanceTime{}) && handlesEffect(fnvcoop::RequestSave{}));
    fnvcoop::ObjectState os;
    os.ref = esm(0x500);
    os.mask = fnvcoop::kObjLock | fnvcoop::kObjOpen | fnvcoop::kObjDestroyed;
    os.locked = true; os.level = 50; os.open = true; os.destroyed = true;
    auto p = planEffect(os, f.env);
    CHECK(p.status == PlanStatus::Ok);
    CHECK(p.steps.size() == 3);
    if (p.steps.size() == 3) {
        for (auto& s : p.steps) CHECK(s.kind == StepKind::ConsoleOnRef && s.refID == 0x500);
        CHECK(p.steps[0].line == nvhooks::lockCmd(50));   // the same console forms L5's RemoteApplier used
        CHECK(p.steps[1].line == nvhooks::setOpenStateCmd(true));
        CHECK(p.steps[2].line == nvhooks::setDestroyedCmd(true));
    }
    os.mask = fnvcoop::kObjLock;
    os.locked = false;
    p = planEffect(os, f.env);
    CHECK(p.steps.size() == 1 && p.steps[0].line == nvhooks::unlockCmd());
    os.mask = fnvcoop::kObjOpen;
    os.open = false;
    p = planEffect(os, f.env);
    CHECK(p.steps.size() == 1 && p.steps[0].line == nvhooks::setOpenStateCmd(false));
    os.mask = fnvcoop::kObjActivated;   // activate-once has no console form: nothing to run, still accepted
    p = planEffect(os, f.env);
    CHECK(p.status == PlanStatus::Ok && p.steps.empty());
    os.mask = fnvcoop::kObjOpen;
    os.ref = FormRef("nosuchmod.esp", 1);
    CHECK(planEffect(os, f.env).status == PlanStatus::FormMissing);

    p = planEffect(fnvcoop::AdvanceTime{90}, f.env);
    CHECK(p.status == PlanStatus::Ok && p.steps.size() == 1 && p.steps[0].kind == StepKind::AdvanceClock && p.steps[0].minutes == 90);

    p = planEffect(fnvcoop::RequestSave{0}, f.env);   // a client never saves because the host's script did
    CHECK(p.status == PlanStatus::Ok && p.steps.empty());
}

// ---------------------------------------------------------------------------------------------- created-ref targets, routing, deferred queue
TEST(created_targets) {
    FormRef t;
    CHECK(createdTargetOf(fnvcoop::RefScriptVar{created(3), 1, 2}, &t) && t == created(3));
    CHECK(createdTargetOf(fnvcoop::DisableRef{created(3)}, &t));
    CHECK(createdTargetOf(fnvcoop::KillActor{created(3)}, &t));
    CHECK(createdTargetOf(fnvcoop::MoveRef{created(3), esm(1), 0, 0, 0, false}, &t));
    CHECK(createdTargetOf(fnvcoop::SetScriptVar{created(3), 1, 2}, &t));
    CHECK(createdTargetOf(fnvcoop::SetFactionRank{created(3), esm(4), 1}, &t));
    CHECK(createdTargetOf(fnvcoop::RefAddItem{created(3), esm(4), 1}, &t));
    CHECK(!createdTargetOf(fnvcoop::RefScriptVar{esm(3), 1, 2}, &t));  // an esm ref is not a created target
    CHECK(!createdTargetOf(fnvcoop::SpawnRef{}, &t));                   // spawn / despawn are the lifecycle, not follow-ups
    CHECK(!createdTargetOf(fnvcoop::DespawnRef{}, &t));
    CHECK(!createdTargetOf(fnvcoop::SetStage{esm(1), 10}, &t));
    CHECK(createdTargetOf(WorldEffect(SetRestrained{created(3), true}), &t));
    CHECK(createdTargetOf(WorldEffect(SetIgnoreCrime{created(3), true}), &t));
    CHECK(createdTargetOf(WorldEffect(SetActorValueRef{created(3), 16, 1, AvMode::Set}), &t));
    CHECK(!createdTargetOf(WorldEffect(SetRelation{created(3), created(4), Relation::Ally}), &t));
}

TEST(route_and_retriable) {
    CreatedRefMap m;
    FormRef w;
    CHECK(routeEffect(fnvcoop::RefScriptVar{created(3), 1, 2}, m, &w) == Route::HoldForCreated && w == created(3));
    CHECK(routeEffect(fnvcoop::RefScriptVar{esm(3), 1, 2}, m, &w) == Route::Apply);
    CHECK(routeEffect(fnvcoop::SpawnRef{}, m, &w) == Route::Apply);
    CHECK(routeEffect(WorldEffect(SetRestrained{created(3), true}), m, &w) == Route::HoldForCreated);
    m.bind(created(3), 0xFF000900u);
    CHECK(routeEffect(fnvcoop::RefScriptVar{created(3), 1, 2}, m, &w) == Route::Apply);
    CHECK(retriable(fnvcoop::SetStage{}, Outcome::Missing));
    CHECK(!retriable(fnvcoop::SetStage{}, Outcome::Rejected));
    CHECK(retriable(fnvcoop::RefScriptVar{}, Outcome::Rejected));
    CHECK(!retriable(fnvcoop::RefScriptVar{}, Outcome::Other));
    CHECK(!retriable(fnvcoop::RefScriptVar{}, Outcome::Ok));
    CHECK(retriable(WorldEffect(SetRestrained{}), Outcome::Missing));
    CHECK(!retriable(WorldEffect(SetRestrained{}), Outcome::Rejected));
    // a door that refuses its lock change while the cell attaches is asked again (the lab door: the client stayed locked)
    CHECK(retriable(fnvcoop::ObjectState{}, Outcome::Rejected));    // twin 65
    CHECK(retriable(WorldEffect(SetObjectState{}), Outcome::Rejected));
    CHECK(!retriable(fnvcoop::ObjectState{}, Outcome::Other));
}

static HeldEffect heldCreated(uint32_t id, double v) {
    HeldEffect h;
    h.fx = fnvcoop::RefScriptVar{created(id), 0, v};
    h.waitOn = created(id);
    h.onCreated = true;
    return h;
}
static HeldEffect heldRetry(double v) {
    HeldEffect h;
    h.fx = fnvcoop::RefScriptVar{esm(0x10), 0, v};
    return h;
}

TEST(deferred_queue_basic) {
    DeferredQueue q;
    CreatedRefMap m;
    q.hold(heldCreated(1, 1));
    q.hold(heldCreated(2, 2));
    q.hold(heldCreated(1, 3));
    CHECK(q.size() == 3);
    CHECK(q.takeBound(m).empty());  // nothing bound yet: nothing released
    m.bind(created(1), 0xFF000A00u);
    auto r = q.takeBound(m);
    CHECK(r.size() == 2);  // arrival order, only ref 1
    if (r.size() == 2) CHECK(std::get<fnvcoop::RefScriptVar>(r[0].fx).value == 1 && std::get<fnvcoop::RefScriptVar>(r[1].fx).value == 3);
    CHECK(q.size() == 1);
    CHECK(q.takeBound(m).empty());  // released once
    CHECK(q.dropFor(created(2)) == 1 && q.size() == 0);
}

TEST(deferred_queue_retry_and_expiry) {
    DeferredQueue q;
    q.hold(heldRetry(1));
    q.hold(heldCreated(5, 2));
    for (int i = 0; i < 3; ++i) {
        auto r = q.takeRetries(3);
        CHECK(r.size() == 1 && r[0].attempts == i + 1);  // the retry waiter comes back every tick
        if (r.empty()) break;
        q.hold(std::move(r[0]));                          // the apply failed again: held with its attempts
    }
    CHECK(q.takeRetries(3).empty());  // attempts 4 > 3: expired, not returned again
    CHECK(q.expired() == 1);
    CHECK(q.size() == 1);  // the created-ref waiter ages 4x slower and is still waiting for its spawn
    for (int i = 0; i < 20; ++i) q.takeRetries(3);
    CHECK(q.size() == 0 && q.expired() == 2);
}

TEST(deferred_queue_cap) {
    DeferredQueue q(3);
    for (int i = 1; i <= 5; ++i) q.hold(heldCreated(1, i));
    CHECK(q.size() == 3 && q.dropped() == 2);
    CreatedRefMap m;
    m.bind(created(1), 1);
    auto r = q.takeBound(m);
    CHECK(r.size() == 3 && std::get<fnvcoop::RefScriptVar>(r[0].fx).value == 3);  // the OLDEST were dropped
}

// A fake executor: records plans, "creates" spawns with local ids 0xFF000A00+n, can fail var writes.
struct FakeRun {
    PlanFx& f;
    std::vector<Step> ran;
    uint32_t nextLocal = 0xFF000A00u;
    bool varsReady = true;
    Outcome operator()(const Plan& p) {
        for (const Step& s : p.steps) {
            if (s.kind == StepKind::CreateSpawn) f.ledger.onCreated(s.spawn.ref(), nextLocal++);
            if (s.kind == StepKind::SetRefVar && !varsReady) return Outcome::Rejected;
            ran.push_back(s);
        }
        return Outcome::Ok;
    }
};
static FallbackApply noFallback() { return [](const fnvcoop::Effect&) { return Outcome::Other; }; }

TEST(routed_follow_up_before_spawn) {
    PlanFx f;
    f.loadedCells.insert(0x10);
    FakeRun run{f};
    RunPlan rp = [&](const Plan& p) { return run(p); };
    // the follow-up overtakes the spawn: held, nothing runs
    CHECK(applyRouted(fnvcoop::RefScriptVar{created(1), 4, 7.5}, f.env, rp, noFallback()) == Outcome::Ok);
    CHECK(applyWorldRouted(SetRestrained{created(1), true}, f.env, rp) == Outcome::Ok);
    CHECK(run.ran.empty() && f.ledger.deferred().size() == 2);
    // the spawn arrives: created, then the held follow-ups run against the LOCAL ref, in order
    CHECK(applyRouted(spawn(1, 0x10), f.env, rp, noFallback()) == Outcome::Ok);
    CHECK(f.ledger.deferred().size() == 0);
    CHECK(run.ran.size() == 3);
    if (run.ran.size() == 3) {
        CHECK(run.ran[0].kind == StepKind::CreateSpawn);
        CHECK(run.ran[1].kind == StepKind::SetRefVar && run.ran[1].refID == 0xFF000A00u && run.ran[1].varIndex == 4 && run.ran[1].varValue == 7.5);
        CHECK(run.ran[2].kind == StepKind::ConsoleOnRef && run.ran[2].refID == 0xFF000A00u && run.ran[2].line == "SetRestrained 1");
    }
    // after that the target is bound: effects go straight through
    run.ran.clear();
    applyRouted(fnvcoop::RefScriptVar{created(1), 4, 8}, f.env, rp, noFallback());
    CHECK(run.ran.size() == 1 && run.ran[0].refID == 0xFF000A00u);
}

TEST(routed_follow_up_on_queued_spawn) {
    PlanFx f;  // cell 0x20 is not loaded: the spawn is queued, follow-ups wait for the creation
    FakeRun run{f};
    RunPlan rp = [&](const Plan& p) { return run(p); };
    applyRouted(spawn(2, 0x20), f.env, rp, noFallback());
    applyRouted(fnvcoop::RefScriptVar{created(2), 1, 3}, f.env, rp, noFallback());
    CHECK(run.ran.empty() && f.ledger.pending() == 1 && f.ledger.deferred().size() == 1);
    CHECK(flushDeferred(f.env, rp, noFallback(), false) == 0);  // spawn not created yet: still held
    f.loadedCells.insert(0x20);
    for (const Plan& p : planQueuedSpawns(f.env)) CHECK(run(p) == Outcome::Ok);
    CHECK(flushDeferred(f.env, rp, noFallback(), false) == 1);
    CHECK(run.ran.size() == 2 && run.ran[1].kind == StepKind::SetRefVar && run.ran[1].refID == 0xFF000A00u);
}

TEST(routed_despawn_drops_follow_ups) {
    PlanFx f;
    FakeRun run{f};
    RunPlan rp = [&](const Plan& p) { return run(p); };
    applyRouted(fnvcoop::RefScriptVar{created(3), 1, 3}, f.env, rp, noFallback());
    applyWorldRouted(SetIgnoreCrime{created(3), true}, f.env, rp);
    CHECK(f.ledger.deferred().size() == 2);
    applyRouted(fnvcoop::DespawnRef{0xFF000003u}, f.env, rp, noFallback());
    CHECK(f.ledger.deferred().size() == 0);
    // the same id arriving later (a new spawn) must not inherit the dead ref's follow-ups
    f.loadedCells.insert(0x10);
    applyRouted(spawn(3, 0x10), f.env, rp, noFallback());
    CHECK(run.ran.size() == 1 && run.ran[0].kind == StepKind::CreateSpawn);
}

TEST(routed_retry_unloaded_form) {
    PlanFx f;
    f.inner.unloaded.insert(0x500);
    FakeRun run{f};
    RunPlan rp = [&](const Plan& p) { return run(p); };
    CHECK(applyWorldRouted(SetRestrained{esm(0x500), true}, f.env, rp) == Outcome::Ok);  // not loaded: held, not lost
    CHECK(run.ran.empty() && f.ledger.deferred().size() == 1);
    CHECK(flushDeferred(f.env, rp, noFallback(), true) == 0);  // still not loaded: held again
    CHECK(f.ledger.deferred().size() == 1);
    f.inner.unloaded.clear();
    CHECK(flushDeferred(f.env, rp, noFallback(), true) == 1);
    CHECK(run.ran.size() == 1 && run.ran[0].refID == 0x500 && f.ledger.deferred().size() == 0);
    // a form that never shows up expires after maxAttempts
    f.inner.unloaded.insert(0x600);
    applyWorldRouted(SetRestrained{esm(0x600), true}, f.env, rp);
    for (int i = 0; i < 5; ++i) flushDeferred(f.env, rp, noFallback(), true, 3);
    CHECK(f.ledger.deferred().size() == 0 && f.ledger.deferred().expired() == 1);
}

TEST(routed_retry_ref_var_rejected_and_fallback) {
    PlanFx f;
    FakeRun run{f};
    RunPlan rp = [&](const Plan& p) { return run(p); };
    run.varsReady = false;  // the ref exists but its script instance has no variable list yet
    CHECK(applyRouted(fnvcoop::RefScriptVar{esm(0x700), 2, 1}, f.env, rp, noFallback()) == Outcome::Ok);
    CHECK(f.ledger.deferred().size() == 1);
    run.varsReady = true;
    CHECK(flushDeferred(f.env, rp, noFallback(), true) == 1);
    CHECK(run.ran.size() == 1 && run.ran[0].varIndex == 2 && f.ledger.deferred().size() == 0);
    // effects of other lanes go to the fallback; a rejection there is NOT retried (only RefScriptVar rejections are)
    int calls = 0;
    FallbackApply fb = [&](const fnvcoop::Effect&) { ++calls; return Outcome::Rejected; };
    CHECK(applyRouted(fnvcoop::SetStage{esm(1), 10}, f.env, rp, fb) == Outcome::Rejected && calls == 1);
    CHECK(f.ledger.deferred().size() == 0);
    // a fallback effect on a created ref is also held until the spawn exists
    fb = [&](const fnvcoop::Effect&) { ++calls; return Outcome::Ok; };
    applyRouted(fnvcoop::KillActor{created(9)}, f.env, rp, fb);
    CHECK(calls == 1 && f.ledger.deferred().size() == 1);
    f.loadedCells.insert(0x10);
    applyRouted(spawn(9, 0x10), f.env, rp, fb);
    CHECK(calls == 2 && f.ledger.deferred().size() == 0);
}

// ---------------------------------------------------------------------------------------------- rejoin
TEST(reconcile_and_plan) {
    PlanFx f;
    f.loadedCells.insert(0x10);
    f.ledger.onCreated(created(1), 0xFF000B01u);
    f.ledger.onCreated(created(2), 0xFF000B02u);
    f.ledger.onCreated(created(3), 0xFF000B03u);
    f.ledger.onSpawn(spawn(4, 0x99), false);  // queued
    f.ledger.onSpawn(spawn(5, 0x99), false);  // queued
    HeldEffect h = heldCreated(2, 1);
    f.ledger.deferred().hold(h);
    // the host still has 1, 5 and a new 6: 2 and 3 were despawned while this client was away; queued 4 is gone too
    auto p = planReconcile({spawn(1), spawn(5, 0x99), spawn(6)}, f.env);
    CHECK(p.steps.size() == 2);
    if (p.steps.size() == 2) {
        CHECK(p.steps[0].kind == StepKind::ConsoleOnRef && p.steps[0].line == "MarkForDelete");
        CHECK((p.steps[0].refID == 0xFF000B02u && p.steps[1].refID == 0xFF000B03u));
    }
    CHECK(f.ledger.map().local(created(1)) == 0xFF000B01u && f.ledger.map().local(created(2)) == 0);
    CHECK(f.ledger.pending() == 1);                 // only 5 remains queued
    CHECK(f.ledger.deferred().size() == 0);         // the follow-up of the vanished ref is gone with it
    CHECK(planReconcile({spawn(1), spawn(5, 0x99), spawn(6)}, f.env).steps.empty());  // idempotent
}

// ---------------------------------------------------------------------------------------------- host baseline
TEST(world_baseline_spawns_and_vars) {
    WorldBaseline b;
    b.onEffect(spawn(1));
    b.onEffect(spawn(2));
    b.onEffect(spawn(1));  // replayed txn
    b.onEffect(fnvcoop::RefScriptVar{created(1), 3, 1});
    b.onEffect(fnvcoop::RefScriptVar{created(1), 3, 2});  // last write wins
    b.onEffect(fnvcoop::RefScriptVar{esm(0x40), 0, 9});
    b.onEffect(fnvcoop::SetStage{esm(1), 10});            // not this lane's business
    auto s = b.snapshot();
    CHECK(b.spawns() == 2);
    CHECK(s.effects.size() == 4);  // 2 spawns first, then the 2 vars
    if (s.effects.size() == 4) {
        CHECK(std::holds_alternative<fnvcoop::SpawnRef>(s.effects[0]) && std::holds_alternative<fnvcoop::SpawnRef>(s.effects[1]));
        CHECK(std::get<fnvcoop::SpawnRef>(s.effects[0]).id == 0xFF000001u);
    }
    int vars = 0;
    for (auto& e : s.effects)
        if (auto* v = std::get_if<fnvcoop::RefScriptVar>(&e))
            if (v->ref == created(1)) { ++vars; CHECK(v->value == 2); }
    CHECK(vars == 1);
    CHECK(b.liveSpawnRefs().size() == 2);
    // despawn: the ref, its vars and its NPC state leave the baseline; other state stays
    b.onWorld(SetRestrained{created(1), true});
    b.onWorld(SetRestrained{esm(0x40), true});
    b.onEffect(fnvcoop::DespawnRef{0xFF000001u});
    s = b.snapshot();
    CHECK(b.spawns() == 1 && b.liveSpawnRefs()[0] == created(2));
    CHECK(s.effects.size() == 2);  // spawn 2 and the esm var
    CHECK(s.world.size() == 1 && std::get<SetRestrained>(s.world[0]).actor == esm(0x40));
    b.clear();
    CHECK(b.snapshot().effects.empty() && b.snapshot().world.empty());
}

TEST(world_baseline_folds_net_state) {
    WorldBaseline b;
    auto a = esm(0x100);
    b.onWorld(SetActorValueRef{a, 16, 10, AvMode::Damage});
    b.onWorld(SetActorValueRef{a, 16, 5, AvMode::Damage});
    b.onWorld(SetActorValueRef{a, 5, 2, AvMode::Mod});
    b.onWorld(SetActorValueRef{a, 5, 3, AvMode::Mod});
    auto s = b.snapshot();
    CHECK(s.world.size() == 2);
    if (s.world.size() == 2) {
        CHECK(std::get<SetActorValueRef>(s.world[0]).value == 15 && std::get<SetActorValueRef>(s.world[0]).mode == AvMode::Damage);  // deltas add up
        CHECK(std::get<SetActorValueRef>(s.world[1]).value == 5);
    }
    b.onWorld(SetActorValueRef{a, 16, 200, AvMode::Set});  // a Set makes everything before it moot
    s = b.snapshot();
    CHECK(s.world.size() == 2);
    b.onWorld(SetActorValueRef{a, 16, 30, AvMode::Damage});  // and later deltas stack on the Set
    s = b.snapshot();
    CHECK(s.world.size() == 3);
    if (s.world.size() == 3) {
        CHECK(std::get<SetActorValueRef>(s.world[1]).mode == AvMode::Set && std::get<SetActorValueRef>(s.world[1]).value == 200);
        CHECK(std::get<SetActorValueRef>(s.world[2]).mode == AvMode::Damage && std::get<SetActorValueRef>(s.world[2]).value == 30);
    }
    b.onWorld(SetActorValueRef{a, 7, 4, AvMode::Mod});
    b.onWorld(SetActorValueRef{a, 7, -4, AvMode::Mod});  // net zero disappears
    CHECK(b.worldEntries() == 3);
    // last write per key wins, in place
    b.onWorld(SetEssential{a, true, false});
    b.onWorld(SetEssential{a, false, false});
    b.onWorld(SetEssential{a, true, true});  // ref-only is a separate key
    b.onWorld(SetIgnoreCrime{a, true});
    b.onWorld(SetIgnoreCrime{a, false});
    b.onWorld(SetFactionPlayerEnemy{esm(9), true});
    b.onWorld(SetFactionPlayerEnemy{esm(9), false});
    b.onWorld(SetRelation{esm(1), esm(2), Relation::Enemy});
    b.onWorld(SetRelation{esm(1), esm(2), Relation::Ally});
    b.onWorld(SetScriptPackage{a, esm(0x20), true});
    b.onWorld(SetScriptPackage{a, esm(0x20), false});
    s = b.snapshot();
    CHECK(s.world.size() == 3 + 2 + 1 + 1 + 1 + 1);
    int ess = 0, ign = 0, rel = 0, pk = 0, pe = 0;
    for (auto& w : s.world) {
        if (auto* x = std::get_if<SetEssential>(&w)) { ++ess; if (!x->refOnly) CHECK(!x->essential); }
        if (auto* x = std::get_if<SetIgnoreCrime>(&w)) { ++ign; CHECK(!x->on); }
        if (auto* x = std::get_if<SetRelation>(&w)) { ++rel; CHECK(x->kind == Relation::Ally); }
        if (auto* x = std::get_if<SetScriptPackage>(&w)) { ++pk; CHECK(!x->add); }
        if (auto* x = std::get_if<SetFactionPlayerEnemy>(&w)) { ++pe; CHECK(!x->on); }
    }
    CHECK(ess == 2 && ign == 1 && rel == 1 && pk == 1 && pe == 1);
}

TEST(baseline_replay_on_client_is_idempotent) {
    // host side builds a baseline; a client applies it twice: spawns are created once, vars/state converge
    WorldBaseline b;
    b.onEffect(spawn(1));
    b.onEffect(fnvcoop::RefScriptVar{created(1), 0, 5});
    b.onWorld(SetRestrained{created(1), true});
    auto snap = b.snapshot();
    PlanFx f;
    f.loadedCells.insert(0x10);
    FakeRun run{f};
    RunPlan rp = [&](const Plan& p) { return run(p); };
    for (int round = 0; round < 2; ++round) {
        std::vector<fnvcoop::SpawnRef> live;
        for (auto& e : snap.effects)
            if (auto* sp = std::get_if<fnvcoop::SpawnRef>(&e)) live.push_back(*sp);
        planReconcile(live, f.env);
        for (auto& e : snap.effects) applyRouted(e, f.env, rp, noFallback());
        for (auto& w : snap.world) applyWorldRouted(w, f.env, rp);
    }
    int creates = 0;
    for (auto& s : run.ran) creates += s.kind == StepKind::CreateSpawn;
    CHECK(creates == 1);
    CHECK(f.ledger.map().size() == 1);
}

// ---------------------------------------------------------------------------------------------- engine hook specs
// The prologue of every NEW engine function this lane detours is pinned here. The patcher refuses to write unless all
// `expectedLen` bytes match (HookRegistry rule 1), so a drifted spec = the hook is skipped, never forced.
struct FakeMem : nvhooks::MemoryOps {
    std::map<nvhooks::Addr, std::vector<uint8_t>> img;
    std::map<nvhooks::Addr, std::vector<uint8_t>> heap;
    nvhooks::Addr next = 0x20000000;
    bool read(nvhooks::Addr a, void* out, size_t n) override {
        for (auto& [b, v] : img)
            if (a >= b && a + n <= b + v.size()) { std::memcpy(out, &v[a - b], n); return true; }
        for (auto& [b, v] : heap)
            if (a >= b && a + n <= b + v.size()) { std::memcpy(out, &v[a - b], n); return true; }
        return false;
    }
    bool write(nvhooks::Addr a, const void* in, size_t n) override {
        for (auto& [b, v] : img)
            if (a >= b && a + n <= b + v.size()) { std::memcpy(&v[a - b], in, n); return true; }
        for (auto& [b, v] : heap)
            if (a >= b && a + n <= b + v.size()) { std::memcpy(&v[a - b], in, n); return true; }
        return false;
    }
    nvhooks::Addr allocExec(size_t n) override {
        nvhooks::Addr a = next;
        next += (nvhooks::Addr)((n + 15) & ~size_t(15)) + 16;
        heap[a] = std::vector<uint8_t>(n, 0xCC);
        return a;
    }
    void freeExec(nvhooks::Addr a, size_t) override { heap.erase(a); }
};

TEST(world_specs_pinned) {
    CHECK(world_addrs::kCount > 0);
    for (size_t i = 0; i < world_addrs::kCount; ++i) {
        const nvhooks::HookSpec& s = *world_addrs::kAll[i];
        std::string why;
        CHECK(nvhooks::specIsValid(s, &why));
        CHECK(s.expected[0] == 0x55 && s.expected[1] == 0x8B && s.expected[2] == 0xEC);
        CHECK(s.retBytes == 0);  // every world hook is a cdecl script-command handler
        FakeMem mem;
        mem.img[s.target] = std::vector<uint8_t>(s.expected, s.expected + s.expectedLen);
        mem.img[s.target].resize(64, 0x90);
        nvhooks::HookRegistry reg(mem);
        nvhooks::Addr tramp = 0;
        CHECK(reg.acquire(s, 0x30000000, &tramp) == nvhooks::PatchStatus::Ok);
        CHECK(reg.release(s) == nvhooks::PatchStatus::Restored);
        // one byte of drift anywhere in the pinned window: refuse, write nothing
        for (int b = 0; b < s.expectedLen; ++b) {
            FakeMem m2;
            m2.img[s.target] = std::vector<uint8_t>(s.expected, s.expected + s.expectedLen);
            m2.img[s.target].resize(64, 0x90);
            m2.img[s.target][b] ^= 0x01;
            auto before = m2.img[s.target];
            nvhooks::HookRegistry r2(m2);
            CHECK(r2.acquire(s, 0x30000000, &tramp) != nvhooks::PatchStatus::Ok);
            CHECK(m2.img[s.target] == before);
        }
    }
}


// The generated header must equal the recorded live-process dump byte for byte (the dump is the evidence; see internal lane notes).
struct DumpRow {
    std::string name;
    unsigned addr = 0;
    int stolen = 0;
    std::vector<uint8_t> bytes;
};
static std::vector<DumpRow> readDump() {
    std::vector<DumpRow> rows;
    FILE* f = std::fopen(NVW_DUMP_PATH, "r");
    if (!f) return rows;
    char line[1024];
    while (std::fgets(line, sizeof line, f)) {
        char name[128], hex[64];
        unsigned addr = 0;
        int stolen = 0;
        if (std::sscanf(line, "%127s 0x%x stolen=%d bytes16=%32s", name, &addr, &stolen, hex) != 4) continue;
        DumpRow r;
        r.name = name;
        r.addr = addr;
        r.stolen = stolen;
        for (int i = 0; i < 16; ++i) {
            unsigned b = 0;
            std::sscanf(hex + 2 * i, "%2x", &b);
            r.bytes.push_back((uint8_t)b);
        }
        rows.push_back(r);
    }
    std::fclose(f);
    return rows;
}

TEST(world_specs_match_dump) {
    auto rows = readDump();
    CHECK(rows.size() >= 10);
    auto find = [&](const char* n) -> const DumpRow* {
        for (auto& r : rows)
            if (r.name == n) return &r;
        return nullptr;
    };
    for (size_t i = 0; i < world_addrs::kCount; ++i) {
        nvhooks::HookSpec s = *world_addrs::kAll[i];
        if (FNV_MUT(26)) {
            s.expected[3] ^= 0x01;  // MUTANT: a spec that drifted from the evidence
        }
        const DumpRow* r = find(s.name);
        CHECK(r != nullptr);
        if (!r) continue;
        CHECK(r->addr == s.target);
        CHECK(r->stolen == s.stolen);
        CHECK(std::memcmp(r->bytes.data(), s.expected, 16) == 0);
    }
    for (size_t i = 0; i < world_addrs::kCallCount; ++i) {
        const auto& c = *world_addrs::kCalls[i];
        const DumpRow* r = find(c.name);
        CHECK(r != nullptr);
        if (!r) continue;
        CHECK(r->addr == c.addr);
        CHECK(std::memcmp(r->bytes.data(), c.bytes, 16) == 0);
    }
    CHECK(world_addrs::kCount == 10);
    CHECK(world_addrs::kCallCount == 3);
}

// Command table entries of the hooked commands (cmdinfo_*.bin): the execute pointer must be the hooked address, and the parameter
// count must be what the detours' ExtractArgs calls assume.
static std::vector<uint8_t> readFile(const std::string& path) {
    std::vector<uint8_t> v;
    FILE* f = std::fopen(path.c_str(), "rb");
    if (!f) return v;
    uint8_t buf[512];
    size_t n;
    while ((n = std::fread(buf, 1, sizeof buf, f)) > 0) v.insert(v.end(), buf, buf + n);
    std::fclose(f);
    return v;
}
static uint32_t le32(const std::vector<uint8_t>& v, size_t o) { return v[o] | (v[o + 1] << 8) | (v[o + 2] << 16) | ((uint32_t)v[o + 3] << 24); }
static uint32_t le16(const std::vector<uint8_t>& v, size_t o) { return v[o] | (v[o + 1] << 8); }

TEST(world_cmdinfo_match) {
    std::string dir = NVW_DUMP_PATH;
    dir = dir.substr(0, dir.find_last_of("/\\") + 1);
    struct C { const nvhooks::HookSpec* spec; const char* info; uint32_t opcode; uint32_t needsParent, numParams; };
    const C cs[] = {
        {&world_addrs::Cmd_MarkForDelete, "MarkForDelete", 0x11bb, 1, 0},
        {&world_addrs::Cmd_SetEssential, "SetEssential", 0x1003, 0, 2},
        {&world_addrs::Cmd_SetActorRefEssential, "SetActorRefEssential", 0x1273, 0, 1},
        {&world_addrs::Cmd_SetRestrained, "SetRestrained", 0x10f3, 0, 1},
        {&world_addrs::Cmd_SetEnemy, "SetEnemy", 0x1078, 0, 4},
        {&world_addrs::Cmd_SetAlly, "SetAlly", 0x1079, 0, 4},
        {&world_addrs::Cmd_RestoreActorValue, "RestoreActorValue", 0x1182, 1, 2},
        {&world_addrs::Cmd_IgnoreCrime, "IgnoreCrime", 0x10ab, 1, 1},
        {&world_addrs::Cmd_SetPCEnemyOfFaction, "SetPCEnemyOfFaction", 0x10c6, 0, 2},
        {&world_addrs::Cmd_ClearFactionPlayerEnemyFlag, "ClearFactionPlayerEnemyFlag", 0x11c9, 0, 1},
    };
    for (const C& c : cs) {
        auto info = readFile(dir + "cmdinfo_" + c.info + ".bin");
        CHECK(info.size() >= 0x24);
        if (info.size() < 0x24) continue;
        uint32_t params = c.numParams;
        if (FNV_MUT(41)) {
            params += 1;  // MUTANT: a spec table that disagrees with the engine's parameter count
        }
        CHECK(le32(info, 8) == c.opcode);
        CHECK(le32(info, 0x18) == c.spec->target);
        CHECK(le16(info, 0x10) == c.needsParent);
        CHECK(le16(info, 0x12) == params);
        auto code = readFile(dir + c.spec->name + std::string(".bin"));
        CHECK(code.size() >= 16 && std::memcmp(code.data(), c.spec->expected, 16) == 0);
    }
}

static std::vector<DumpRow> readDumpFile(const std::string& path) {
    std::vector<DumpRow> rows;
    FILE* f = std::fopen(path.c_str(), "r");
    if (!f) return rows;
    char line[1024];
    while (std::fgets(line, sizeof line, f)) {
        char name[128], hex[64];
        unsigned addr = 0;
        int stolen = 0;
        if (std::sscanf(line, "%127s 0x%x stolen=%d bytes16=%32s", name, &addr, &stolen, hex) != 4) continue;
        DumpRow r;
        r.name = name;
        r.addr = addr;
        r.stolen = stolen;
        for (int i = 0; i < 16; ++i) {
            unsigned b = 0;
            std::sscanf(hex + 2 * i, "%2x", &b);
            r.bytes.push_back((uint8_t)b);
        }
        rows.push_back(r);
    }
    std::fclose(f);
    return rows;
}

TEST(owner_specs_match_the_static_dump) {
    std::string dir = NVW_DUMP_PATH;
    dir = dir.substr(0, dir.find_last_of("/\\") + 1);
    auto rows = readDumpFile(dir + "owner_prologues.txt");
    CHECK(rows.size() == 4);
    CHECK(world_owner_addrs::kCount == 4);
    for (size_t i = 0; i < world_owner_addrs::kCount; ++i) {
        nvhooks::HookSpec s = *world_owner_addrs::kAll[i];
        if (FNV_MUT(51)) s.expected[2] ^= 0x01;   // MUTANT 51: a spec that drifted from the evidence
        const DumpRow* r = nullptr;
        for (auto& x : rows) if (x.name == s.name) r = &x;
        CHECK(r != nullptr);
        if (!r) continue;
        CHECK(r->addr == s.target);
        CHECK(r->stolen == s.stolen);
        CHECK(std::memcmp(r->bytes.data(), s.expected, 16) == 0);
        CHECK(s.stolen >= 5 && s.expected[0] == 0x55 && s.expected[1] == 0x8b && s.expected[2] == 0xec);
        CHECK(s.retBytes == 0);
    }
}

// ---------------------------------------------------------------------------------------------- world.query on detoured evaluators
// Track A inbox 2026-10-21: our own capture detour on Cond_GetActorValue (and Cond_GetItemCount) changed the evaluator's first bytes to E9,
// and the old "must start with 55 8B EC" check made world.query GetActorValue fail for every ref. Twins 43..46 live in world_query.cpp.
static ReadMem memReader(FakeMem& m) { return [&m](uint32_t a, void* o, size_t n) { return m.read(a, o, n); }; }

static void putEntry(FakeMem& mem, const nvhooks::HookSpec& s) {
    mem.img[s.target] = std::vector<uint8_t>(s.expected, s.expected + s.expectedLen);
    mem.img[s.target].resize(64, 0x90);
}

TEST(query_table) {
    size_t n = 0;
    const Evaluator* t = evaluators(&n);
    CHECK(n >= 12);
    for (size_t i = 0; i < n; ++i) {
        CHECK(t[i].addr >= 0x590000 && t[i].addr < 0x5B0000);
        const Evaluator* f = findEvaluator(t[i].name);
        CHECK(f && f->addr == t[i].addr);
    }
    CHECK(findEvaluator("NoSuchFunction") == nullptr);
    CHECK(findEvaluator("getactorvalue") == nullptr);  // exact names, like the console grammar the callers use
    // the two evaluators our capture hooks patch must be in the table at the same addresses (that is the case the fix is for)
    CHECK(findEvaluator("GetActorValue") && findEvaluator("GetActorValue")->addr == nvhooks::addr::Cond_GetActorValue.target);
    CHECK(findEvaluator("GetItemCount") && findEvaluator("GetItemCount")->addr == nvhooks::addr::Cond_GetItemCount.target);
    CHECK(findEvaluator("GetItemCount")->formParam && !findEvaluator("GetActorValue")->formParam);
}

TEST(query_clean_entry_is_called_live) {
    FakeMem mem;
    const Evaluator& q = *findEvaluator("GetRestrained");
    mem.img[q.addr] = {0x55, 0x8B, 0xEC, 0x83, 0xEC, 0x08, 0x90, 0x90};
    EvalPlan p = resolveEvaluator(q, memReader(mem), [](uint32_t) { return 0u; });
    CHECK(p.kind == EvalCall::Live && p.addr == q.addr);
}

TEST(query_our_detour_is_called_through_the_trampoline) {
    for (const char* name : {"GetActorValue", "GetItemCount"}) {
        const Evaluator& q = *findEvaluator(name);
        const nvhooks::HookSpec& spec = std::string(name) == "GetActorValue" ? nvhooks::addr::Cond_GetActorValue : nvhooks::addr::Cond_GetItemCount;
        FakeMem mem;
        putEntry(mem, spec);
        nvhooks::HookRegistry reg(mem);
        nvhooks::Addr tramp = 0;
        CHECK(reg.acquire(spec, 0x30000000, &tramp) == nvhooks::PatchStatus::Ok);
        CHECK(mem.img[spec.target][0] == 0xE9);  // the live entry is now OUR jump: the old check refused exactly this
        CHECK(reg.trampolineOf(spec.target) == tramp && tramp != 0);
        CHECK(nvhooks::trampolineOfAny(spec.target) == tramp);
        CHECK(nvhooks::trampolineOfAny(spec.target + 0x10) == 0);
        EvalPlan p = resolveEvaluator(q, memReader(mem), [&](uint32_t t) { return static_cast<uint32_t>(nvhooks::trampolineOfAny(t)); });
        CHECK(p.kind == EvalCall::Trampoline);
        CHECK(p.addr == tramp);
        CHECK(p.addr != q.addr);
        // what is at the trampoline is the ORIGINAL function start (stolen bytes), followed by a jump back into the evaluator
        uint8_t tb[16] = {};
        CHECK(mem.read(tramp, tb, spec.stolen + 5));
        CHECK(std::memcmp(tb, spec.expected, spec.stolen) == 0);
        CHECK(tb[spec.stolen] == 0xE9);
        // hook released: the entry is original again and is called live
        CHECK(reg.release(spec) == nvhooks::PatchStatus::Restored);
        CHECK(nvhooks::trampolineOfAny(spec.target) == 0);
        p = resolveEvaluator(q, memReader(mem), [&](uint32_t t) { return static_cast<uint32_t>(nvhooks::trampolineOfAny(t)); });
        CHECK(p.kind == EvalCall::Live && p.addr == q.addr);
    }
}

TEST(query_suspended_hook_is_called_live) {
    const Evaluator& q = *findEvaluator("GetActorValue");
    const nvhooks::HookSpec& spec = nvhooks::addr::Cond_GetActorValue;
    FakeMem mem;
    putEntry(mem, spec);
    nvhooks::HookRegistry reg(mem);
    nvhooks::Addr tramp = 0;
    CHECK(reg.acquire(spec, 0x30000000, &tramp) == nvhooks::PatchStatus::Ok);
    CHECK(reg.suspendAll() == 0);  // reload drain: original bytes are back at the entry but the trampoline is still valid
    EvalPlan p = resolveEvaluator(q, memReader(mem), [&](uint32_t t) { return static_cast<uint32_t>(reg.trampolineOf(t)); });
    CHECK(p.kind == EvalCall::Live && p.addr == q.addr);
    CHECK(reg.resumeAll() == 0);
    p = resolveEvaluator(q, memReader(mem), [&](uint32_t t) { return static_cast<uint32_t>(reg.trampolineOf(t)); });
    CHECK(p.kind == EvalCall::Trampoline && p.addr == tramp);
}

TEST(query_foreign_or_broken_entries_are_refused) {
    const Evaluator& q = *findEvaluator("GetActorValue");
    auto none = [](uint32_t) { return 0u; };
    {   // somebody else's detour: E9 but not in our registry
        FakeMem mem;
        mem.img[q.addr] = {0xE9, 0x10, 0x20, 0x30, 0x00, 0x90, 0x90, 0x90};
        EvalPlan p = resolveEvaluator(q, memReader(mem), none);
        CHECK(p.kind == EvalCall::Refuse && p.addr == 0);
    }
    {   // the entry changed to something that is neither the prologue nor a jump (other exe build, other patcher)
        FakeMem mem;
        mem.img[q.addr] = {0xC3, 0x8B, 0xEC, 0x83, 0xEC, 0x0C, 0x90, 0x90};
        CHECK(resolveEvaluator(q, memReader(mem), none).kind == EvalCall::Refuse);
        mem.img[q.addr] = {0x55, 0x8B, 0xEB, 0x83, 0xEC, 0x0C, 0x90, 0x90};  // one byte off the prologue
        CHECK(resolveEvaluator(q, memReader(mem), none).kind == EvalCall::Refuse);
    }
    {   // unreadable memory
        FakeMem mem;
        EvalPlan p = resolveEvaluator(q, memReader(mem), none);
        CHECK(p.kind == EvalCall::Refuse && p.addr == 0);
    }
    {   // our registry names a trampoline, but its bytes are not the original prologue (corrupt / freed): never call it
        FakeMem mem;
        mem.img[q.addr] = {0xE9, 0x10, 0x20, 0x30, 0x00, 0x90, 0x90, 0x90};
        mem.heap[0x21000000] = {0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC};
        EvalPlan p = resolveEvaluator(q, memReader(mem), [](uint32_t) { return 0x21000000u; });
        CHECK(p.kind == EvalCall::Refuse && p.addr == 0);
        // and a trampoline that cannot be read at all
        p = resolveEvaluator(q, memReader(mem), [](uint32_t) { return 0x22000000u; });
        CHECK(p.kind == EvalCall::Refuse);
    }
}

TEST(world_actors_type_filter_reads_creatures_from_the_reference) {
    // spawn_proof 2026-10-22: `world.actors {type: 60}` (creatures) listed nothing, so "every instance has exactly one more actor" saw [0, 0, 0].
    const uint32_t kChar = 0x3B, kCreature = 0x3C, kNpcBase = 0x2A, kCreatureBase = 0x2B, kContainer = 0x1B, kDoor = 0x1C, kRefr = 0x3A;
    CHECK(actorTypeMatches(kChar, kChar, kNpcBase));                    // an NPC reference, as before
    CHECK(!actorTypeMatches(kChar, kCreature, kCreatureBase));          // a creature is not a character
    CHECK(actorTypeMatches(kCreature, kCreature, kCreatureBase));       // a radroach reference is a creature
    CHECK(!actorTypeMatches(kCreature, kChar, kNpcBase));
    CHECK(actorTypeMatches(kContainer, kRefr, kContainer));             // placed objects: the BASE form has the type
    CHECK(actorTypeMatches(kDoor, kRefr, kDoor));
    CHECK(!actorTypeMatches(kContainer, kRefr, kDoor));
    CHECK(actorTypeMatches(kRefr, kRefr, 0x28));                        // 0x3A asks for plain references
}

#include "refvars_tests.h"
#include "presentation_tests.h"

int main() {
    std::setvbuf(stdout, nullptr, _IONBF, 0);
    run("query_table", query_table);
    run("query_clean_entry_is_called_live", query_clean_entry_is_called_live);
    run("query_our_detour_is_called_through_the_trampoline", query_our_detour_is_called_through_the_trampoline);
    run("query_suspended_hook_is_called_live", query_suspended_hook_is_called_live);
    run("query_foreign_or_broken_entries_are_refused", query_foreign_or_broken_entries_are_refused);
    run("created_ref_map", created_ref_map);
    run("world_resolver_host", world_resolver_host);
    run("world_resolver_client", world_resolver_client);
    run("spawn_ledger_create_and_despawn", spawn_ledger_create_and_despawn);
    run("spawn_ledger_queue", spawn_ledger_queue);
    run("spawn_ledger_clear", spawn_ledger_clear);
    run("ref_var_tracker", ref_var_tracker);
    run("ref_var_budget", ref_var_budget);
    run("ref_var_forget", ref_var_forget);
    run("weather_time_sync", weather_time_sync);
    run("weather_block_local", weather_block_local);
    run("av_effects", av_effects);
    run("av_names_and_lines", av_names_and_lines);
    run("console_lines", console_lines);
    run("world_log_coalesce", world_log_coalesce);
    run("world_effect_json", world_effect_json);
    run("plan_spawn", plan_spawn);
    run("plan_spawn_missing_base", plan_spawn_missing_base);
    run("plan_despawn", plan_despawn);
    run("plan_ref_script_var", plan_ref_script_var);
    run("plan_world_effects", plan_world_effects);
    run("world_specs_pinned", world_specs_pinned);
    run("world_specs_match_dump", world_specs_match_dump);
    run("world_cmdinfo_match", world_cmdinfo_match);
    run("new_world_effects_lines_json_log", new_world_effects_lines_json_log);
    run("plan_new_world_effects", plan_new_world_effects);
    run("ownership_lines_json_log", ownership_lines_json_log);
    run("plan_ownership", plan_ownership);
    run("console_steps_are_scripts_and_run_outside_the_script_gate", console_steps_are_scripts_and_run_outside_the_script_gate);
    run("plan_object_state_clock_and_scripted_saves", plan_object_state_clock_and_scripted_saves);
    run("owner_specs_match_the_static_dump", owner_specs_match_the_static_dump);
    run("created_targets", created_targets);
    run("route_and_retriable", route_and_retriable);
    run("deferred_queue_basic", deferred_queue_basic);
    run("deferred_queue_retry_and_expiry", deferred_queue_retry_and_expiry);
    run("deferred_queue_cap", deferred_queue_cap);
    run("routed_follow_up_before_spawn", routed_follow_up_before_spawn);
    run("routed_follow_up_on_queued_spawn", routed_follow_up_on_queued_spawn);
    run("routed_despawn_drops_follow_ups", routed_despawn_drops_follow_ups);
    run("routed_retry_unloaded_form", routed_retry_unloaded_form);
    run("routed_retry_ref_var_rejected_and_fallback", routed_retry_ref_var_rejected_and_fallback);
    run("reconcile_and_plan", reconcile_and_plan);
    run("world_baseline_spawns_and_vars", world_baseline_spawns_and_vars);
    run("world_baseline_folds_net_state", world_baseline_folds_net_state);
    run("baseline_replay_on_client_is_idempotent", baseline_replay_on_client_is_idempotent);
    run("world_actors_type_filter_reads_creatures_from_the_reference", world_actors_type_filter_reads_creatures_from_the_reference);
    runRefVarTests();
    runPresentationTests();
    std::printf("%d checks, %d failed\n", g_checks, g_fail);
    return g_fail ? 1 : 0;
}
