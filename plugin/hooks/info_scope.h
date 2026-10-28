// The condition scope of an INFO (W3d item 2, W3b request 6). Game-free; the detour is h_InfoConditions in engine_hooks.cpp.
//
// The party-aware condition evaluators reduce their per-member values under the rules of "the quest" (RulesEngine::reduceCondition). A quest script finds its
// quest through the capture scope that surrounds it, but the conditions of an INFO are evaluated while the dialogue menu is BUILT, outside any scope, so the Jed
// weight gate (`GetActorValue InventoryWeight >= 76` on INFOs HonestHearts.esm:009EB6 / 011525, "the heaviest pack of the party") and the Shaman's root count
// (0094AA) would reduce as `initiator`. The engine evaluates one INFO's conditions in one function (TESTopicInfo::CheckConditions 0x61E600, thiscall on the INFO):
// the detour opens a ConditionQuestScope for that INFO's quest around it. The INFO -> quest table is data (nvcoop_infoquest.json, plugin/coop/tools/gen_coop_data.py).
#pragma once
#include <cstdint>
#include <functional>
#include <unordered_map>

#include "fnvcoop/types.h"

namespace nvhooks {

// info form id (runtime) -> the quest whose rules apply to it. The session gives the lookup by FormRef; answers are remembered per runtime id (negative ones too:
// the dialogue menu asks about every INFO of an NPC each time it is built).
class InfoQuestScopes {
public:
    using Lookup = std::function<bool(uint32_t infoRuntimeId, fnvcoop::FormRef* quest)>;
    void set(Lookup l) {
        lookup_ = std::move(l);
        cache_.clear();
    }
    bool active() const { return static_cast<bool>(lookup_); }
    // True and the quest when INFO `id` has one.
    bool questOf(uint32_t id, fnvcoop::FormRef* quest) {
        if (!lookup_ || !id) return false;
#ifdef NVHOOKS_MUTANT
        return false;   // mutant: the INFO's conditions are never put under its quest (the Jed gate reduces as the initiator)
#else
        auto it = cache_.find(id);
        if (it == cache_.end()) {
            Entry e;
            e.has = lookup_(id, &e.quest);
            it = cache_.emplace(id, std::move(e)).first;
        }
        if (!it->second.has) return false;
        if (quest) *quest = it->second.quest;
        return true;
#endif
    }
    size_t cached() const { return cache_.size(); }
    void clear() { cache_.clear(); }

private:
    struct Entry {
        bool has = false;
        fnvcoop::FormRef quest;
    };
    Lookup lookup_;
    std::unordered_map<uint32_t, Entry> cache_;
};

}  // namespace nvhooks
