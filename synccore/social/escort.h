// social/escort: bring a recruiter's companions along when the recruiter is moved (campaign gap G34: Fort entry and exit, the Dam start, any PartyWarp).
// C++17, no engine, no network: time and facts are passed in.
//
// The story scripts move "every hired companion" by name to one marker (and only the ones the script's flags name), the follower AI of the game only follows
// through doors the player WALKS through. A scripted move of a player (PartyWarp, a script's player.MoveTo, a teleporter) therefore leaves the companions of the
// players that were moved behind. Each recruiter's machine (the one that simulates the companion) watches its own player: a companion that is loaded here, alive,
// not told to wait, and farther than `farDistance` from its owner for `graceMs` is moved next to the owner (at most once per `cooldownMs`). A companion that is
// merely lagging a few seconds behind walks to the player by itself before the grace is over and is never touched.
#pragma once
#include <cstdint>
#include <map>
#include <vector>
#include "fnvcoop/types.h"
#include "social/rewards.h"

namespace fnvcoop::social {

struct EscortRules {
    uint32_t graceMs = 4000;        // how long a companion may be far away before it is brought
    uint32_t cooldownMs = 12000;    // per companion: the next bring-along not before this
    float farDistance = 3500;       // game units; a companion in another cell is normally far in these units too (interior and world coordinates differ)
};

struct EscortSubject {
    FormRef npc;
    bool loaded = false;            // a real actor exists here
    bool alive = true;
    bool waiting = false;           // told to wait (the companion script's Waiting variable, or the owner's wait command)
    uint32_t cell = 0;
    cc::Vec3 pos;
};

class Escort {
public:
    explicit Escort(EscortRules r = {}) : rules_(r) {}
    const EscortRules& rules() const { return rules_; }
    // `me` = the place of the player whose machine this is; `mine` = the companions this machine simulates for that player.
    std::vector<FormRef> poll(uint64_t nowMs, const PlayerPlace& me, const std::vector<EscortSubject>& mine);
    void forget(const FormRef& npc) { farSince_.erase(npc); lastBrought_.erase(npc); }
    void reset() { farSince_.clear(); lastBrought_.clear(); }
private:
    EscortRules rules_;
    std::map<FormRef, uint64_t> farSince_;
    std::map<FormRef, uint64_t> lastBrought_;
};

}  // namespace fnvcoop::social
