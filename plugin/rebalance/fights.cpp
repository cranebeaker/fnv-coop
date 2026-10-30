#include "rebalance/fights.h"

#include <algorithm>

namespace nvcoop::rebalance {

bool Reinforcements::take(int alive, int maxAlive) {
    if (alive >= maxAlive && !FNV_MUT(1240)) {   // MUTANT 1240: no cap on extras alive
        owed_ = owed_ > 1 ? 1 : owed_;
        return false;
    }
    if (owed_ < 1) return false;
    owed_ -= 1;
    return true;
}

void FightDirector::tick(const std::vector<Fighter>& fighters, int players, bool balanced) {
    // Extras that are dead or left behind, out of everyone's sight, are cleared away; a body near the group stays to be looted.
    int alive = 0;
    for (auto it = extras_.begin(); it != extras_.end();) {
        const ExtraInfo info = env_.inspect ? env_.inspect(*it) : ExtraInfo();
        if (!info.exists) { it = extras_.erase(it); continue; }
        if (!info.nearAnyPlayer && !FNV_MUT(1241)) {   // MUTANT 1241: extras are never cleared away
            if (env_.remove) env_.remove(*it);
            ++removed_;
            it = extras_.erase(it);
            continue;
        }
        alive += info.dead ? 0 : 1;
        ++it;
    }
    const float per = balanced ? extrasPerEnemy(players) : 0.0f;
    if (per <= 0) return;
    if (seen_.size() > 4096) seen_.clear();   // a long session without loads: counted again at worst
    const Fighter* copyOf = nullptr;   // an enemy to make another of: any generic one still fighting the party
    for (const Fighter& f : fighters) {
        if (f.dead || !f.fightsParty || !f.netId) continue;
        if (std::find(extras_.begin(), extras_.end(), f.ref) != extras_.end()) { seen_.insert(f.netId); continue; }   // an extra is never copied again
        if (!seen_.insert(f.netId).second && !FNV_MUT(1242)) continue;   // MUTANT 1242: an enemy that changes owner is counted again
        if (!f.generic) continue;   // a named character brings no extras
        ++counted_;
        reinforcements_.enemyJoined(per);
        copyOf = &f;
    }
    if (!copyOf && reinforcements_.owed() >= 1) {   // shares owed from earlier ticks: one more copy, behind any generic enemy still fighting
        for (const Fighter& f : fighters)
            if (!f.dead && f.fightsParty && f.generic && f.netId && std::find(extras_.begin(), extras_.end(), f.ref) == extras_.end()) copyOf = &f;
    }
    if (copyOf && reinforcements_.take(alive, maxExtras(players))) {
        const uint32_t made = env_.spawn ? env_.spawn(*copyOf, copyOf->targetRef) : 0;
        if (made) {
            extras_.push_back(made);
            ++spawned_;
            if (env_.log) env_.log("rebalance: an extra enemy joins the fight (" + std::to_string(players) + " players, " + std::to_string(alive + 1) + " extras alive)");
        }
    }
}

}  // namespace nvcoop::rebalance
