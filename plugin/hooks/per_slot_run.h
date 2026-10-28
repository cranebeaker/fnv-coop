// Scripts that run once per joined player slot (G24 / G42 / G43, W3d item 1). Game-free; the detour is h_ScriptExec in engine_hooks.cpp.
//
// A PerPlayerState rule (Dead Money collar, speakers, vault and suites timers, kitchen traps, the cloud tick, the Old World Blues AutoDoc) names scripts
// (RulesEngine::perPlayerScripts) whose block must run for EACH player, not once for the host: the engine runs an object / quest script one time per frame and
// every `player` inside it is the host's player. The host therefore runs the same script again, right after the engine's own run, once for every joined
// guest with a puppet: the acting player is that guest (ActingPlayerScope, so `player.GetDistance`, `player.kill`, `player.RemoveAllItems` and the party
// conditions answer for that slot), the quest variables hold that slot's copy (RulesEngine::beginActing, done by the session in the runner), and the
// object variables the rule lists (`Script.var`, e.g. the speaker's bInRadius) hold that slot's copy of THIS script instance.
//
// The object variables are kept per INSTANCE (the script's event list): two speakers are two scripts with two bInRadius each, so a copy keyed by the
// variable name alone would hand one speaker's state to the other. The first time an instance is seen its values are taken as the seed every slot starts
// from (the engine has not run the instance's first frame yet at that point), a slot's copy is stored when its run ends, and the host's own values are put
// back before the engine goes on.
#pragma once
#include <cstdint>
#include <functional>
#include <map>
#include <string>
#include <tuple>
#include <utility>
#include <vector>

namespace nvhooks {

// One script the rules declare: its form id and the (lower-case) names of its own variables that keep one copy per slot.
struct PerSlotScript {
    uint32_t scriptId = 0;
    std::vector<std::string> vars;
};

class PerSlotScripts {
public:
    void set(std::vector<PerSlotScript> scripts) {
        byId_.clear();
        for (PerSlotScript& s : scripts)
            if (s.scriptId) byId_[s.scriptId] = std::move(s);
    }
    void clear() { byId_.clear(); }
    bool empty() const { return byId_.empty(); }
    size_t size() const { return byId_.size(); }
    const PerSlotScript* find(uint32_t scriptId) const {
#ifdef NVHOOKS_MUTANT
        (void)scriptId;   // mutant: no script is ever one that runs per slot
        return nullptr;
#else
        auto it = byId_.find(scriptId);
        return it == byId_.end() ? nullptr : &it->second;
#endif
    }

private:
    std::map<uint32_t, PerSlotScript> byId_;
};

// Per-instance copies of object variables. `live` points at the instance's variable slots, parallel to `names` (a null entry: the script has no such
// variable, skipped).
class ObjVarCopies {
public:
    struct Bracket {
        std::vector<double> host;   // the values to put back
    };
    // One stored value, for the save game: an instance is (script form id << 32 | the reference the script runs on), stable across a load (an event list's address is not).
    struct Copy { uint64_t instance = 0; std::string name; uint8_t slot = 0; double value = 0; };
    struct Seed { uint64_t instance = 0; std::string name; double value = 0; };
    std::vector<Copy> exportCopies() const {
        std::vector<Copy> out;
        for (const auto& kv : copies_) out.push_back({std::get<0>(kv.first), std::get<1>(kv.first), std::get<2>(kv.first), kv.second});
        return out;
    }
    std::vector<Seed> exportSeeds() const {
        std::vector<Seed> out;
        for (const auto& kv : seeds_) out.push_back({kv.first.first, kv.first.second, kv.second});
        return out;
    }
    // Replaces everything with what a save game held (a save without it passes two empty lists).
    void importAll(const std::vector<Copy>& copies, const std::vector<Seed>& seeds) {
        copies_.clear();
        seeds_.clear();
        for (const Copy& c : copies) copies_[Key{c.instance, c.name, c.slot}] = c.value;
        for (const Seed& s : seeds) seeds_[{s.instance, s.name}] = s.value;
    }
    // Before the engine's own run of an instance: remember the pristine values the first time the instance is seen.
    void seed(uint64_t instance, const std::vector<std::string>& names, const std::vector<double*>& live) {
        for (size_t i = 0; i < names.size() && i < live.size(); ++i)
            if (live[i] && !seeds_.count({instance, names[i]})) seeds_[{instance, names[i]}] = *live[i];
    }
    // Before the run for `slot`: that slot's copy (else the seed) is in the variables; the host's values are kept in the bracket.
    void enter(uint64_t instance, uint8_t slot, const std::vector<std::string>& names, const std::vector<double*>& live, Bracket& b) {
        b.host.assign(live.size(), 0.0);
        for (size_t i = 0; i < names.size() && i < live.size(); ++i) {
            if (!live[i]) continue;
            b.host[i] = *live[i];
            if (!seeds_.count({instance, names[i]})) seeds_[{instance, names[i]}] = *live[i];
            auto c = copies_.find(Key{instance, names[i], slotKey(slot)});
            *live[i] = c != copies_.end() ? c->second : seeds_[{instance, names[i]}];
        }
    }
    // After the run: the variables are that slot's copy now; the host's values go back.
    void leave(uint64_t instance, uint8_t slot, const std::vector<std::string>& names, const std::vector<double*>& live, const Bracket& b) {
        for (size_t i = 0; i < names.size() && i < live.size(); ++i) {
            if (!live[i]) continue;
#ifndef NVHOOKS_MUTANT
            copies_[Key{instance, names[i], slotKey(slot)}] = *live[i];   // mutant: the slot's state is lost after every run
#endif
            if (i < b.host.size()) *live[i] = b.host[i];
        }
    }
    bool copyOf(uint64_t instance, const std::string& name, uint8_t slot, double* out) const {
        auto c = copies_.find(Key{instance, name, slotKey(slot)});
        if (c == copies_.end()) return false;
        if (out) *out = c->second;
        return true;
    }
    size_t copies() const { return copies_.size(); }
    void clear() { copies_.clear(); seeds_.clear(); }

private:
    using Key = std::tuple<uint64_t, std::string, uint8_t>;
    static uint8_t slotKey(uint8_t slot) {
#ifdef NVHOOKS_MUTANT
        (void)slot;
        return 0;   // mutant: every slot shares one copy
#else
        return slot;
#endif
    }
    std::map<Key, double> copies_;
    std::map<std::pair<uint64_t, std::string>, double> seeds_;
};

// The key of a script instance that survives a save and a load: the script and the reference it runs on (a quest script runs on none).
inline uint64_t objVarInstance(uint32_t scriptId, uint32_t thisRefId) { return (static_cast<uint64_t>(scriptId) << 32) | thisRefId; }

// The session's side: calls `each(slot, ghostRefID)` once for every joined slot that has a puppet (the host's own slot is the engine's own run). The Runtime
// brackets each call with that slot's quest-variable copies and the commit origin.
using PerSlotEach = std::function<void(uint8_t slot, uint32_t ghostRefID)>;
using PerSlotRunner = std::function<void(uint32_t scriptRefID, const PerSlotEach& each)>;

}  // namespace nvhooks
