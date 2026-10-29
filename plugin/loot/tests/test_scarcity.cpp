// LootSync's scarcity hook (plugin/rebalance, Co-op balanced; docs/rebalance.md): a container or corpse THIS machine registers is thinned once, before its contents become
// the shared ones, and the engine copy is made to match. Twins 1490..1494 are guarded here.
#include "rig.h"

using namespace nvl_test;

namespace {

constexpr uint32_t kStim = 20, kBox = 21, kAmmo = 30, kWater = 31;

void classify(Cluster& c) {
    for (auto& mc : c.m) {
        mc->eng.classes[F(kStim)] = ItemClass::Supply;
        mc->eng.classes[F(kWater)] = ItemClass::Supply;
        mc->eng.classes[F(kAmmo)] = ItemClass::Ammo;
    }
}
void share(Cluster& c, NetId net, const Items& items, lc::ContainerKind k = lc::ContainerKind::Container) {
    for (auto& mc : c.m) mc->eng.add(net, items, k);
}
const lc::ContainerRec* tableOf(Cluster& c, NetId net) { return c.host().sync->host()->container(net); }

// A stand-in for plugin/rebalance's rule: half of every supply stack (rounded down), nothing else. It records what it was told.
struct Probe {
    int calls = 0;
    std::vector<NetId> nets;
    bool sawCorpse = false, sawUnique = false;
    TrimHook hook(bool changes = true) {
        return [this, changes](const TrimContext& c, Items& items) {
            ++calls;
            nets.push_back(c.net);
            sawCorpse = c.kind == lc::ContainerKind::Corpse;
            sawUnique = c.uniqueActor;
            bool did = false;
            for (auto& kv : items) {
                if (changes && c.classify(kv.first) == ItemClass::Supply && kv.second > 1) { kv.second /= 2; did = true; }
            }
            return did;
        };
    }
};

}  // namespace

TEST(scarcity, a_corpse_the_owner_registers_is_thinned_once_and_every_machine_ends_with_the_same_list) {
    Cluster c(3);
    classify(c);
    share(c, 0x2000, I({{kStim, 40}, {kBox, 5}, {kAmmo, 40}}), lc::ContainerKind::Corpse);
    Machine& g = c.machine(1);
    Probe p;
    g.sync->setTrimHook(p.hook());
    c.run(30);
    g.sync->onActorDied(0x2000, g.eng.conts[0x2000].ref);
    c.run(900);
    REQUIRE(tableOf(c, 0x2000));
    CHECK(p.calls == 1 && p.nets == std::vector<NetId>{0x2000} && p.sawCorpse && !p.sawUnique);     // twin 1490: the hook is never asked
    const Items thin = I({{kStim, 20}, {kBox, 5}, {kAmmo, 40}});
    CHECK(tableOf(c, 0x2000)->items == thin);                                                       // the shared list is the thinned one
    CHECK(g.eng.conts[0x2000].items == thin);                                                       // the registering machine's own copy was made to match
    c.settle();
    CHECK(c.host().eng.conts[0x2000].items == thin && c.machine(2).eng.conts[0x2000].items == thin);   // and so is everybody else's
    CHECK(g.sync->stats().trimmedContainers == 1 && g.sync->stats().trimmedItems == 20);
    CHECK(g.sync->statusJson().find("\"trim\":{\"hook\":true,\"containers\":1,\"items\":20}") != std::string::npos);
}

TEST(scarcity, the_engine_copy_is_changed_when_the_contents_are_registered_not_left_to_the_audit) {
    Cluster c(2);
    classify(c);
    share(c, 0x2000, I({{kStim, 40}}), lc::ContainerKind::Corpse);
    Machine& g = c.machine(1);
    Probe p;
    g.sync->setTrimHook(p.hook());
    c.run(30);
    g.sync->onActorDied(0x2000, g.eng.conts[0x2000].ref);
    for (int i = 0; i < 100 && g.sync->stats().corpsesResolved == 0; ++i) c.step(10);
    REQUIRE(g.sync->stats().corpsesResolved == 1);
    CHECK(g.eng.setContentsCalls >= 1 && g.eng.conts[0x2000].items == I({{kStim, 20}}));              // twin 1492: the engine copy keeps the full list
}

TEST(scarcity, two_events_for_one_container_thin_it_once) {
    Cluster c(2);
    classify(c);
    share(c, 0x100, I({{kStim, 40}, {kAmmo, 7}}));
    Machine& g = c.machine(1);
    Probe p;
    g.sync->setTrimHook(p.hook());
    c.run(30);
    g.sync->onContainerLoaded(g.eng.conts[0x100].ref);
    g.sync->onContainerLoaded(g.eng.conts[0x100].ref);                       // a cell attach AND a discovery pass: the host has not answered yet
    c.settle();
    CHECK(p.calls == 1);                                                     // twin 1491: the rule runs again for the same container
    CHECK(g.eng.conts[0x100].items == I({{kStim, 20}, {kAmmo, 7}}));         // not 10
    REQUIRE(tableOf(c, 0x100));
    CHECK(tableOf(c, 0x100)->items == I({{kStim, 20}, {kAmmo, 7}}));
}

TEST(scarcity, what_the_engine_would_not_give_up_stays_in_the_shared_list) {
    Cluster c(2);
    classify(c);
    share(c, 0x100, I({{kStim, 40}, {kWater, 10}}));
    Machine& g = c.machine(1);
    g.eng.stuck.insert(F(kStim));                                            // the engine will not take the Stimpaks out
    Probe p;
    g.sync->setTrimHook(p.hook());
    c.run(30);
    g.sync->onContainerLoaded(g.eng.conts[0x100].ref);
    c.run(300);
    REQUIRE(tableOf(c, 0x100));
    const Items real = I({{kStim, 40}, {kWater, 5}});
    CHECK(g.eng.conts[0x100].items == real);
    CHECK(tableOf(c, 0x100)->items == real);                                 // twin 1493: the shared list says 20 Stimpaks the engine still holds 40 of
}

TEST(scarcity, the_body_of_a_unique_character_is_flagged_for_the_rule) {
    Cluster c(2);
    classify(c);
    share(c, 0x2000, I({{kStim, 40}}), lc::ContainerKind::Corpse);
    c.machine(1).eng.conts[0x2000].uniqueActor = true;
    Probe p;
    c.machine(1).sync->setTrimHook(p.hook(false));
    c.run(30);
    c.machine(1).sync->onActorDied(0x2000, c.machine(1).eng.conts[0x2000].ref);
    c.run(900);
    CHECK(p.calls == 1 && p.sawUnique);
    REQUIRE(tableOf(c, 0x2000));
    CHECK(tableOf(c, 0x2000)->items == I({{kStim, 40}}));
    CHECK(c.machine(1).eng.setContentsCalls == 0);                           // a hook that changes nothing leaves the engine alone
    CHECK(c.machine(1).sync->stats().trimmedContainers == 0);
}

TEST(scarcity, no_hook_no_change) {
    Cluster c(2);
    classify(c);
    share(c, 0x100, I({{kStim, 40}}));
    c.run(30);
    c.machine(1).sync->onContainerLoaded(c.machine(1).eng.conts[0x100].ref);
    c.settle();
    REQUIRE(tableOf(c, 0x100));
    CHECK(tableOf(c, 0x100)->items == I({{kStim, 40}}) && c.machine(1).eng.setContentsCalls == 0);
    CHECK(c.machine(1).sync->statusJson().find("\"trim\":{\"hook\":false") != std::string::npos);
}

TEST(scarcity, a_new_session_thins_its_containers_again) {
    Cluster c(1);
    classify(c);
    share(c, 0x100, I({{kStim, 40}}));
    Machine& h = c.host();
    Probe p;
    h.sync->setTrimHook(p.hook());
    c.run(30);
    h.sync->onContainerLoaded(h.eng.conts[0x100].ref);
    c.settle();
    CHECK(p.calls == 1 && h.eng.conts[0x100].items == I({{kStim, 20}}));
    // the campaign is left and another one starts: the world is new (full contents again, an empty table), the rule must apply to it
    h.sync->sessionEnd();
    h.eng.conts[0x100].items = I({{kStim, 40}});
    ll::SessionInfo si;
    si.isHost = true; si.self = 0;
    h.sync->sessionBegin(si, h.link);
    c.run(30);
    h.sync->onContainerLoaded(h.eng.conts[0x100].ref);
    c.settle();
    CHECK(p.calls == 2);                                                     // twin 1494: the old session's "already thinned" survives
    CHECK(h.eng.conts[0x100].items == I({{kStim, 20}}));
}
