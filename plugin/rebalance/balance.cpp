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

float resourceShare(int players) {
    if (FNV_MUT(1474)) return 1.0f;   // MUTANT 1474: a bigger party loots and heals as much per body, box and Stimpak as two players
    return 1.0f / (1.0f + extrasPerEnemy(players));
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
    const float share = resourceShare(players);
    p.suppliesKept = b.suppliesKept * share;
    p.bodyAmmoKept = b.bodyAmmoKept * share;
    p.stimpakHeal = b.stimpakHealMult * (FNV_MUT(1475) ? 1.0f : share);   // MUTANT 1475: a Stimpak heals the same for any party size
    p.headMult = b.headMult; p.limbMult = b.limbMult;
    p.mediumArmorDr = b.mediumArmorDr; p.heavyArmorDr = b.heavyArmorDr;
    return p;
}

float bodyHitFactor(const Profile& p, uint8_t limb, ArmorClass armor) {
    float f = 1;
    if (limb == 1 && !FNV_MUT(1495)) f = p.headMult;          // MUTANT 1495: a head shot is a body shot
    else if (limb >= 2 && limb <= 5) f = p.limbMult;
    float dr = 0;
    if (armor == ArmorClass::Medium) dr = p.mediumArmorDr;
    else if (armor == ArmorClass::Heavy && !FNV_MUT(1496)) dr = p.heavyArmorDr;   // MUTANT 1496: heavy armour gets nothing extra
    if (dr > 90) dr = 90;
    return f * (1.0f - dr / 100.0f);
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
    p.stimpakHeal = l(a.stimpakHeal, b.stimpakHeal);
    p.headMult = l(a.headMult, b.headMult); p.limbMult = l(a.limbMult, b.limbMult);
    p.mediumArmorDr = l(a.mediumArmorDr, b.mediumArmorDr); p.heavyArmorDr = l(a.heavyArmorDr, b.heavyArmorDr);
    return p;
}

}  // namespace nvcoop::rebalance
