// Revive, the friend's side (game-free): holding the activate key next to a downed player. The tracker decides begin / cancel / done and the
// progress bar; the host judges the result (rv_authority.h).
//
// Mutation twins 1381..1395.
#pragma once
#include <vector>

#include "mk_types.h"

namespace nvmk {

struct HoldConfig {
    uint32_t holdMs = 3000;         // [revive] hold_ms
    float reachUnits = 150.0f;      // [revive] reach: how close to stand to a downed friend (about 2 m)
    uint32_t cooldownMs = 3000;     // after a done: the downed player's health takes a moment to come back
};

struct HoldCandidate {
    Slot slot = kNoSlot;
    float distUnits = 0;
    bool revivable = false;         // downed, inside his window, not "down until a checkpoint"
    bool busyWithOther = false;     // somebody else is already holding the key on him
};

struct HoldInput {
    uint64_t now = 0;
    bool selfUp = true;             // the local player is not down himself
    bool activateHeld = false;
    bool blocked = false;           // a menu, a dialogue or the chat is open
    std::vector<HoldCandidate> candidates;
};

enum class HoldEvent : uint8_t { None, Begin, Cancel, Done };
struct HoldOut {
    HoldEvent ev = HoldEvent::None;
    Slot target = kNoSlot;
};

class ReviveHold {
public:
    explicit ReviveHold(HoldConfig c = {}) : cfg_(c) {}
    void setConfig(const HoldConfig& c) { cfg_ = c; }
    const HoldConfig& config() const { return cfg_; }

    // One event per call at most (a target change is a Cancel now and the Begin on the next call).
    HoldOut step(const HoldInput& in);
    void reset();

    bool active() const { return target_ != kNoSlot; }
    Slot target() const { return target_; }
    Slot candidate() const { return candidate_; }   // who would be revived by holding the key now (for the prompt), kNoSlot if nobody
    float progress(uint64_t now) const;             // 0..1 while active

private:
    HoldConfig cfg_;
    Slot target_ = kNoSlot;
    Slot candidate_ = kNoSlot;
    uint64_t startMs_ = 0;
    uint64_t cooldownUntil_ = 0;
    bool haveCooldown_ = false;
};

}  // namespace nvmk
