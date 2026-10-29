// Co-op balanced: the numbers, and what they become for a party of a given size (docs/rebalance.md). Game-free.
//
// The base numbers are the ones the other co-op mod uses for a balanced group (damage to players, damage by players, carry weight, level-up skill points, kill
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
    float suppliesKept = 0.5f;         // food, drink and medicine enemies and boxes give: this share as often as the game's
    float bodyAmmoKept = 0.5f;         // a generic enemy that dies keeps this share of its ammo
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
    bool operator==(const Profile& o) const {
        return blend == o.blend && damageToPlayers == o.damageToPlayers && damageByPlayers == o.damageByPlayers && killXp == o.killXp &&
               carryWeightBase == o.carryWeightBase && skillPointsBase == o.skillPointsBase && suppliesKept == o.suppliesKept && bodyAmmoKept == o.bodyAmmoKept;
    }
    bool vanilla() const { return *this == Profile(); }
};

// The profile of a campaign for a party of `players` (the game's own for Normal, an unchosen campaign, and no players known yet).
Profile profileFor(const CampaignSettings& s, int players);

// Linear mix of two profiles (t 0..1), for the smooth change.
Profile mix(const Profile& a, const Profile& b, float t);

// The value of a flat number: the game's own for blend 0, lerp for blend in between.
inline float flatValue(float original, float target, float blend) { return original + (target - original) * blend; }

}  // namespace nvcoop::rebalance
