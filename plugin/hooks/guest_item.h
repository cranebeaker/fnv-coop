// A guest's hand-in must not take the item from the HOST's pack (guest pack facts). Game-free.
//
// A dialogue result script of a guest's pick runs on the host, where `player` is the host's own player (0x14). `player.RemoveItem` therefore ran on the host's inventory: the host
// lost the totems / roots / lunchboxes while the guest that really carried them kept them (and the rules charged the guest as well). While a GuestItemScope is alive the engine call
// is skipped and the removal is only recorded; the rules engine splits it over the holders (PooledItems, with the pack facts of every guest) or charges the initiator, the txn reaches
// each guest's machine as an order for that machine's player (RemoteApplier::applyTxn, Recipient), and the host's own share, if any, is taken by the session
// (Runtime::onHostCommitted) from the host's pack.
#pragma once
#include <cstdint>

#include "fnvcoop_mut.h"

namespace nvhooks {

class GuestItems {
public:
    void push(uint8_t slot) { if (depth_ < 8) slots_[depth_++] = slot; }
    void pop() { if (depth_) --depth_; }
    // A guest (slot above 0) is the player the running script acts for.
    bool active() const { return depth_ > 0 && slots_[depth_ - 1] != 0; }
    uint8_t slot() const { return depth_ ? slots_[depth_ - 1] : 0; }

private:
    uint8_t slots_[8] = {};
    int depth_ = 0;
};

struct RemovePlan {
    bool runEngine = true;   // the original RemoveItem runs (on the host's own player)
    bool record = false;     // the removal is recorded for the rules and the other machines
};
// `onPlayer`: the command's calling reference is the player (0x14); `guestActing`: a GuestItemScope is open; `capturing`: a capture scope is open; `parsed`: the item and the count were read.
inline RemovePlan planPlayerRemoveItem(bool onPlayer, bool guestActing, bool capturing, bool parsed) {
    RemovePlan p;
    p.record = capturing && parsed;
#ifndef NVHOOKS_MUTANT
    if (onPlayer && guestActing && capturing && parsed) p.runEngine = false;   // the host's pack is not the guest's
#else
    (void)onPlayer; (void)guestActing;   // mutant: a guest's hand-in is taken from the host's own pack
#endif
    return p;
}

// The same for an item a guest's script GIVES (`player.AddItem` in a guest's result or in an activator script run for a guest): the engine call would put it into the host's pack
// whatever the committed mask says (the guest alone for a non-quest cause: the host got a free copy). While a guest acts, the engine call is skipped and the item is only
// recorded; the guest's machine gets it through the txn, and the host's own share, if the mask names the host, is applied by the session (Runtime::onHostCommitted), once.
// A leveled list / form list still goes through the engine (the resolved items are read back from the pack).
inline RemovePlan planPlayerAddItem(bool onPlayer, bool guestActing, bool capturing, bool parsed, bool resolving) {
    RemovePlan p;
    p.record = capturing && parsed;
    if (FNV_MUT(98)) return p;   // MUTANT 98: the engine puts a guest's reward into the host's pack as well (the session then adds the host's share again)
    if (onPlayer && guestActing && capturing && parsed && !resolving) p.runEngine = false;
    return p;
}

// XP, karma and reputation a guest's script gives (RewardXP, RewardKarma, ModActorValue Karma, AddReputation ...): the same rule. The engine would move the HOST's player/standing
// whatever the committed mask says, so a guest-only reward (a non-quest cause) leaked to the host. While a guest acts the engine's change is not made (XP, karma) or put back (the
// engine computes the net of a reputation change itself), the effect is recorded, and the session gives the host its share when the mask names it (Runtime::onHostCommitted).
inline RemovePlan planGuestReward(bool guestActing, bool capturing, bool parsed) {
    RemovePlan p;
    p.record = capturing && parsed;
    if (FNV_MUT(100)) return p;   // MUTANT 100: a guest's XP / karma / reputation also lands on the host's player (the session then adds the host's share again)
    if (guestActing && capturing && parsed) p.runEngine = false;
    return p;
}

}  // namespace nvhooks
