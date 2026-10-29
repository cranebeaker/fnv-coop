// plugin/threat: enemies split their attention between the players (docs/threat.md). Game-free types: the settings, what the
// director sees of the world (players, enemies), and the one port it acts through.
//
// Vocabulary: an ENEMY is an actor whose AI runs on THIS machine (the machine is its owner) and that is fighting a player. A PLAYER is
// the local player or the puppet of a remote one (plugin/avatar): on every machine every player is a reference, so an enemy's combat
// target "is" a player when that reference is one of those. Slots are the party slots (the local one included).
#pragma once
#include <cstdint>
#include <string>
#include <vector>

namespace nvcoop::threat {

using Slot = uint8_t;
using RefId = uint32_t;
constexpr Slot kNoSlot = 0xFF;

struct Vec3 {
    float x = 0, y = 0, z = 0;
};

// All the knobs (nvcoop.ini [threat], see docs/threat.md). Scores are in "points of damage": a player who hurt the enemy for 30
// within the fade time is worth 30, a player standing next to it is worth `nearBonus`.
struct Config {
    bool enabled = true;
    bool tactics = false;                  // cover / side-step / flee-when-hurt combat styles (tactics.h); the module decides it from the rebalance mode
    uint32_t tickMs = 500;                 // how often the director looks at the fights
    uint32_t fadeMs = 8000;                // damage counts less with age and is gone after this
    float damageWeight = 1.0f;
    float nearBonus = 15;                  // a player right next to the enemy is worth this...
    float nearRange = 1500;                // ...down to nothing at this distance (game units)
    float seenBonus = 8;                   // the enemy can see the player
    float engagedBonus = 6;                // the enemy is already fighting the player (stickiness)
    float switchMargin = 1.25f;            // another player must beat the one it fights by this factor...
    float switchAbsolute = 4;              // ...plus this many points
    uint32_t minOnTargetMs = 3000;         // it stays on a target at least this long (unless the target is gone / downed)
    float spreadPenalty = 14;              // minus this per OTHER enemy of the group already on that player
    float groupRadius = 2500;              // "the group": enemies within this of the enemy
    float reach = 5000;                    // players further than this are never picked
    uint32_t sightMs = 1000;               // a sight check is good for this long
    uint32_t maxSightChecks = 24;          // sight checks per tick (the engine call is the costly part)
    uint32_t maxSwitches = 3;              // retargets per tick (each is two console lines)
    uint32_t forgetMs = 20000;             // an enemy not seen in a fight for this long is forgotten
    bool includeNamed = false;             // unique story NPCs: false = left to the story's rules
    // Damage the engine glue could not attribute (no hit feed): the health drop of an enemy is split between the players who were aiming at it.
    bool inferDamage = true;
    uint32_t feedQuietMs = 1500;           // an enemy a hit feed reported on within this is not inferred about
    float aimCone = 0.866f;                // cosine of the half angle a shooter's aim must be within (30 degrees)
    float aimRange = 6000;
    // The engine overruled a retarget (its own AI went back) this many times: leave that enemy to the engine for a while.
    uint32_t overrideLimit = 3;
    uint32_t overrideWindowMs = 2500;
    uint32_t quietMs = 20000;
};

// One player as the director sees them this tick.
struct PlayerView {
    Slot slot = kNoSlot;
    RefId ref = 0;
    uint32_t space = 0;                    // worldspace or interior cell: positions compare only within one space
    Vec3 pos;
    float yaw = 0;                         // radians, 0 = +Y, clockwise (the engine's rotZ)
    bool present = false;                  // loaded here with a position
    bool downed = false;                   // dead, dying or down: never picked, a target that goes down is left
    bool weaponOut = false;
};

// One actor of the loaded world as the director sees it this tick.
struct EnemyView {
    RefId ref = 0;
    bool mine = false;                     // this machine owns it, committed, no handoff in flight
    bool standIn = false;
    bool dead = false;
    bool teammate = false;                 // a companion (the player's teammate flag)
    bool unique = false;                   // a named story NPC
    bool inCombat = false;
    RefId combatTarget = 0;                // the engine ref it fights (0 = none)
    uint32_t space = 0;
    Vec3 pos;
};

enum class Sight : uint8_t { Unknown, Seen, Hidden };

// The one way out to the game. Production: threat_module.cpp (actors engine + avatar directory + the engine's detection check); tests: a fake.
class IThreatPort {
public:
    virtual ~IThreatPort() = default;
    virtual void players(std::vector<PlayerView>& out) = 0;
    virtual void enemies(std::vector<EnemyView>& out) = 0;                 // every loaded actor worth a look (the director filters)
    virtual Sight sight(RefId enemy, RefId playerRef) = 0;                 // can it see / has it noticed the player
    virtual bool health(RefId enemy, float& out) = 0;
    virtual bool retarget(RefId enemy, RefId playerRef) = 0;               // stop its fight and start one against the player
};

enum class Skip : uint8_t {
    None,
    NotMine,       // another machine runs it (or a handoff is in flight, or it is a stand-in)
    Gone,          // dead
    Companion,
    Story,         // a unique NPC the story's rules hold
    Calm,          // not fighting
    NotAboutPlayers,   // fighting something that is not a player (a companion, another faction)
    Count_
};
const char* skipName(Skip s);

struct Stats {
    uint64_t ticks = 0, enemiesSeen = 0, switches = 0, fresh = 0, held = 0, failed = 0, overrides = 0, quieted = 0;
    uint64_t sightChecks = 0, hitsNoted = 0, damageInferred = 0, damageNoted = 0;
    uint64_t skipped[static_cast<size_t>(Skip::Count_)] = {};
    uint64_t soloTicks = 0;                // fewer than two players here: nothing to split
};

}  // namespace nvcoop::threat
