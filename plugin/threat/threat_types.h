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

// All the knobs (nvcoop.ini [threat], docs/threat.md). The reasoning for every default is in docs/threat.md section "Where the numbers come from"; the short form:
//   * Scores are in "points of damage". One scale anchors the rest: a second of ordinary fire is 20 points (two hits of ten).
//   * Distances are in game units. A character is about 128 units tall (about 1.8 m), so a metre is about 70 units, and an exterior cell is 4096 units wide.
//   * Times are multiples of the tick, because nothing here can change faster than the director looks.
struct Config {
    bool enabled = true;
    bool tactics = false;                  // cover / side-step / flee-when-hurt combat styles (tactics.h); the module decides it from the rebalance mode
    uint32_t tickMs = 500;                 // how often the director looks at the fights: a human retargets in a quarter to a half second, and each retarget costs the
                                           // owner two console lines, so looking more often than this buys nothing
    uint32_t fadeMs = 8000;                // damage counts less with age and is gone after this: two volleys of a typical four second burst-and-reload cycle
    float damageWeight = 1.0f;             // the unit: a point of damage is a point of score
    float nearBonus = 20;                  // a player right next to the enemy is worth a second of fire (it can melee or shotgun the enemy)...
    float nearRange = 1500;                // ...down to nothing at this distance, about 21 m, the reach of pistols, shotguns and SMGs
    float seenBonus = 10;                  // the enemy can see the player: half a second of fire (it can shoot them now, but has not hurt them yet)
    float engagedBonus = 10;               // the enemy already fights the player: as much as sight, so seeing someone else never outweighs the fight it is in
    float switchMargin = 1.3f;             // another player must beat the one it fights by this factor...
    float switchAbsolute = 5;              // ...plus a quarter second of fire, so two near-zero scores cannot swap on rounding noise
    uint32_t minOnTargetMs = 4000;         // it stays on a target at least this long (unless the target is gone / downed): turning to a new target costs the enemy
                                           // about a second and a half of aiming, so a swap sooner than ~2.5 s of shooting later is a net loss
    float spreadPenalty = 20;              // minus this per OTHER enemy of the group already on that player: a second of fire. One step of balancing (leaving a player
                                           // with one more enemy than another) must beat the stickiness plus the hysteresis (10 + 5), or a group stops at 3-1-2 instead
                                           // of 2-2-2; 20 gives a margin of 5 and the tests check that six enemies over three players end two each
    float groupRadius = 2048;              // "the group": enemies within this of the enemy, half a cell, about 29 m: a squad that fights together
    float reach = 4096;                    // players further than this are never picked: one cell, about 58 m, the farthest a fight is worth walking to
    uint32_t sightMs = 1000;               // a sight check is good for this long (two ticks)
    uint32_t maxSightChecks = 24;          // sight checks per tick (the engine call is the costly part): six enemies times a party of four
    uint32_t maxSwitches = 3;              // retargets per tick (each is two console lines): six lines a tick at most
    uint32_t forgetMs = 30000;             // an enemy not seen in a fight for this long is forgotten: longer than the quiet period, so a quiet enemy is not forgotten
    bool includeNamed = false;             // unique story NPCs: false = left to the story's rules
    // Damage the engine glue could not attribute (no hit feed): the health drop of an enemy is split between the players who were aiming at it.
    bool inferDamage = true;
    uint32_t feedQuietMs = 1500;           // an enemy a hit feed reported on within this (three ticks) is not inferred about
    float aimCone = 0.866f;                // cosine of the half angle a shooter's aim must be within: 30 degrees, a little inside the half field of view of 37
                                           // degrees at the game's default 75 degree field of view, where a player keeps a target they are shooting at
    float aimRange = 6144;                 // shooters further than this are not counted: 1.5 cells, the farthest a rifle shot is taken in the loaded area
    // The engine overruled a retarget (its own AI went back) this many times: leave that enemy to the engine for a while. One overrule can be a race, two a
    // coincidence, three within ten seconds a pattern.
    uint32_t overrideLimit = 3;
    uint32_t overrideWindowMs = 2500;      // "went back right after we moved it": within five ticks
    uint32_t quietMs = 20000;              // long enough for the engine's own fight to run its course (a firefight is 10 to 20 s)
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
