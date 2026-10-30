// plugin/areadigest tests (game-free), part 1: ini settings, the actor source over a fake facade, the shadow sources.
// Twins 61..63 (config), 64..68 and 70 (actor source), 72..74 (shadow sources) must each make one of these fail.
#include <algorithm>
#include <map>
#include <string>
#include "plugin_rig.h"

using namespace nvat;

namespace {

IniGetter iniOf(std::map<std::string, std::string> kv) {
    return [kv](const char* section, const char* key, const char* def) -> std::string {
        if (std::string(section) != "areadigest") return def;
        auto i = kv.find(key);
        return i == kv.end() ? std::string(def) : i->second;
    };
}

Item authority(uint32_t id, CellId cell, Vec3 pos, float hp = 100, bool alive = true) {
    Item it;
    it.key = id; it.cat = ad::Cat::Actors; it.cell = cell; it.pos = pos; it.hp = hp; it.alive = alive;
    it.settledMs = 9000; it.version = 1;
    return it;
}

// heal the local actor `id` to `auth`, the way AreaSync does it: look it up, compare, hand the plan over
HealResult healTo(ActorAreaSource& src, const Item& auth, uint64_t now, ad::Diff* planOut = nullptr) {
    Item local;
    if (!src.find(auth.key, local, now)) return HealResult::Missing;
    const ad::Diff plan = ad::compareItems(auth, local, ad::Config());
    if (planOut) *planOut = plan;
    return src.heal(auth, plan, now);
}

}  // namespace

// ---- config ----------------------------------------------------------------------------------------------------------------------
TEST(config, off_by_default_and_every_value_is_read) {
    CHECK(!readSettings(iniOf({})).enabled);
    AreaSettings s = readSettings(iniOf({{"enabled", "1"}, {"intervalMs", "4000"}, {"settleMs", "500"}, {"smoothMetres", "3"}, {"snapMetres", "12"},
                                        {"hpTolerance", "8"}, {"healBudgetPerMin", "100"}, {"maxActors", "100"}, {"story", "0"}, {"containers", "0"},
                                        {"objects", "no"}, {"actors", "off"}}));
    CHECK(s.enabled);                                                  // twins 61 (ini ignored) and 63 (on by default, see the first line)
    CHECK(s.cfg.intervalMs == 4000 && s.cfg.settleMs == 500);
    CHECK(s.cfg.smoothMetres == 3.0f && s.cfg.snapMetres == 12.0f && s.cfg.posBucketMetres == 3.0f);
    CHECK(s.cfg.hpTolerance == 8.0f && s.cfg.healBudgetPerMin == 100 && s.maxActors == 100);
    CHECK(!s.story && !s.cfg.compareStory && !s.containers && !s.objects && !s.actors);
    CHECK(s.warnings.empty());
    const AreaSettings d = readSettings(iniOf({}));
    CHECK(d.cfg.intervalMs == 2000 && d.cfg.settleMs == 1000 && d.cfg.smoothMetres == 2.0f && d.cfg.snapMetres == 10.0f);
    CHECK(d.actors && d.containers && d.objects && d.story && d.maxActors == 400);
    CHECK(describe(s).find("enabled=1") != std::string::npos && describe(d).find("enabled=0") != std::string::npos);
}

TEST(config, out_of_range_values_are_clamped_and_say_so) {
    AreaSettings s = readSettings(iniOf({{"enabled", "1"}, {"intervalMs", "10"}, {"settleMs", "99999"}, {"smoothMetres", "1000"}, {"hpTolerance", "0"},
                                        {"healBudgetPerMin", "-5"}, {"maxActors", "1"}}));
    CHECK(s.cfg.intervalMs == 500);                                    // twin 62: no clamp
    CHECK(s.cfg.settleMs == 10000);
    CHECK(s.cfg.smoothMetres == 50.0f && s.cfg.snapMetres >= s.cfg.smoothMetres + 0.5f);
    CHECK(s.cfg.hpTolerance == 0.5f && s.cfg.healBudgetPerMin == 0 && s.maxActors == 20);
    CHECK(!s.warnings.empty() && s.warnings.find("intervalMs") != std::string::npos);
}

TEST(config, text_that_is_not_a_number_keeps_the_default) {
    AreaSettings s = readSettings(iniOf({{"enabled", "1"}, {"intervalMs", "fast"}, {"snapMetres", "10 m"}}));
    CHECK(s.cfg.intervalMs == 2000 && s.cfg.snapMetres == 10.0f);
    CHECK(s.warnings.find("intervalMs") != std::string::npos);
}

TEST(config, the_health_bucket_stays_wider_than_the_tolerance) {
    AreaSettings s = readSettings(iniOf({{"hpTolerance", "40"}}));
    CHECK(s.cfg.hpBucket >= 2 * s.cfg.hpTolerance);
    CHECK(parseBool(" Yes ") && parseBool("TRUE") && parseBool("1") && parseBool("on") && !parseBool("") && !parseBool("2") && !parseBool("enable"));
}

// ---- actor source: reading ----------------------------------------------------------------------------------------------------
TEST(actorsource, collect_reads_every_tracked_actor_and_the_loaded_cells) {
    FakeFacade f;
    f.add(5, 0x100, V(10, 20, 30), 80);
    f.add(6, 0x101, V(500, 0, 0), 20, false);
    ad::Config cfg;
    ActorAreaSource src(f, cfg);
    std::vector<Item> items;
    std::vector<CellId> cells;
    src.collect(1000, items, cells);
    REQUIRE(items.size() == 2);
    CHECK(items[0].key == 5 && items[0].cell == 0x100 && items[0].hp == 80 && items[0].alive && items[0].pos.x == 10);
    CHECK(items[1].key == 6 && !items[1].alive);                      // twin 70: a dead actor reported alive
    CHECK(items[0].settledMs == 0 && items[0].version == 1);          // just appeared
    CHECK(std::count(cells.begin(), cells.end(), 0x100u) >= 1 && std::count(cells.begin(), cells.end(), 0x101u) >= 1);
    CHECK(std::count(cells.begin(), cells.end(), f.playerCell_) >= 1);
    items.clear(); cells.clear();
    src.collect(3500, items, cells);
    CHECK(items[0].settledMs == 2500 && items[0].version == 1);
    f.a[5].r.hp = 30;
    items.clear(); cells.clear();
    src.collect(4000, items, cells);
    CHECK(items[0].version == 2 && items[0].settledMs == 0);
    f.isReady = false;
    items.clear(); cells.clear();
    src.collect(5000, items, cells);
    CHECK(items.empty() && cells.empty());
}

TEST(actorsource, a_big_world_is_capped) {
    FakeFacade f;
    for (uint32_t i = 1; i <= 50; ++i) f.add(i, 0x100, V(float(i), 0, 0));
    ActorAreaSource src(f, ad::Config(), 20);
    std::vector<Item> items;
    std::vector<CellId> cells;
    src.collect(1000, items, cells);
    CHECK(items.size() == 20 && src.stats().capped == 1);
}

TEST(actorsource, find_is_a_fresh_read_with_the_tracked_version) {
    FakeFacade f;
    f.add(5, 0x100, V(0, 0, 0), 80);
    ActorAreaSource src(f, ad::Config());
    std::vector<Item> items;
    std::vector<CellId> cells;
    src.collect(1000, items, cells);
    Item it;
    CHECK(src.find(5, it, 2000) && it.hp == 80 && it.version == 1 && it.settledMs == 1000);
    f.a[5].r.hp = 10;
    CHECK(src.find(5, it, 2100) && it.hp == 10 && it.version == 2 && it.settledMs == 0);
    CHECK(!src.find(99, it, 2100));
    CHECK(!src.find(0x1FFFFFFFFull, it, 2100));
    OwnerView ov;
    f.a[5].own.owner = 2; f.a[5].own.epoch = 7;
    CHECK(src.owner(5, ov) && ov.known && ov.owner == 2 && ov.epoch == 7);
    CHECK(!src.owner(99, ov));
}

// ---- actor source: healing -------------------------------------------------------------------------------------------------------
TEST(actorsource, a_big_position_error_is_snapped_and_a_cell_error_moves_between_cells) {
    FakeFacade f;
    f.add(5, 0x100, V(0, 0, 0));
    ActorAreaSource src(f, ad::Config());
    ad::Diff plan;
    CHECK(healTo(src, authority(5, 0x100, V(15 * kU, 0, 0)), 1000, &plan) == HealResult::Applied);
    CHECK(plan.pos == ad::RubberMode::Snap);
    CHECK(f.a[5].r.pos.x == 15 * kU && f.calls.back() == "pos 5" && src.stats().snaps == 1 && src.smoothing() == 0);
    CHECK(healTo(src, authority(5, 0x105, V(1, 2, 3)), 2000) == HealResult::Applied);
    CHECK(f.a[5].r.cell == 0x105 && f.calls.back() == "cell 5 261");      // twin 67: moved with a same-cell write
    f.failMoves = true;
    CHECK(healTo(src, authority(5, 0x105, V(900, 0, 0)), 3000) == HealResult::Refused);
    CHECK(src.stats().moveFailed == 1);
}

// A copy that is still getting out of its chair is held in place by the actor port; a position heal moved it anyway (a 2 m .. 10 m error is smoothed, more is
// snapped), so Doc Mitchell floated across the room seated, a hop every digest round (2026-10-30).
TEST(actorsource, a_copy_that_is_held_while_it_gets_out_of_furniture_is_not_dragged_by_a_heal) {   // twin 83
    FakeFacade f;
    f.add(5, 0x100, V(0, 0, 0));
    ActorAreaSource src(f, ad::Config());
    f.held.insert(5);
    CHECK(healTo(src, authority(5, 0x100, V(15 * kU, 0, 0)), 1000) == HealResult::Refused);   // a snap-sized error
    CHECK(f.a[5].r.pos.x == 0 && src.stats().snaps == 0 && src.stats().heldByPoseLock == 1);
    CHECK(healTo(src, authority(5, 0x100, V(5 * kU, 0, 0)), 2000) == HealResult::Refused);    // a smooth-sized one
    CHECK(src.smoothing() == 0 && src.stats().smooths == 0);
    f.held.clear();                                                                           // out of the chair: the next round heals it
    CHECK(healTo(src, authority(5, 0x100, V(15 * kU, 0, 0)), 3000) == HealResult::Applied);
    CHECK(f.a[5].r.pos.x == 15 * kU);
    // a smoothing job that was running when the hold began waits instead of moving the body
    f.add(6, 0x100, V(0, 0, 0));
    CHECK(healTo(src, authority(6, 0x100, V(5 * kU, 0, 0)), 4000) == HealResult::Applied);
    f.held.insert(6);
    for (uint64_t t = 4000; t < 5000; t += 33) src.tick(t);
    CHECK(f.a[6].r.pos.x == 0 && src.smoothing() == 1);
    f.held.clear();
    for (uint64_t t = 5000; t < 7000; t += 33) src.tick(t);
    CHECK(f.a[6].r.pos.x == 5 * kU && src.smoothing() == 0);
}

TEST(actorsource, a_medium_position_error_is_smoothed_over_several_steps) {
    FakeFacade f;
    f.add(5, 0x100, V(0, 0, 0));
    ActorAreaSource src(f, ad::Config());
    ad::Diff plan;
    CHECK(healTo(src, authority(5, 0x100, V(5 * kU, 0, 0)), 1000, &plan) == HealResult::Applied);
    CHECK(plan.pos == ad::RubberMode::Smooth);
    CHECK(f.a[5].r.pos.x == 0 && src.smoothing() == 1 && src.stats().snaps == 0);     // twin 68: snapped at once
    float last = 0;
    int steps = 0;
    for (uint64_t t = 1000; t <= 3000; t += 33) {
        src.tick(t);
        const float x = f.a[5].r.pos.x;
        CHECK(x >= last);
        last = x;
        if (src.smoothing() == 0) break;
        ++steps;
    }
    CHECK(steps >= 10);
    CHECK(f.a[5].r.pos.x == 5 * kU && src.smoothing() == 0 && src.stats().smoothSteps >= 10);
}

TEST(actorsource, a_smoothing_job_ends_with_its_actor_and_a_snap_replaces_it) {
    FakeFacade f;
    f.add(5, 0x100, V(0, 0, 0));
    f.add(6, 0x100, V(0, 0, 0));
    ActorAreaSource src(f, ad::Config());
    CHECK(healTo(src, authority(5, 0x100, V(5 * kU, 0, 0)), 1000) == HealResult::Applied);
    CHECK(healTo(src, authority(6, 0x100, V(5 * kU, 0, 0)), 1000) == HealResult::Applied);
    CHECK(src.smoothing() == 2);
    f.a.erase(5);                                         // cell unloaded
    f.a[6].r.alive = false;                               // died meanwhile
    src.tick(1100);
    CHECK(src.smoothing() == 0);
    f.add(7, 0x100, V(0, 0, 0));
    CHECK(healTo(src, authority(7, 0x100, V(5 * kU, 0, 0)), 2000) == HealResult::Applied && src.smoothing() == 1);
    CHECK(healTo(src, authority(7, 0x100, V(30 * kU, 0, 0)), 2100) == HealResult::Applied);
    CHECK(src.smoothing() == 0 && f.a[7].r.pos.x == 30 * kU);   // the snap won, the old job is gone
}

TEST(actorsource, health_is_set_and_never_to_zero_on_a_living_actor) {
    FakeFacade f;
    f.add(5, 0x100, V(0, 0, 0), 20);
    ActorAreaSource src(f, ad::Config());
    CHECK(healTo(src, authority(5, 0x100, V(0, 0, 0), 90), 1000) == HealResult::Applied);
    CHECK(f.a[5].r.hp == 90 && f.calls.back() == "hp 5 90" && src.stats().hpSet == 1);
    f.a[5].r.hp = 80;
    CHECK(healTo(src, authority(5, 0x100, V(0, 0, 0), 0.2f), 2000) == HealResult::Applied);
    CHECK(f.a[5].r.hp == 1.0f);                           // a living authority never writes 0: that would be a death
}

TEST(actorsource, a_death_is_handed_to_the_actor_module_never_done_here) {
    FakeFacade f;
    f.add(5, 0x100, V(0, 0, 0), 100);
    f.add(6, 0x100, V(0, 0, 0), 100, true, /*named*/ true);
    ActorAreaSource src(f, ad::Config());
    CHECK(healTo(src, authority(5, 0x100, V(0, 0, 0), 0, false), 1000) == HealResult::Delegated);
    CHECK(f.resyncs.size() == 1 && f.resyncs[0] == 5 && f.a[5].r.alive && src.stats().deathRequests == 1);     // twin 64: the request is never made
    f.resyncOk = false;
    CHECK(healTo(src, authority(5, 0x100, V(0, 0, 0), 0, false), 2000) == HealResult::Refused);
    // a named NPC dies through the story host only
    f.resyncOk = true;
    CHECK(healTo(src, authority(6, 0x100, V(0, 0, 0), 0, false), 3000) == HealResult::Refused);
    CHECK(f.resyncs.size() == 2);                         // twin 65: a named actor is also sent through (3 requests)
    CHECK(f.a[6].r.alive);
}

TEST(actorsource, a_corpse_is_never_revived_by_a_heal) {
    FakeFacade f;
    f.add(5, 0x100, V(0, 0, 0), 0, false);
    ActorAreaSource src(f, ad::Config());
    CHECK(healTo(src, authority(5, 0x100, V(0, 0, 0), 100, true), 1000) == HealResult::Refused);
    CHECK(!f.a[5].r.alive && f.calls.empty() && f.resyncs.empty());
    CHECK(src.stats().refused == 1);                      // twin 66: the corpse is revived through a health write
}

TEST(actorsource, an_actor_that_is_gone_is_missing) {
    FakeFacade f;
    ActorAreaSource src(f, ad::Config());
    CHECK(healTo(src, authority(5, 0x100, V(0, 0, 0)), 1000) == HealResult::Missing);
    Item a = authority(5, 0x100, V(0, 0, 0));
    ad::Diff plan;
    plan.material = true; plan.hp = true;
    CHECK(src.heal(a, plan, 1000) == HealResult::Missing && src.stats().missing == 1);
}

// ---- shadow sources ----------------------------------------------------------------------------------------------------------------
namespace {
struct Shadow {
    FakeWorld w;
    ShadowRefSource containers, objects;
    Shadow()
        : containers(ad::Cat::Containers, [this]() { return &w.st; }, nullptr, ad::Config()),
          objects(ad::Cat::Objects, [this]() { return &w.st; }, nullptr, ad::Config()) {}
};

Item byKey(const std::vector<Item>& v, uint32_t localId) {
    const ad::Key k = keyOfRef(FormRef("falloutnv.esm", localId));
    for (const Item& it : v) if (it.key == k) return it;
    Item none;
    none.key = 0;
    return none;
}
}  // namespace

TEST(shadow, ref_keys_are_stable_and_case_insensitive) {
    CHECK(keyOfRef(FormRef("FalloutNV.esm", 0x1234)) == keyOfRef(FormRef("falloutnv.esm", 0x1234)));
    CHECK(keyOfRef(FormRef("falloutnv.esm", 0x1234)) != keyOfRef(FormRef("falloutnv.esm", 0x1235)));
    CHECK(keyOfRef(FormRef("falloutnv.esm", 0x1234)) != keyOfRef(FormRef("dlc.esm", 0x1234)));
    CHECK(keyOfRef(FormRef("", 0)) != 0);
}

TEST(shadow, container_contents_change_the_state) {
    Shadow s;
    std::vector<Item> items;
    std::vector<CellId> cells;
    s.containers.collect(1000, items, cells);
    CHECK(items.empty() && std::count(cells.begin(), cells.end(), 0u) == 1);          // nothing yet, but "everywhere" is loaded
    s.w.addItem(0x500, 0x10, 3);
    s.containers.collect(2000, items, cells);
    REQUIRE(items.size() == 1);
    const uint32_t before = items[0].state;
    s.w.addItem(0x500, 0x11, 1);
    items.clear(); cells.clear();
    s.containers.collect(3000, items, cells);
    REQUIRE(items.size() == 1);
    CHECK(items[0].state != before);                                                   // twin 73: a missing item is invisible
    CHECK(items[0].version == 2 && s.containers.delegated());
    // the same contents on another machine give the same state
    Shadow t;
    t.w.addItem(0x500, 0x10, 3);
    t.w.addItem(0x500, 0x11, 1);
    std::vector<Item> other;
    std::vector<CellId> oc;
    t.containers.collect(9000, other, oc);
    REQUIRE(other.size() == 1);
    CHECK(other[0].state == items[0].state && other[0].key == items[0].key);
}

TEST(shadow, object_open_lock_and_destroyed_state_change_the_item) {
    Shadow s;
    s.w.setLock(0x600, true, 50);
    std::vector<Item> items;
    std::vector<CellId> cells;
    s.objects.collect(1000, items, cells);
    REQUIRE(items.size() == 1);
    const uint32_t locked = items[0].state;
    CHECK(items[0].alive);
    s.w.setLock(0x600, true, 75);
    items.clear();
    s.objects.collect(2000, items, cells);
    CHECK(items[0].state != locked);                                                   // twin 72: lock changes are invisible
    {
        fnvcoop::EffectTxn t;
        t.seq = ++s.w.seq;
        fnvcoop::ObjectState o;
        o.ref = FormRef("falloutnv.esm", 0x600);
        o.mask = fnvcoop::kObjDestroyed;
        o.destroyed = true;
        t.effects.push_back(o);
        s.w.st.applyTxn(t);
    }
    items.clear();
    s.objects.collect(3000, items, cells);
    CHECK(!items[0].alive);                                                            // twin 74: destroyed is ignored
    Item it;
    CHECK(s.objects.find(items[0].key, it, 4000) && !it.alive && it.version == items[0].version);
    CHECK(!s.objects.find(12345, it, 4000));
    CHECK(s.objects.heal(it, ad::Diff(), 1) == HealResult::Refused);
}

TEST(shadow, a_cell_resolver_puts_references_into_their_cells) {
    FakeWorld w;
    ShadowRefSource objs(ad::Cat::Objects, [&]() { return &w.st; },
                         [](const FormRef& r, CellId& c) { if (r.localId == 0x600) { c = 0x2A; return true; } return false; }, ad::Config());
    w.setLock(0x600, true, 1);
    w.setLock(0x601, true, 1);
    std::vector<Item> items;
    std::vector<CellId> cells;
    objs.collect(1000, items, cells);
    CHECK(byKey(items, 0x600).cell == 0x2A && byKey(items, 0x601).cell == 0);
    CHECK(std::count(cells.begin(), cells.end(), 0x2Au) >= 1);
}

TEST(shadow, the_story_source_reports_the_session_state_and_asks_for_repairs) {
    FakeWorld w;
    w.stage(0x100, 10);
    ShadowStorySource st;
    ad::StoryDigest none;
    CHECK(!st.local(none));                                              // no hook, no digest
    st.world = [&](ShadowStorySource::World& x) { x.state = &w.st; x.seq = w.seq; x.epoch = w.epoch; return true; };
    st.repair = [&](fnvcoop::PlayerId p) { w.repairAsked.push_back(p); };
    st.repairCount = [&]() { return w.repairs; };
    ad::StoryDigest d;
    REQUIRE(st.local(d));
    CHECK(d.ok && d.seq == 1 && d.epoch == 1);
    const uint64_t h = d.hash;
    w.stage(0x100, 20);
    REQUIRE(st.local(d));
    CHECK(d.hash != h && d.seq == 2);
    w.epoch = 3;
    REQUIRE(st.local(d));
    CHECK(d.epoch == 3);
    st.requestRepair(2);
    w.repairs = 4;
    CHECK(w.repairAsked.size() == 1 && w.repairAsked[0] == 2 && st.repairs() == 4);
    st.world = [](ShadowStorySource::World&) { return false; };
    CHECK(!st.local(d));
}
