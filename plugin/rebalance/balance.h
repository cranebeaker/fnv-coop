// Co-op balanced: the numbers, and what they become for a party of a given size (docs/rebalance.md). Game-free.
//
// Everything here comes from one small model of "N players share one world" (derived in docs/rebalance.md section 2; read it before changing a number):
//   * N players put more guns on every enemy: the party kills `focus(N) = 1 + focusPerExtraPlayer x (N - 1)` times as fast as one player (not N times: some
//     shots overkill, some players are reloading or walking, some target different enemies).
//   * Enemies are made `toughness(N)` times harder to kill (every shot takes less off), and a share of extra enemies joins each fight, so a fight is bigger
//     and a little longer, never a wall of health.
//   * Enemy damage is spread over N players, so each player takes about two thirds of what a lone player takes per fight (`loadPerPlayer`). The Stimpak is
//     made that much weaker so the number of Stimpaks a player burns per fight stays what it is alone.
//   * The extra enemies are bodies with a kit and a kill reward; the world's loot and XP per fight must not grow with the party, so those rewards shrink by
//     the same share (`rewardShare`).
// One player gets the game's own difficulty (every factor is 1); each player added moves the numbers by a fixed step.
#pragma once
#include <cstdint>

#include "rebalance/campaign_settings.h"

namespace nvcoop::rebalance {

constexpr int kMaxScaledPlayers = 8;   // a party larger than this is scaled as this many (the coop runtime's own party limit is 4; this leaves room)
constexpr int kSoloParty = 1;          // the party size at which Co-op balanced is the game's own difficulty

struct Balance {
    // How much more the party kills per second than one player does, per player past the first. Less than 1: with several players some shots are wasted on a
    // dying enemy, some players reload or walk while others shoot, and the party seldom all aims at the same target.
    float focusPerExtraPlayer = 0.7f;
    // Enemy toughness: `1 + this x extra players`. Chosen with the two numbers below so a party-sized fight lasts about 6 % (2 players) to 29 % (4 players)
    // longer than a solo fight and costs each player about two thirds of the damage (table in docs/rebalance.md).
    float toughnessPerExtraPlayer = 0.5f;
    float extrasPerExtraPlayer = 0.2f;     // reinforcements: extra enemies per enemy that starts a fight, per player past the first (2 players: one in five)
    int extrasCapPerExtraPlayer = 2;       // reinforcements alive at once: this many per player past the first (a cap for the actor budget and the sync traffic)
    // Damage players take: `1 + this x extra players`. Teammates end fights sooner, so without this a hit would matter less the bigger the party; the small step
    // keeps one hit worth about the same fraction of a player's health.
    float damageToPlayersPerExtraPlayer = 0.1f;
    // A head hit gets back `toughness ^ this` of the damage the uniform toughness took off, so aiming stays the way to shorten a long fight. 0.5: half of the
    // loss in log terms (a head hit then does 1/sqrt(toughness) of the game's damage where a body hit does 1/toughness).
    float headRegainExponent = 0.5f;
    float limbMult = 1.0f;                 // arms and legs: the game's own (a limb hit already pays by crippling the limb; the party model gives no reason to change it)
    // Extra Damage Resistance (percent off a hit) for the victim's armour weight, only in a party (two or more). Role argument: enemies are pulled toward
    // whoever hurts them or stands near them, so a player who wears heavy armour to hold attention should be paid for it. A tank draws about 1.5 times an even
    // share of the hits; taking a third off would cancel that exactly, 15 % cancels about half of it. Medium armour is half of heavy. Light armour gets nothing.
    float mediumArmorDr = 7, heavyArmorDr = 15;
};

const Balance& coopBalance();

inline int clampPlayers(int players) { return players < 1 ? 1 : players > kMaxScaledPlayers ? kMaxScaledPlayers : players; }
inline int extraPlayers(int players) { const int n = clampPlayers(players); return n > kSoloParty ? n - kSoloParty : 0; }

// How many times as fast as one player the party kills (1 for one player).
float focus(int players);
// Extra enemies for each one that starts a fight (0 for one player, 0.2 for two, 0.4 for three, 0.6 for four).
float extrasPerEnemy(int players);
// How many times tougher enemies are than in the game (1 for one player): damage players deal is divided by this.
float toughness(int players);
// The most reinforcements alive at once (0 for one player).
int maxExtras(int players);
// How many times the game's damage a player takes (1 for one player).
float damageToPlayersFactor(int players);
// The share of a reward that stays at this party size: 1 / (1 + extras per enemy). Extra enemies are extra bodies with kits and XP, so what a fight pays in
// loot and kill XP is divided by the same number the fight grew by.
float rewardShare(int players);
// How long a fight lasts compared with a lone player's: (1 + extras) x toughness / focus.
float fightLength(int players);
// What each player takes per fight compared with a lone player: damage factor x (1 + extras)^2 x toughness / (players x focus). The square is the usual
// result for a fight that is won one enemy at a time: the group is (1 + extras) times as many and each lives (1 + extras) times as long. It assumes the
// threat module spreads the hits evenly; a real player at the front takes more, one at the back less.
float loadPerPlayer(int players);

// What the game's numbers become. Multipliers are of the game's own value.
struct Profile {
    float blend = 0;               // 0: the game's own numbers, 1: Co-op balanced (the smooth change moves it)
    float damageToPlayers = 1;     // x
    float damageByPlayers = 1;     // x
    float killXp = 1;              // x
    float suppliesKept = 1;        // share
    float bodyAmmoKept = 1;        // share
    float stimpakHeal = 1;         // x the amount a Stimpak heals
    float headMult = 1, limbMult = 1;          // x a hit's damage on the head / an arm or leg
    float mediumArmorDr = 0, heavyArmorDr = 0; // percent taken off a hit on a victim in medium / heavy body armour
    bool operator==(const Profile& o) const {
        return blend == o.blend && damageToPlayers == o.damageToPlayers && damageByPlayers == o.damageByPlayers && killXp == o.killXp &&
               suppliesKept == o.suppliesKept && bodyAmmoKept == o.bodyAmmoKept &&
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

}  // namespace nvcoop::rebalance
