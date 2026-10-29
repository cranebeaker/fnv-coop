// plugin/threat: the director. Once per tick, on the machine that RUNS the enemies' AI (their owner), it looks at every enemy that fights a player,
// scores the players (threat_model.h), and moves the enemy's combat target to another player when that player clearly wins. Only the owner decides;
// the copies on the other machines follow through the normal AiState.combatTarget sync (plugin/actors). Game-free: the world is IThreatPort.
#pragma once
#include <cstdint>
#include <deque>
#include <functional>
#include <map>
#include <string>
#include <vector>

#include "threat/threat_model.h"
#include "threat/threat_types.h"

namespace nvcoop::threat {

class ThreatDirector {
public:
    ThreatDirector(const Config& cfg, IThreatPort& port) : cfg_(cfg), port_(port) {}

    const Config& config() const { return cfg_; }
    void setEnabled(bool on) { cfg_.enabled = on; }
    void setLog(std::function<void(const std::string&)> log) { log_ = std::move(log); }

    // A hit feed: `slot` dealt `damage` to the enemy (the combat module's accepted hits, or the local engine's own).
    void noteHit(RefId enemy, Slot slot, float damage, uint64_t now);
    // Once per frame; does its work every Config::tickMs.
    void tick(uint64_t now);
    // A new world (a load, the end of a session): everything remembered is dropped.
    void reset();

    const Stats& stats() const { return stats_; }
    size_t tracked() const { return book_.size(); }
    const Record* record(RefId enemy) const;
    Slot targetOf(RefId enemy) const;                         // the player it was last observed to fight (kNoSlot = unknown)
    const std::deque<std::string>& recent() const { return recent_; }
    std::string statusJson() const;

private:
    void note(const std::string& line);
    void infer(Record& r, const EnemyView& e, const std::vector<PlayerView>& players, float drop, uint64_t now);

    Config cfg_;
    IThreatPort& port_;
    std::function<void(const std::string&)> log_;
    std::map<RefId, Record> book_;
    uint64_t nextAt_ = 0;
    uint32_t rotate_ = 0;
    Stats stats_;
    std::deque<std::string> recent_;
};

}  // namespace nvcoop::threat
