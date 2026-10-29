#include "rebalance/gmst_applier.h"

#include <cmath>

namespace nvcoop::rebalance {

const std::vector<GmstEntry>& gmstTable() {
    static const std::vector<GmstEntry> t = {
        {"fDiffMultHPToPCVE", GmstKind::DamageToPlayers, false}, {"fDiffMultHPToPCE", GmstKind::DamageToPlayers, false},
        {"fDiffMultHPToPCN", GmstKind::DamageToPlayers, false},  {"fDiffMultHPToPCH", GmstKind::DamageToPlayers, false},
        {"fDiffMultHPToPCVH", GmstKind::DamageToPlayers, false},
        {"fDiffMultHPByPCVE", GmstKind::DamageByPlayers, false}, {"fDiffMultHPByPCE", GmstKind::DamageByPlayers, false},
        {"fDiffMultHPByPCN", GmstKind::DamageByPlayers, false},  {"fDiffMultHPByPCH", GmstKind::DamageByPlayers, false},
        {"fDiffMultHPByPCVH", GmstKind::DamageByPlayers, false},
        {"fAVDCarryWeightsBase", GmstKind::CarryWeight, false},
        {"iLevelUpSkillPointsBase", GmstKind::SkillPoints, true},
        {"iXPLevelKillCreatureVeryEasy", GmstKind::KillXp, true}, {"iXPLevelKillCreatureEasy", GmstKind::KillXp, true},
        {"iXPLevelKillCreatureAverage", GmstKind::KillXp, true},  {"iXPLevelKillCreatureHard", GmstKind::KillXp, true},
        {"iXPLevelKillCreatureVeryHard", GmstKind::KillXp, true},
        {"iXPLevelKillNPCVeryEasy", GmstKind::KillXp, true},      {"iXPLevelKillNPCEasy", GmstKind::KillXp, true},
        {"iXPLevelKillNPCAverage", GmstKind::KillXp, true},       {"iXPLevelKillNPCHard", GmstKind::KillXp, true},
        {"iXPLevelKillNPCVeryHard", GmstKind::KillXp, true},
    };
    return t;
}

float gmstValue(const GmstEntry& e, float original, const Profile& p) {
    float v = original;
    switch (e.kind) {
        case GmstKind::DamageToPlayers: v = original * p.damageToPlayers; break;
        case GmstKind::DamageByPlayers: v = original * p.damageByPlayers; break;
        case GmstKind::KillXp: v = original * p.killXp; break;
        case GmstKind::CarryWeight: v = flatValue(original, p.carryWeightBase, p.blend); break;
        case GmstKind::SkillPoints: v = flatValue(original, p.skillPointsBase, p.blend); break;
    }
    if (e.integer) v = std::round(v);
    return v;
}

bool GmstApplier::apply(const Profile& p) {
    if (p.vanilla() && written_.empty()) return true;   // the game's own numbers, never touched: nothing to read or write
    bool allRead = true;
    for (const GmstEntry& e : gmstTable()) {
        const std::string name = e.name;
        auto it = originals_.find(name);
        if (it == originals_.end()) {
            float v = 0;
            if (!g_.read(name, v)) { ++readsFailed_; allRead = false; continue; }
            it = originals_.emplace(name, v).first;
        }
        float want = gmstValue(e, it->second, p);
        if (FNV_MUT(1230)) want = gmstValue(e, lastWritten(name), p);   // MUTANT 1230: computed from the current value, so it compounds
        const float last = lastWritten(name);
        if (std::fabs(want - last) < (e.integer ? 0.5f : 0.001f) && !(FNV_MUT(1231))) continue;   // MUTANT 1231: written every time
        if (g_.write(name, want)) {
            written_[name] = want;
            ++writes_;
        } else {
            allRead = false;
        }
    }
    return allRead;
}

}  // namespace nvcoop::rebalance
