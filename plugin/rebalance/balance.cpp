#include "rebalance/balance.h"

namespace nvcoop::rebalance {

const Balance& coopBalance() {
    static const Balance b;
    return b;
}

float extrasPerEnemy(int players) {
    float per = coopBalance().extrasPerExtraPlayer;
    if (FNV_MUT(1210)) per = 0.0f;   // MUTANT 1210: a bigger party brings no reinforcements
    return per * static_cast<float>(extraPlayers(players));
}

float toughness(int players) {
    if (FNV_MUT(1211)) return 1.0f;   // MUTANT 1211: enemies are as tough for four players as for two
    return 1.0f + coopBalance().toughnessPerExtraPlayer * static_cast<float>(extraPlayers(players));
}

Profile profileFor(const CampaignSettings& s, int players) {
    Profile p;
    if (!s.balanced() || FNV_MUT(1212)) return p;   // MUTANT 1212: Normal gets the balanced numbers too
    const Balance& b = coopBalance();
    p.blend = 1;
    p.damageToPlayers = b.damageToPlayers;
    p.damageByPlayers = b.damageByPlayers / toughness(players);
    p.killXp = b.killXpMult / (1.0f + extrasPerEnemy(players));
    p.carryWeightBase = b.carryWeightBase;
    p.skillPointsBase = b.skillPointsBase;
    p.suppliesKept = b.suppliesKept;
    p.bodyAmmoKept = b.bodyAmmoKept;
    return p;
}

Profile mix(const Profile& a, const Profile& b, float t) {
    if (t <= 0) return a;
    if (t >= 1) return b;
    auto l = [t](float x, float y) { return x + (y - x) * t; };
    Profile p;
    p.blend = l(a.blend, b.blend);
    p.damageToPlayers = l(a.damageToPlayers, b.damageToPlayers);
    p.damageByPlayers = l(a.damageByPlayers, b.damageByPlayers);
    p.killXp = l(a.killXp, b.killXp);
    // absolute numbers: a vanilla side has none, so the other side's value is used the whole way (the blend carries the change)
    p.carryWeightBase = a.blend > 0 && b.blend > 0 ? l(a.carryWeightBase, b.carryWeightBase) : (b.blend > 0 ? b.carryWeightBase : a.carryWeightBase);
    p.skillPointsBase = a.blend > 0 && b.blend > 0 ? l(a.skillPointsBase, b.skillPointsBase) : (b.blend > 0 ? b.skillPointsBase : a.skillPointsBase);
    p.suppliesKept = l(a.suppliesKept, b.suppliesKept);
    p.bodyAmmoKept = l(a.bodyAmmoKept, b.bodyAmmoKept);
    return p;
}

}  // namespace nvcoop::rebalance
