// Session side of the party-aware conditions (campaign gaps G23 / G10 / G42 call sites). Game-free and header-only.
//
// plugin/hooks evaluates a condition once per party member, but a remote player has only a ghost / puppet reference: the engine cannot tell
// what perks, notes, region or equipment that player has. The session knows (shared shadow state, the avatar layer), so it answers through
// nvhooks::StateProvider. Three pieces:
//   makePartyStateProvider(source)  StateQuery -> value, one rule per CondFunc (below). false = unknown: the hooks skip that member.
//   presentMembers(rt, ghostOf)     the members the evaluators loop over: the local player (0x14) + every joined guest whose ghost / puppet exists.
//   makeSessionReducer(rules)       RulesEngine::reduceCondition behind nvhooks' adapter; with no rules engine (a guest, rules off) the initiator's own
//                                   value stands, which is what vanilla would have produced.
// coop_module.cpp installs all three on the host (nvhooks::setPartyConditions + setPartyStateProvider).
#pragma once
#include <functional>
#include <map>
#include <vector>

#include "coop_runtime.h"
#include "fnvcoop_mut.h"
#include "party_cond.h"
#include "rules_reducer.h"
#include "slot_facts.h"

namespace nvcoop::coop {

struct PartyStateSource {
    std::function<bool(uint32_t runtimeId, fnvcoop::FormRef* out)> toRef;                                           // engine refID / formID -> FormRef
    std::function<bool(uint8_t slot, const fnvcoop::FormRef& perk, int32_t* rank)> perkRank;                         // the slot's SetPerk ledger
    std::function<bool(const fnvcoop::FormRef& note)> hasNote;                                                       // the shared journal
    std::function<int(const fnvcoop::FormRef& npc)> companionOwner;                                                  // -1 = unknown
    std::function<bool(uint8_t slot, uint32_t equippedFormID, double* value)> equipped;                              // avatar layer (optional)
    std::function<bool(uint8_t slot, double* value)> sneaking;                                                       // avatar layer (optional)
    std::function<bool(uint8_t slot, uint32_t regionFormID, double* value)> inRegion;                                // member position vs the region (optional)
    // wave 1b: the player's own level, sex, seat and action. bindSlotFacts() fills them (and the three above) from the avatar layer (W3a item 3); empty = the member is skipped.
    std::function<bool(uint8_t slot, double* level)> level;
    std::function<bool(uint8_t slot, int* sex)> sex;                                                                 // 0 male / 1 female
    std::function<bool(uint8_t slot, double* seated)> sitting;
    std::function<bool(uint8_t slot, int32_t actionId, double* active)> actionActive;
    // guest pack facts: what the guest carries, from its Pack message (bindSlotPack). false = unknown: the ghost (puppet) answers as before.
    std::function<bool(uint8_t slot, uint32_t itemFormID, double* count)> itemCount;                                 // GetItemCount of a watched item form
    std::function<bool(uint8_t slot, double* weight)> inventoryWeight;                                              // GetActorValue InventoryWeight
};

// The actor value code of InventoryWeight (the Honest Hearts Jed gate reads it); the provider answers GetActorValue for this one only.
constexpr int32_t kAvInventoryWeightCode = 46;

// Guest pack facts: the host's pack-aware answers for a remote player (plugin/coop/live/avatar_net.h packCount / packWeight through live_glue). Without them the puppet's own
// pack answers, which is empty (twin 960: not connected).
using SlotPackCountFn = std::function<bool(uint8_t slot, uint32_t item, int64_t* count)>;
using SlotPackWeightFn = std::function<bool(uint8_t slot, double* weight)>;
inline void bindSlotPack(PartyStateSource& s, SlotPackCountFn count, SlotPackWeightFn weight) {
    if (FNV_MUT(960)) return;   // MUTANT 960: the pack facts are not connected (a guest's GetItemCount / InventoryWeight stay the puppet's)
    if (count)
        s.itemCount = [count](uint8_t slot, uint32_t item, double* v) {
            int64_t n = 0;
            if (!count(slot, item, &n)) return false;
            *v = static_cast<double>(n);
            return true;
        };
    if (weight) s.inventoryWeight = std::move(weight);
}

// W3a item 3: the avatar-layer groups of the source (equipped, sneaking, region, level, sex, sitting, action) from ONE lookup of what the host knows about a remote player
// (SlotFacts: Look + snapshot + Facts message). A group the player has not delivered yet answers "unknown" (the member is skipped), never a guess.
using SlotFactsFn = std::function<bool(uint8_t slot, SlotFacts* out)>;
inline void bindSlotFacts(PartyStateSource& s, SlotFactsFn facts) {
    if (!facts || FNV_MUT(36)) return;   // MUTANT 36: the avatar layer is not connected (GetEquipped, IsSneaking, the region, level, sex, seat and action of a guest stay unknown)
    s.equipped = [facts](uint8_t slot, uint32_t item, double* v) {
        SlotFacts f;
        if (!facts(slot, &f) || !f.haveEquip) return false;
        bool worn = false;
        for (uint32_t e : f.equipped) worn = worn || e == item;
        *v = worn ? 1.0 : 0.0;
        return true;
    };
    s.sneaking = [facts](uint8_t slot, double* v) {
        SlotFacts f;
        if (!facts(slot, &f) || !f.haveSneak) return false;
        *v = f.sneaking ? 1.0 : 0.0;
        return true;
    };
    s.inRegion = [facts](uint8_t slot, uint32_t region, double* v) {
        SlotFacts f;
        if (!facts(slot, &f) || !f.haveFacts) return false;
        bool in = false;
        for (uint32_t r : f.regions) in = in || r == region;
        *v = in ? 1.0 : 0.0;
        return true;
    };
    s.level = [facts](uint8_t slot, double* v) {
        SlotFacts f;
        if (!facts(slot, &f) || !f.haveFacts || f.level == 0) return false;
        *v = f.level;
        return true;
    };
    s.sex = [facts](uint8_t slot, int* v) {
        SlotFacts f;
        if (!facts(slot, &f) || !f.haveSex) return false;
        *v = f.sex;
        return true;
    };
    s.sitting = [facts](uint8_t slot, double* v) {
        SlotFacts f;
        if (!facts(slot, &f) || !f.haveFacts) return false;
        *v = f.sitting;
        return true;
    };
    s.actionActive = [facts](uint8_t slot, int32_t id, double* v) {
        SlotFacts f;
        if (!facts(slot, &f) || !f.haveFacts || id < 0 || id >= 32) return false;   // only ids 0..31 are sampled by the guest
        *v = (f.actions >> id) & 1u ? 1.0 : 0.0;
        return true;
    };
}

// The source backed by a running host Runtime; the avatar-layer callbacks and the id converter come from the module.
inline PartyStateSource sourceFromRuntime(Runtime& rt, std::function<bool(uint32_t, fnvcoop::FormRef*)> toRef) {
    PartyStateSource s;
    s.toRef = std::move(toRef);
    s.perkRank = [&rt](uint8_t slot, const fnvcoop::FormRef& perk, int32_t* rank) { return rt.slotPerkRank(slot, perk, rank); };
    s.hasNote = [&rt](const fnvcoop::FormRef& n) { return rt.sharedJournalHas(n); };
    s.companionOwner = [&rt](const fnvcoop::FormRef& npc) { return rt.companionOwnerOf(npc); };
    return s;
}

inline nvhooks::StateProvider makePartyStateProvider(PartyStateSource src) {
    return [src = std::move(src)](const nvhooks::StateQuery& q, double* value) -> bool {
        // MUTANT 13: every question is answered for the host (slot 0), whoever is asked
        const uint8_t slot = FNV_MUT(13) ? uint8_t(0) : q.slot;
        fnvcoop::FormRef f;
        switch (q.f) {
            case nvhooks::CondFunc::HasPerk: {
                int32_t rank = 0;
                if (!src.toRef || !q.formID || !src.toRef(q.formID, &f) || !src.perkRank || !src.perkRank(slot, f, &rank)) return false;
                *value = rank > 0 ? 1.0 : 0.0;
                return true;
            }
            case nvhooks::CondFunc::GetHasNote: {
                if (!src.toRef || !q.formID || !src.toRef(q.formID, &f) || !src.hasNote) return false;
                *value = src.hasNote(f) ? 1.0 : 0.0;
                return true;
            }
            case nvhooks::CondFunc::GetPlayerTeammate: {
                // thisRefID is the NPC. "does THIS member own it": unknown owner = no answer, the engine flag alone stands (party_cond.h MemberView::Owner)
                if (!src.toRef || !q.thisRefID || !src.toRef(q.thisRefID, &f) || !src.companionOwner) return false;
                const int owner = src.companionOwner(f);
                if (owner < 0) return false;
                *value = owner == static_cast<int>(slot) ? 1.0 : 0.0;
                return true;
            }
            case nvhooks::CondFunc::GetEquipped:
                return src.equipped && q.formID && src.equipped(slot, q.formID, value);
            case nvhooks::CondFunc::IsSneaking:
                return src.sneaking && src.sneaking(slot, value);
            case nvhooks::CondFunc::IsPlayerInRegion:
                return src.inRegion && q.formID && src.inRegion(slot, q.formID, value);
            case nvhooks::CondFunc::GetLevel:
                return src.level && src.level(slot, value);
            case nvhooks::CondFunc::GetIsSex:
            case nvhooks::CondFunc::GetPCIsSex: {   // arg1 = 0 male / 1 female: 1 when the member's sex equals it
                int sex = 0;
                if (!src.sex || !src.sex(slot, &sex)) return false;
                *value = sex == q.arg1 ? 1.0 : 0.0;
                return true;
            }
            case nvhooks::CondFunc::GetItemCount:
                return src.itemCount && q.formID && src.itemCount(slot, q.formID, value);
            case nvhooks::CondFunc::GetActorValue:   // only InventoryWeight is known for a guest; every other actor value is the puppet's
                return (q.arg1 == kAvInventoryWeightCode || FNV_MUT(961)) && src.inventoryWeight && src.inventoryWeight(slot, value);   // MUTANT 961: the weight answers for every actor value
            case nvhooks::CondFunc::GetSitting:
                return src.sitting && src.sitting(slot, value);
            case nvhooks::CondFunc::IsPlayerActionActive:
                return src.actionActive && src.actionActive(slot, q.arg1, value);
            default:
                return false;
        }
    };
}

// The members to loop over: slot 0 is the local player (0x14); a guest counts once its ghost / puppet reference exists (0 = not placed yet: skipped).
inline std::vector<nvhooks::PartyMember> presentMembers(const Runtime& rt, const std::function<uint32_t(fnvcoop::PlayerId)>& ghostOf) {
    std::vector<nvhooks::PartyMember> m;
    m.push_back({0, nvhooks::kHostPlayerRef});
    if (FNV_MUT(14)) return m;   // MUTANT 14: the party is the host alone (every condition reads the host's player only)
    for (fnvcoop::PlayerId s : rt.joinedSlots()) {
        const uint32_t id = ghostOf ? ghostOf(s) : 0u;
        if (id) m.push_back({static_cast<uint8_t>(s), id});
    }
    return m;
}

// Tells nvhooks which engine reference stands for which slot (registerPlayerSlot), so `player` inside a script that runs for a guest, a player kill or a
// stash move resolves to that guest. The slot table never forgets, so a load clears it (clearPlayerSlots) and reset() makes the next sync() register again.
class SlotRegistrar {
public:
    using Register = std::function<void(uint32_t ref, uint8_t slot)>;
    void sync(const Runtime& rt, const std::function<uint32_t(fnvcoop::PlayerId)>& ghostOf, const Register& reg) {
        for (fnvcoop::PlayerId s : rt.joinedSlots()) {
            const uint32_t id = ghostOf ? ghostOf(s) : 0u;
            if (!id) continue;
            auto it = last_.find(s);
            if (it != last_.end() && it->second == id) continue;
            last_[s] = id;
            if (reg) reg(id, static_cast<uint8_t>(s));
        }
    }
    void reset() { if (!FNV_MUT(18)) last_.clear(); }   // MUTANT 18: after a load the puppets are not registered again (acting-player lookups fall back to the host)
    size_t known() const { return last_.size(); }

private:
    std::map<fnvcoop::PlayerId, uint32_t> last_;
};

inline nvhooks::ConditionReducer makeSessionReducer(std::function<fnvcoop::RulesEngine*()> rules) {
    return [rules = std::move(rules)](const fnvcoop::FormRef& quest, const std::string& func, const std::vector<nvhooks::CondValue>& vals, uint8_t initiator,
                                      fnvcoop::CondPhase phase) -> double {
        fnvcoop::RulesEngine* r = rules ? rules() : nullptr;
        if (r && !FNV_MUT(15)) return nvhooks::makeRulesReducer(*r)(quest, func, vals, initiator, phase);   // MUTANT 15: the rules engine is never asked
        for (const auto& v : vals) if (v.slot == initiator) return v.value;
        return vals.empty() ? 0.0 : vals.front().value;
    };
}

}  // namespace nvcoop::coop
