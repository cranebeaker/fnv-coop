// Co-op balanced: scarcer supplies and ammo (docs/rebalance.md). The rule that thins what a body or a box holds, game-free. The loot service (plugin/loot) calls it
// ONCE per container or corpse, on the machine that registers it, before the contents become the shared ones (LootSync::setTrimHook); this file never touches the game.
//
// Rules, by what the stack is and where it lies:
//   * food, drink and medicine (Supply): every box and every body keeps `suppliesKept` of each stack
//   * ammo: only a body keeps `bodyAmmoKept` of it (a box of ammo is somebody's stash and stays whole)
//   * a unique character's body and boxes are left alone (their kit is written by hand, often for a quest)
// The share is exact in the long run and the same on every machine: a stack of n keeps floor(n x share + u), where u in [0,1) comes from the container's net id and
// the item's id, never from a random number. So a stack of one at 0.5 survives in half of the containers and the answer for one container never changes.
#pragma once
#include <cstdint>
#include <string>
#include <vector>

#include "rebalance/balance.h"

namespace nvcoop::rebalance {

enum class ItemKind : uint8_t { Other, Ammo, Supply };

struct LootStack {
    std::string id;                 // the item's stable id ("plugin:localId")
    int32_t count = 0;
    ItemKind kind = ItemKind::Other;
};

struct LootSource {
    uint32_t net = 0;               // the container's or the corpse's net id
    bool corpse = false;
    bool uniqueActor = false;       // the body of a unique character
};

// How many of `count` stay at `share` (0..1) for this container and item. share >= 1 keeps all, <= 0 keeps none.
int32_t keptCount(uint32_t net, const std::string& id, int32_t count, float share);

// Thins the stacks in place (a stack may end at 0). Returns how many items went. The profile is the TARGET one (the numbers the campaign is heading to, not a
// point of the smooth change), so a body looted while the party size settles is not thinned by a half-way share.
int32_t trimLoot(const LootSource& src, std::vector<LootStack>& stacks, const Profile& p);

}  // namespace nvcoop::rebalance
