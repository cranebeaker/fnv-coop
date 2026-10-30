// Co-op balanced: the numbers, and what they become for a party of a given size (docs/rebalance.md). Game-free.
//
// The base numbers are for a balanced group (damage to players, damage by players, carry weight, level-up skill points, kill
// XP, how much food and medicine enemies drop, the reinforcement rule). On top, this module scales them by the party size: the tuning point is two players;
// each player past two makes enemies tougher (they take less from every shot), makes fights bigger (reinforcements) and pays less XP for each kill (there are more
// kills to make). One or two players get the base numbers.
#pragma once
#include <cstdint>

#include "rebalance/campaign_settings.h"

namespace nvcoop::rebalance {

constexpr int kMaxScaledPlayers = 8;   // a party larger than this is scaled as this many
constexpr int kTunedFor = 2;           // the party size the base numbers are meant for

struct Balance {
    float damageToPlayers = 2.0f;      // damage players take, times (the same as halving player health)
    float damageByPlayers = 1.25f;     // damage players deal, times, at the tuning point
    float carryWeightBase = 75.0f;     // base carry weight (the game's 150)
    float skillPointsBase = 8.0f;      // skill points a level before Intelligence (the game's 11)
    float killXpMult = 0.75f;          // kill XP, times, at the tuning point
    float suppliesKept = 0.5f;         // food, drink and medicine in boxes and on bodies: this share of what the game puts there
    float bodyAmmoKept = 0.5f;         // a generic enemy that dies keeps this share of its ammo
    float headMult = 1.5f, limbMult = 0.85f;  // a hit on the head / an arm or leg does this times the game's (the other mod: head 3x instead of 2x, limbs a little less)
    float mediumArmorDr = 10, heavyArmorDr = 20;   // Damage Resistance added by medium / heavy body armour, in percent (the other mod: armour matters more)
    float stimpakHealMult = 2.0f / 3.0f;     // Stimpak and Super Stimpak heal this share of the game's amount (the other mod: 20 in all for the game's 30, 40 for 60)
    float toughnessPerExtraPlayer = 0.25f;   // enemies take this much less from every shot per player past the tuning point (1 + 0.25 x extra)
    float extrasPerExtraPlayer = 0.25f;      // reinforcements: extra enemies per enemy that starts a fight, per player past the tuning point
    int maxExtrasAlive = 8;                  // reinforcements alive at once, at most
};

const Balance& coopBalance();

inline int clampPlayers(int players) { return players < 1 ? 1 : players > kMaxScaledPlayers ? kMaxScaledPlayers : players; }
inline int extraPlayers(int players) { const int n = clampPlayers(players); return n > kTunedFor ? n - kTunedFor : 0; }

// Extra enemies for each one that starts a fight (0 for one or two players, 0.25 for three, 0.5 for four ...).
float extrasPerEnemy(int players);
// How many times tougher enemies are than at the tuning point (1 for one or two players).
float toughness(int players);
// The share of a resource (supplies, body ammo, Stimpak healing) that stays at this party size: 1 for one or two players, 1 / (1 + extras per enemy) past that.
// A bigger party brings more enemies and so more bodies and boxes to loot, and every player carries their own Stimpaks, so what each one is worth shrinks the
// same way kill XP does.
float resourceShare(int players);

// What the game's numbers become. Multipliers are of the game's own value; the flat numbers are absolute and mixed with the game's own by `blend`
// (0 = the game's, 1 = balanced), so that going from Normal to balanced (or back) is one smooth change of every field.
struct Profile {
    float blend = 0;               // 0: the game's own numbers
    float damageToPlayers = 1;     // x
    float damageByPlayers = 1;     // x
    float killXp = 1;              // x
    float carryWeightBase = 0;     // absolute, used when blend > 0
    float skillPointsBase = 0;     // absolute, used when blend > 0
    float suppliesKept = 1;        // share
    float bodyAmmoKept = 1;        // share
    float stimpakHeal = 1;         // x the amount a Stimpak heals
    float headMult = 1, limbMult = 1;          // x a hit's damage on the head / an arm or leg
    float mediumArmorDr = 0, heavyArmorDr = 0; // percent taken off a hit on a victim in medium / heavy body armour
    bool operator==(const Profile& o) const {
        return blend == o.blend && damageToPlayers == o.damageToPlayers && damageByPlayers == o.damageByPlayers && killXp == o.killXp &&
               carryWeightBase == o.carryWeightBase && skillPointsBase == o.skillPointsBase && suppliesKept == o.suppliesKept && bodyAmmoKept == o.bodyAmmoKept &&
               stimpakHeal == o.stimpakHeal && headMult == o.headMult && limbMult == o.limbMult && mediumArmorDr == o.mediumArmorDr && heavyArmorDr == o.heavyArmorDr;
    }
    bool vanilla() const { return *this == Profile(); }
};

// The profile of a campaign for a party of `players` (the game's own for Normal, an unchosen campaign, and no players known yet).
Profile profileFor(const CampaignSettings& s, int players);

// Where a hit lands (combat's limb order: 0 torso, 1 head, 2..5 arms and legs) and what the victim wears: the factor the attacker's engine damage is
// multiplied by. 1 for the game's own profile. Applied once, by the machine whose engine computed the hit (plugin/combat's hit gate).
enum class ArmorClass : uint8_t { None = 0, Light, Medium, Heavy };
float bodyHitFactor(const Profile& p, uint8_t limb, ArmorClass armor);

// Linear mix of two profiles (t 0..1), for the smooth change.
Profile mix(const Profile& a, const Profile& b, float t);

// The value of a flat number: the game's own for blend 0, lerp for blend in between.
inline float flatValue(float original, float target, float blend) { return original + (target - original) * blend; }

}  // namespace nvcoop::rebalance
