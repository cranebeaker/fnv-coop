// Bigger fights for bigger groups (Co-op balanced, host only): each generic enemy that starts fighting the party brings a share of an extra copy of itself, more
// with more players (three players: a quarter, four: a half). Never more health. The extra is made by the host's engine like any host-made actor, so the
// other machines get it through the normal actor sync; the guests do nothing. Game-free; the engine comes in through FightDirector::Env.
//
// The rule (accumulating shares, a cap on extras alive, no rush of them once some die) follows the other co-op mod's reinforcements; the observation (who is
// fighting the party) is ours: the host sees the fights of the actors it owns from its own engine and the fights of the actors a guest owns from the owner's
// reported combat target (the actor sync carries it).
#pragma once
#include <cstdint>
#include <functional>
#include <set>
#include <string>
#include <vector>

#include "rebalance/balance.h"

namespace nvcoop::rebalance {

// Shares of a copy owed. One call to take() makes at most one copy.
class Reinforcements {
public:
    void enemyJoined(float perEnemy) { owed_ += perEnemy; }
    // Whether to add a copy now (none while `maxAlive` are alive, and then not all that was owed: no rush of them once some die).
    bool take(int alive, int maxAlive);
    void reset() { owed_ = 0; }
    float owed() const { return owed_; }
private:
    float owed_ = 0;
};

// One enemy as the host sees it.
struct Fighter {
    uint32_t netId = 0;          // the same on every machine: an enemy that changes owner is still counted once
    uint32_t ref = 0;            // the host's reference for it (its own actor or its copy of a guest's)
    uint32_t baseId = 0;         // the form to make another of
    bool generic = false;        // not a named / unique character
    bool dead = false;
    bool fightsParty = false;    // its combat target is a player (the local one or a puppet)
    uint32_t targetRef = 0;      // that player's reference on the host
};

class FightDirector {
public:
    struct ExtraInfo { bool exists = false, dead = false, nearAnyPlayer = false; };
    struct Env {
        std::function<uint32_t(const Fighter& like, uint32_t targetRef)> spawn;   // a new enemy next to `like` fighting `targetRef`; 0 = it could not be made
        std::function<ExtraInfo(uint32_t ref)> inspect;
        std::function<void(uint32_t ref)> remove;                                  // an extra that is dead or left behind, out of everyone's sight
        std::function<void(const std::string&)> log;
    };
    explicit FightDirector(Env env) : env_(std::move(env)) {}
    // Once a second on the host. `balanced`: the campaign is Co-op balanced. `players`: the party size.
    void tick(const std::vector<Fighter>& fighters, int players, bool balanced);
    size_t extrasKnown() const { return extras_.size(); }
    uint64_t spawned() const { return spawned_; }
    uint64_t removed() const { return removed_; }
    uint64_t counted() const { return counted_; }
    void reset() { reinforcements_.reset(); seen_.clear(); extras_.clear(); }
    const Reinforcements& reinforcements() const { return reinforcements_; }
private:
    Env env_;
    Reinforcements reinforcements_;
    std::set<uint32_t> seen_;        // netIds of enemies already counted
    std::vector<uint32_t> extras_;   // refs of the extras made
    uint64_t spawned_ = 0, removed_ = 0, counted_ = 0;
};

}  // namespace nvcoop::rebalance
