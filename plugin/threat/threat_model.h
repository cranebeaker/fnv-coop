// plugin/threat: the scoring (game-free, pure). Who an enemy should fight, among the players, from the damage each dealt to it (fading), how
// near each is, whether it can see them and whom it already fights; the hysteresis that keeps it from flickering; the spreading that keeps a group
// from picking one player; who is eligible at all; and the split of a health drop between the players who were aiming at the enemy.
#pragma once
#include <cstdint>
#include <map>
#include <vector>

#include "threat/threat_types.h"

namespace nvcoop::threat {

// What is remembered about one enemy.
struct Hurt {
    Slot slot = kNoSlot;
    float damage = 0;                      // the faded sum as of `at`
    uint64_t at = 0;
};
struct SightMemo {
    bool seen = false;
    uint64_t at = 0;                       // 0 = never checked
};
struct Record {
    std::vector<Hurt> hurts;
    Slot target = kNoSlot;                 // the player it fights (as last observed)
    uint64_t since = 0;                    // since when
    Slot assigned = kNoSlot;               // what we last told the engine (kNoSlot = never)
    uint64_t assignedAt = 0;
    uint32_t overrides = 0;                // the engine going back on what we assigned, within the window
    uint64_t overrideFrom = 0;
    uint64_t quietUntil = 0;               // left to the engine until
    uint64_t retryAt = 0;                  // a failed retarget is tried again after this
    uint64_t fedAt = 0;                    // the last time a hit feed reported damage on it
    uint64_t seenAt = 0;                   // the last tick it was an eligible fight
    float lastHealth = -1;
    std::map<Slot, SightMemo> sight;
};

// ---- damage ledger ----
float fade(float damage, uint64_t at, uint64_t now, uint32_t fadeMs);                       // linear to nothing at fadeMs
float damageOf(const Record& r, Slot slot, uint64_t now, const Config& cfg);
void addDamage(Record& r, Slot slot, float damage, uint64_t now, const Config& cfg);

// ---- who is aiming at it ----
struct Shooter {
    Slot slot = kNoSlot;
    float distance = 0;
    float aimCos = 0;                      // cosine of the angle between the player's facing and the direction to the enemy
    bool weaponOut = false;
};
float aimCosine(const Vec3& from, float yaw, const Vec3& to);   // XY only; yaw as the engine's rotZ (0 = +Y, clockwise)
// A health drop split between the shooters whose weapon is out, who are in range and whose aim is within the cone, in proportion to how well they aim.
std::vector<std::pair<Slot, float>> splitDamage(float drop, const std::vector<Shooter>& shooters, const Config& cfg);

// ---- eligibility ----
// Whether the director may touch the enemy, and which player it fights now (kNoSlot when none). Only a fight against a player is ever touched:
// a stray shot must not start a fight, a companion's fight is the companion's.
Skip eligibility(const EnemyView& e, const Config& cfg, const std::vector<PlayerView>& players, Slot& fighting);

// ---- the pick ----
struct Candidate {
    Slot slot = kNoSlot;
    float distance = 0;
    bool seen = false;
    bool engaged = false;                  // the enemy fights this player now
    int others = 0;                        // OTHER enemies of the group already on this player
    float score = 0;                       // filled by decide()
};
float baseScore(const Record& r, const Candidate& c, uint64_t now, const Config& cfg);   // damage + nearness + sight + stickiness
enum class Verdict : uint8_t {
    None,       // no player to pick
    Keep,       // stay (the best is the one it fights, or the best is not clearly ahead)
    Hold,       // another is ahead but the minimum time on a target is not up
    Switch,     // another is clearly ahead
    Fresh,      // it fights nobody it could (the target is gone / downed): the best one
};
struct Decision {
    Verdict verdict = Verdict::None;
    Slot to = kNoSlot;
    float incumbent = 0, best = 0;
};
Decision decide(const Record& r, std::vector<Candidate>& cands, uint64_t now, const Config& cfg);

}  // namespace nvcoop::threat
