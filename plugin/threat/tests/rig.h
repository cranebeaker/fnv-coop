// plugin/threat tests: a fake world behind IThreatPort. Players and enemies are plain tables; retarget() behaves like the engine (the enemy now fights that ref).
#pragma once
#include <map>
#include <utility>
#include <vector>

#include "threat/threat_director.h"

namespace nvth {

using namespace nvcoop::threat;

constexpr RefId kHostRef = 0x14;
inline RefId puppetRef(Slot s) { return 0x00200000u + s; }

struct FakeWorld : IThreatPort {
    std::vector<PlayerView> ps;
    std::vector<EnemyView> es;
    std::map<std::pair<RefId, RefId>, Sight> sights;      // (enemy, player ref)
    std::map<RefId, float> hp;
    std::vector<std::pair<RefId, RefId>> retargets;      // every retarget that was asked for, in order
    bool retargetOk = true;
    bool engineOverrules = false;                         // the engine's own AI puts the enemy back on its old target right after a retarget
    int sightCalls = 0;
    Sight defaultSight = Sight::Hidden;

    void players(std::vector<PlayerView>& out) override { out = ps; }
    void enemies(std::vector<EnemyView>& out) override { out = es; }
    Sight sight(RefId enemy, RefId player) override {
        ++sightCalls;
        auto it = sights.find({enemy, player});
        return it == sights.end() ? defaultSight : it->second;
    }
    bool health(RefId enemy, float& out) override {
        auto it = hp.find(enemy);
        if (it == hp.end()) return false;
        out = it->second;
        return true;
    }
    bool retarget(RefId enemy, RefId player) override {
        retargets.push_back({enemy, player});
        if (!retargetOk) return false;
        if (engineOverrules) return true;                 // accepted, then the engine goes back (the test flips the target itself)
        for (EnemyView& e : es)
            if (e.ref == enemy) e.combatTarget = player;
        return true;
    }

    PlayerView& player(Slot s) {
        for (PlayerView& p : ps)
            if (p.slot == s) return p;
        return ps.front();
    }
    EnemyView& enemy(RefId r) {
        for (EnemyView& e : es)
            if (e.ref == r) return e;
        return es.front();
    }
    PlayerView& addPlayer(Slot s, float x, float y, uint32_t space = 1) {
        PlayerView p;
        p.slot = s;
        p.ref = s == 0 ? kHostRef : puppetRef(s);
        p.space = space;
        p.pos = {x, y, 0};
        p.present = true;
        ps.push_back(p);
        return ps.back();
    }
    EnemyView& addEnemy(RefId r, float x, float y, RefId target, uint32_t space = 1) {
        EnemyView e;
        e.ref = r;
        e.mine = true;
        e.inCombat = target != 0;
        e.combatTarget = target;
        e.space = space;
        e.pos = {x, y, 0};
        es.push_back(e);
        return es.back();
    }
    RefId lastTargetOf(RefId enemy) const {
        for (const EnemyView& e : es)
            if (e.ref == enemy) return e.combatTarget;
        return 0;
    }
};

// A director over a fake world, with the numbers made plain: a tick every 500 ms.
struct Rig {
    FakeWorld w;
    Config cfg;
    ThreatDirector d;
    uint64_t now = 100000;
    Rig() : d(cfg, w) {}
    explicit Rig(const Config& c) : cfg(c), d(cfg, w) {}
    void tick() {
        d.tick(now);
        now += 500;
    }
    void ticks(int n) {
        for (int i = 0; i < n; ++i) tick();
    }
};

}  // namespace nvth
