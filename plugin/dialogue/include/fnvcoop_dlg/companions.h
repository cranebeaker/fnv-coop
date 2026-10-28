// Companion ownership + per-owner rewrite of the shared follower quest variables (L8). Game-free, header-only.
//
// Vanilla tracks followers in ONE quest (VNPCFollowers): bHumanoidInParty, bCritterInParty, bPlayerHasFollower,
// nCurrentFollowers and a Hired/Available/Fired triple per companion. With several players those shared values
// make player B think player A's companion is "in my party" (no recruit possible once anyone has a humanoid) and a
// recruit INFO's own `set ...bHumanoidInParty to 1` blocks everybody. Owner decisions (MASTER-PLAN):
//   * a companion is owned by the recruiting player; commands (wait, follow, dismiss, trade, wheel) are owner-only
//   * follower cap is per recruiting player (1 humanoid + 1 critter), party total configurable (default 4)
//
// Mechanism: every machine keeps this registry (filled from CompanionOwner effects; the host also validates
// recruits). Dialogue conditions are evaluated on the initiator's machine, so the engine hook for
// GetQuestVariable/GetScriptVariable calls viewVar(quest, idx, evaluatingSlot, realValue) and gets the value as seen
// by THAT player. The shared quest variable itself keeps the party AGGREGATE (aggregateVar) so host-side quest
// scripts that read it (waiting-companion logic, achievements) keep working.
#pragma once
#include <cstdint>
#include <cstdlib>
#include <istream>
#include <map>
#include <set>
#include <sstream>
#include <string>
#include <vector>
#include "fnvcoop/types.h"
#include "fnvcoop_dlg/mutations.h"

namespace fnvcoop_dlg {

using fnvcoop::FormRef;
using fnvcoop::PlayerId;

enum class CompanionKind : uint8_t { Humanoid = 0, Critter = 1 };

struct CompanionRules {
    uint8_t humanoidCap = 1;   // per recruiting player
    uint8_t critterCap = 1;    // per recruiting player
    uint8_t partyTotal = 4;    // across the whole party
};

enum class RecruitResult : uint8_t {
    Ok = 0,
    AlreadyYours = 1,      // idempotent: this player already owns the companion
    OwnedByOther = 2,
    OwnerCapHumanoid = 3,
    OwnerCapCritter = 4,
    PartyFull = 5,
    Unknown = 6,           // not a registered companion
};

// How a (quest, var index) is rewritten.
enum class VarKind : uint8_t {
    HumanoidInParty, CritterInParty, HasFollower, CurrentFollowers,   // per-owner aggregates
    Hired, Available, Fired,                                         // per companion (needs npc)
};

class CompanionRegistry {
public:
    explicit CompanionRegistry(CompanionRules r = {}) : rules_(r) {}

    const CompanionRules& rules() const { return rules_; }
    void setRules(const CompanionRules& r) { rules_ = r; }
    void define(const FormRef& npc, CompanionKind kind) { kinds_[canon(npc)] = kind; }
    // One companion known under several forms (the base NPC and every placed reference, ED-E's upgrade variants): refs[0] is the
    // canonical identity, every other form resolves to it, so ownership follows the companion whichever form an effect names.
    void defineGroup(const std::vector<FormRef>& refs, CompanionKind kind) {
        if (refs.empty()) return;
        for (const FormRef& r : refs) if (!mutations().noCompanionCanon) canon_[r] = refs[0];
        kinds_[refs[0]] = kind;
    }
    bool isCompanion(const FormRef& npc) const { return kinds_.count(canon(npc)) != 0; }
    FormRef canon(const FormRef& f) const {
        auto it = canon_.find(f);
        return it == canon_.end() ? f : it->second;
    }

    // ---- ownership
    PlayerId ownerOf(const FormRef& npcIn) const {
        auto it = owner_.find(canon(npcIn));
        return it == owner_.end() ? fnvcoop::kNoPlayer : it->second;
    }
    // A PARTY companion (ED-E in Lonesome Road: the rules engine's owner=party, RulesEngine::isPartyCompanion) belongs to the whole party: it uses nobody's slot,
    // so count() and partyCount() leave it out (the recruiter can still take Rex next to ED-E) and the Hired variable reads 1 for every slot. The feed is the
    // roster listener (Companion::party). The flag is a property of the NPC: it survives setOwner / dismiss.
    void setPartyOwned(const FormRef& npc, bool party) {
        if (party) party_.insert(canon(npc)); else party_.erase(canon(npc));
    }
    bool isPartyOwned(const FormRef& npc) const { return party_.count(canon(npc)) != 0; }

    int count(PlayerId slot, int kind = -1) const {
        int n = 0;
        for (auto& kv : owner_) {
            if (kv.second != slot) continue;
            if (isPartyOwned(kv.first) && !mutations().partyCompanionCounts) continue;   // uses nobody's slot
            auto k = kinds_.find(kv.first);
            if (kind < 0 || (k != kinds_.end() && int(k->second) == kind)) ++n;
        }
        return n;
    }
    int partyCount() const {
        int n = 0;
        for (auto& kv : owner_) if (!isPartyOwned(kv.first) || mutations().partyCompanionCounts) ++n;
        return n;
    }

    RecruitResult canRecruit(const FormRef& npcIn, PlayerId slot) const {
        const FormRef npc = canon(npcIn);
        auto k = kinds_.find(npc);
        if (k == kinds_.end()) return RecruitResult::Unknown;
        PlayerId o = ownerOf(npc);
        if (o == slot) return RecruitResult::AlreadyYours;
        if (o != fnvcoop::kNoPlayer) return RecruitResult::OwnedByOther;
        if (mutations().capIsPartyWide) {  // MUTANT: the old "2 companions total" rule
            if (count(0, 0) + count(1, 0) + count(2, 0) + count(3, 0) >= rules_.humanoidCap && k->second == CompanionKind::Humanoid)
                return RecruitResult::OwnerCapHumanoid;
        } else {
            if (k->second == CompanionKind::Humanoid && count(slot, 0) >= rules_.humanoidCap) return RecruitResult::OwnerCapHumanoid;
            if (k->second == CompanionKind::Critter && count(slot, 1) >= rules_.critterCap) return RecruitResult::OwnerCapCritter;
        }
        if (partyCount() >= rules_.partyTotal) return RecruitResult::PartyFull;
        return RecruitResult::Ok;
    }
    RecruitResult recruit(const FormRef& npcIn, PlayerId slot) {
        RecruitResult r = canRecruit(npcIn, slot);
        if (r == RecruitResult::Ok) owner_[canon(npcIn)] = slot;
        return r;
    }
    // Only the owner may dismiss (kNoPlayer = host/system override, e.g. companion death).
    bool dismiss(const FormRef& npcIn, PlayerId by) {
        auto it = owner_.find(canon(npcIn));
        if (it == owner_.end()) return false;
        if (by != fnvcoop::kNoPlayer && by != it->second && !mutations().anyoneDismisses) return false;
        owner_.erase(it);
        return true;
    }
    // Replicated state (CompanionOwner effect): force without cap checks. owner == kNoPlayer clears.
    void setOwner(const FormRef& npcIn, PlayerId owner) {
        const FormRef npc = canon(npcIn);
        if (owner == fnvcoop::kNoPlayer) owner_.erase(npc); else owner_[npc] = owner;
    }
    // The owner left the session: their companions are released (they walk home; vanilla "fired" state).
    std::vector<FormRef> releaseOwner(PlayerId slot) {
        std::vector<FormRef> freed;
        if (mutations().keepCompanionsOfLeaver) return freed;
        for (auto it = owner_.begin(); it != owner_.end();) {
            if (it->second == slot) { freed.push_back(it->first); it = owner_.erase(it); } else ++it;
        }
        return freed;
    }

    // ---- variable rules
    void addVarRule(const FormRef& quest, uint32_t idx, VarKind kind, const FormRef& npc = FormRef()) {
        rules_by_[{quest, idx}] = {kind, canon(npc)};
    }
    bool hasRule(const FormRef& quest, uint32_t idx) const { return rules_by_.count({quest, idx}) != 0; }
    // Every registered rule (W3a item 4: the session writes a player's view of them into the shared variables around that player's script run).
    template <class F> void forEachRule(F&& fn) const { for (const auto& kv : rules_by_) fn(kv.first.first, kv.first.second, kv.second.kind); }
    size_t ruleCount() const { return rules_by_.size(); }

    // The value of the variable as seen by `slot`. `real` is the shared (aggregate) value the engine holds.
    int32_t viewVar(const FormRef& quest, uint32_t idx, PlayerId slot, int32_t real) const {
        auto it = rules_by_.find({quest, idx});
        if (it == rules_by_.end() || mutations().noOwnerRewrite) return real;
        const Rule& r = it->second;
        switch (r.kind) {
            case VarKind::HumanoidInParty:
                return (count(slot, 0) >= rules_.humanoidCap || partyFull()) ? 1 : 0;   // party total reached: recruiting closed
            case VarKind::CritterInParty:
                return (count(slot, 1) >= rules_.critterCap || partyFull()) ? 1 : 0;
            case VarKind::HasFollower: return count(slot) > 0 ? 1 : 0;
            case VarKind::CurrentFollowers: return count(slot);
            case VarKind::Hired:
                if (isPartyOwned(r.npc) && ownerOf(r.npc) != fnvcoop::kNoPlayer && !mutations().partyHiredPerSlot) return 1;   // everybody sees the party companion as hired
                return ownerOf(r.npc) == slot ? 1 : 0;
            case VarKind::Available: {
                PlayerId o = ownerOf(r.npc);
                if (o != fnvcoop::kNoPlayer && o != slot && !isPartyOwned(r.npc)) return 0;   // already travelling with another courier
                return real;
            }
            case VarKind::Fired: return real;
        }
        return real;
    }
    // What the shared variable should hold: the party aggregate (any owner has one / total).
    int32_t aggregateVar(const FormRef& quest, uint32_t idx, int32_t real) const {
        auto it = rules_by_.find({quest, idx});
        if (it == rules_by_.end()) return real;
        const Rule& r = it->second;
        switch (r.kind) {
            case VarKind::HumanoidInParty: return anyKind(0) ? 1 : 0;
            case VarKind::CritterInParty: return anyKind(1) ? 1 : 0;
            case VarKind::HasFollower: return partyCount() > 0 ? 1 : 0;
            case VarKind::CurrentFollowers: return partyCount();
            case VarKind::Hired: return ownerOf(r.npc) != fnvcoop::kNoPlayer ? 1 : 0;
            case VarKind::Available: case VarKind::Fired: return real;
        }
        return real;
    }
    struct VarFix { FormRef quest; uint32_t idx; int32_t value; };
    // After CompanionOwner effects were applied: the SetScriptVar corrections that bring the shared variables back
    // in line with the registry (the recruit/dismiss INFO's own writes are per-origin, the registry is the truth).
    std::vector<VarFix> reconcile(const std::map<std::pair<FormRef, uint32_t>, int32_t>& realValues) const {
        std::vector<VarFix> out;
        for (auto& kv : rules_by_) {
            auto rv = realValues.find(kv.first);
            int32_t real = rv == realValues.end() ? 0 : rv->second;
            if (kv.second.kind == VarKind::Available || kv.second.kind == VarKind::Fired) continue;
            int32_t want = aggregateVar(kv.first.first, kv.first.second, real);
            if (rv == realValues.end() || want != real) out.push_back({kv.first.first, kv.first.second, want});
        }
        return out;
    }

private:
    struct Rule { VarKind kind; FormRef npc; };
    bool partyFull() const { return partyCount() >= rules_.partyTotal; }
    bool anyKind(int k) const {
        for (auto& kv : owner_) { auto it = kinds_.find(kv.first); if (it != kinds_.end() && int(it->second) == k) return true; }
        return false;
    }

    CompanionRules rules_;
    std::map<FormRef, FormRef> canon_;
    std::map<FormRef, CompanionKind> kinds_;
    std::map<FormRef, PlayerId> owner_;
    std::set<FormRef> party_;   // party companions (setPartyOwned)
    std::map<std::pair<FormRef, uint32_t>, Rule> rules_by_;
};

// VNPCFollowers (FalloutNV.esm:0B16D0) variable indices, verified against its quest script (tools/check_follower_vars.py).
namespace vnpc {
constexpr uint32_t kHumanoidInParty = 47, kCritterInParty = 48, kHasFollower = 18, kCurrentFollowers = 19;
struct Companion { const char* name; uint32_t hired, fired, available; CompanionKind kind; };
constexpr Companion kCompanions[] = {
    {"Boone", 6, 7, 8, CompanionKind::Humanoid},     {"Cass", 21, 22, 23, CompanionKind::Humanoid},
    {"Veronica", 27, 28, 29, CompanionKind::Humanoid}, {"Lily", 33, 34, 35, CompanionKind::Humanoid},
    {"EDE", 30, 31, 32, CompanionKind::Critter},     {"Raul", 9, 11, 10, CompanionKind::Humanoid},
    {"Arcade", 12, 14, 13, CompanionKind::Humanoid}, {"Rex", 15, 17, 16, CompanionKind::Critter},
};
}  // namespace vnpc

// Registers the VNPCFollowers rules; npcByName maps Boone/Cass/... to the companion NPC (base or ref) FormRef.
inline void registerVanillaFollowers(CompanionRegistry& reg, const FormRef& quest, const std::map<std::string, FormRef>& npcByName) {
    reg.addVarRule(quest, vnpc::kHumanoidInParty, VarKind::HumanoidInParty);
    reg.addVarRule(quest, vnpc::kCritterInParty, VarKind::CritterInParty);
    reg.addVarRule(quest, vnpc::kHasFollower, VarKind::HasFollower);
    reg.addVarRule(quest, vnpc::kCurrentFollowers, VarKind::CurrentFollowers);
    for (auto& c : vnpc::kCompanions) {
        auto it = npcByName.find(c.name);
        if (it == npcByName.end()) continue;
        reg.define(it->second, c.kind);
        reg.addVarRule(quest, c.hired, VarKind::Hired, it->second);
        reg.addVarRule(quest, c.fired, VarKind::Fired, it->second);
        reg.addVarRule(quest, c.available, VarKind::Available, it->second);
    }
}

// Follower quest rules (generated by tools/gen_follower_rules.py, data/follower_rules.txt):
//   quest <plugin>:<HEXID> <edid>
//   flag <humanoid|critter|hasFollower|current> <var index>
//   companion <name> <humanoid|critter> hired=<i> fired=<i> available=<i> refs=<plugin>:<HEXID>[,...]
// Registers the vanilla VNPCFollowers rules and the Dead Money / Honest Hearts follower quests in one registry (gap G33), so a second
// recruiter sees the recruit topics of their own party, not the first recruiter's.
struct FollowerRulesStats { size_t quests = 0, companions = 0, rules = 0, bad = 0; };
inline bool parseFollowerRef(const std::string& tok, FormRef& out) {
    size_t c = tok.rfind(':');
    if (c == std::string::npos || c == 0 || c + 1 >= tok.size()) return false;
    std::string plugin = tok.substr(0, c);
    for (auto& ch : plugin) ch = char(tolower(static_cast<unsigned char>(ch)));
    char* end = nullptr;
    unsigned long v = std::strtoul(tok.c_str() + c + 1, &end, 16);
    if (!end || *end) return false;
    out = FormRef(plugin, uint32_t(v));
    return true;
}
inline FollowerRulesStats loadFollowerRules(std::istream& in, CompanionRegistry& reg) {
    FollowerRulesStats st;
    FormRef quest;
    bool haveQuest = false;
    std::string line;
    while (std::getline(in, line)) {
        if (line.empty() || line[0] == '#') continue;
        std::istringstream ls(line);
        std::string kind;
        ls >> kind;
        if (kind == "quest") {
            std::string ref;
            haveQuest = (ls >> ref) && parseFollowerRef(ref, quest);
            if (haveQuest) ++st.quests; else ++st.bad;
        } else if (kind == "flag" && haveQuest) {
            std::string which;
            long idx = -1;
            if (!(ls >> which >> idx) || idx < 0) { ++st.bad; continue; }
            if (which != "humanoid" && which != "critter" && which != "hasFollower" && which != "current") { ++st.bad; continue; }
            VarKind vk = which == "humanoid" ? VarKind::HumanoidInParty : which == "critter" ? VarKind::CritterInParty
                       : which == "hasFollower" ? VarKind::HasFollower : VarKind::CurrentFollowers;
            reg.addVarRule(quest, uint32_t(idx), vk);
            ++st.rules;
        } else if (kind == "companion" && haveQuest) {
            std::string name, ck, tok;
            long hired = -1, fired = -1, avail = -1;
            std::vector<FormRef> refs;
            if (!(ls >> name >> ck) || (ck != "humanoid" && ck != "critter")) { ++st.bad; continue; }
            while (ls >> tok) {
                size_t eq = tok.find('=');
                if (eq == std::string::npos) continue;
                std::string k = tok.substr(0, eq), v = tok.substr(eq + 1);
                if (k == "hired") hired = std::atol(v.c_str());
                else if (k == "fired") fired = std::atol(v.c_str());
                else if (k == "available") avail = std::atol(v.c_str());
                else if (k == "refs") {
                    size_t pos = 0;
                    while (pos < v.size()) {
                        size_t comma = v.find(',', pos);
                        FormRef r;
                        if (parseFollowerRef(v.substr(pos, comma == std::string::npos ? std::string::npos : comma - pos), r)) refs.push_back(r);
                        pos = comma == std::string::npos ? v.size() : comma + 1;
                    }
                }
            }
            if (refs.empty()) { ++st.bad; continue; }
            reg.defineGroup(refs, ck == "critter" ? CompanionKind::Critter : CompanionKind::Humanoid);
            if (hired >= 0) { reg.addVarRule(quest, uint32_t(hired), VarKind::Hired, refs[0]); ++st.rules; }
            if (fired >= 0) { reg.addVarRule(quest, uint32_t(fired), VarKind::Fired, refs[0]); ++st.rules; }
            if (avail >= 0) { reg.addVarRule(quest, uint32_t(avail), VarKind::Available, refs[0]); ++st.rules; }
            ++st.companions;
        }
    }
    return st;
}

// Owner-only topic list (generated by tools/gen_companion_topics.py): lines "ownerOnly <plugin>:<HEXID> <name>".
inline std::set<FormRef> loadOwnerOnlyTopics(std::istream& in) {
    std::set<FormRef> s;
    std::string line;
    while (std::getline(in, line)) {
        std::istringstream ls(line);
        std::string kind, ref;
        if (!(ls >> kind >> ref) || kind != "ownerOnly") continue;
        size_t c = ref.rfind(':');
        if (c == std::string::npos) continue;
        std::string plugin = ref.substr(0, c);
        for (auto& ch : plugin) ch = char(tolower(static_cast<unsigned char>(ch)));
        s.insert(FormRef(plugin, uint32_t(std::stoul(ref.substr(c + 1), nullptr, 16))));
    }
    return s;
}

}  // namespace fnvcoop_dlg
