// Revive, the host's side (game-free). The rules engine decides that a player is downed (death policy G41: docs/quest-policies/rules.json
// "policy" death.mode / death.revive_s); this decides whether a friend may get him up before the policy respawns him at the checkpoint.
//
//   a friend holds the activate key next to the downed player  ->  begin ... (holdMs) ... done      (cancel when the key is let go)
//   the host checks: the revive is on; the target is down, within his window and not "down until a checkpoint"; the reviver is up and
//   in the same space, within reach; nobody else is reviving him; a done comes holdMs after its begin (a little early is allowed for lag).
//   Ok on a done = the target gets up in place (the service asks the rules engine to forget his death and orders the respawn).
//
// Mutation twins 1351..1375.
#pragma once
#include <functional>
#include <map>
#include <vector>

#include "mk_types.h"
#include "mk_wire.h"

namespace nvmk {

struct ReviveRules {
    bool enabled = true;           // [revive] enabled
    uint32_t holdMs = 3000;        // how long the key is held
    uint32_t slackMs = 400;        // a done may come this much before holdMs has passed on the host (latency and clock jitter)
    float hostReachUnits = 600.0f; // the host's own check (the friend's machine uses a tighter one): generous for lag and blasts
    uint32_t holdTtlMs = 8000;     // a begin that is never finished or cancelled is dropped after this
    float healthFraction = 0.30f;  // the revived player gets up with this share of his health
};

enum class Verdict : uint8_t {
    Ok, Disabled, UnknownPlayer, SelfRevive, TargetNotDown, NotRevivable, WindowOver, ReviverDown, OtherSpace, TooFar, Busy, NotHolding, TooEarly,
};
const char* verdictName(Verdict v);

using PlaceFn = std::function<bool(Slot, Place&)>;   // where a player is, as far as the host knows (a puppet's place first, else the last beacon)

class ReviveAuthority {
public:
    explicit ReviveAuthority(ReviveRules r = {}) : rules_(r) {}
    void setRules(const ReviveRules& r) { rules_ = r; }
    const ReviveRules& rules() const { return rules_; }

    // Each frame, for every player in the party: what the rules engine says. `windowMs` = how long a downed player waits for a revive (0 = none:
    // death.mode respawn). Edge-triggered: the window starts when the player is first seen downed.
    void sync(Slot s, bool downed, bool checkpointOnly, uint32_t windowMs, uint64_t now);
    void forget(Slot s);   // left the party
    void clear();

    Verdict request(Slot from, RevivePhase ph, Slot target, uint64_t now, const PlaceFn& where);
    // Drops holds whose reviver let go for good (left, went down, walked off, never finished).
    void tick(uint64_t now, const PlaceFn& where);

    struct Revived { Slot target = kNoSlot; Slot reviver = kNoSlot; };
    std::vector<Revived> takeRevived();

    bool isDowned(Slot s) const { return down_.count(s) > 0; }
    bool checkpointOnly(Slot s) const;
    uint32_t windowLeftMs(Slot s, uint64_t now) const;
    Slot reviverOf(Slot s) const;
    uint32_t holdLeftMs(Slot s, uint64_t now) const;

private:
    struct Down {
        uint64_t since = 0;
        uint32_t windowMs = 0;
        bool checkpointOnly = false;
        Slot reviver = kNoSlot;
        uint64_t holdSince = 0;
    };
    Verdict check(Slot from, Slot target, uint64_t now, const PlaceFn& where) const;
    ReviveRules rules_;
    std::map<Slot, bool> known_;     // slot -> downed (everybody sync() has seen)
    std::map<Slot, Down> down_;
    std::vector<Revived> revived_;
};

}  // namespace nvmk
