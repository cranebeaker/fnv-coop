#include "rebalance/scarcity.h"

#include <cmath>

namespace nvcoop::rebalance {

namespace {

// FNV-1a over the net id and the item id: a stable number in [0,1) that every machine works out the same way.
float unitOf(uint32_t net, const std::string& id) {
    uint64_t h = 1469598103934665603ull;
    auto eat = [&h](uint8_t b) { h ^= b; h *= 1099511628211ull; };
    for (int i = 0; i < 4; ++i) eat(static_cast<uint8_t>(net >> (8 * i)));
    for (char c : id) eat(static_cast<uint8_t>(c));
    h ^= h >> 29; h *= 0xBF58476D1CE4E5B9ull; h ^= h >> 32;   // the last bytes of an id would otherwise barely move the high bits
    return static_cast<float>(static_cast<double>(h >> 11) / 9007199254740992.0);
}

}  // namespace

int32_t keptCount(uint32_t net, const std::string& id, int32_t count, float share) {
    if (count <= 0) return 0;
    if (share >= 1.0f) return count;
    if (share <= 0.0f) return 0;
    float u = unitOf(net, id);
    if (FNV_MUT(1473)) u = 0.0f;   // MUTANT 1473: always rounded down, so a stack of one never survives a share below 1
    const int32_t kept = static_cast<int32_t>(std::floor(static_cast<double>(count) * share + u));
    return kept < 0 ? 0 : kept > count ? count : kept;
}

int32_t trimLoot(const LootSource& src, std::vector<LootStack>& stacks, const Profile& p) {
    if (src.uniqueActor && !FNV_MUT(1471)) return 0;   // MUTANT 1471: a unique character's body is thinned too
    int32_t removed = 0;
    for (LootStack& s : stacks) {
        float share = 1.0f;
        if (s.kind == ItemKind::Supply && !FNV_MUT(1470)) share = p.suppliesKept;   // MUTANT 1470: supplies are never thinned
        else if (s.kind == ItemKind::Ammo && (src.corpse || FNV_MUT(1472))) share = p.bodyAmmoKept;   // MUTANT 1472: the ammo in boxes is thinned too
        if (share >= 1.0f) continue;
        const int32_t kept = keptCount(src.net, s.id, s.count, share);
        removed += s.count - kept;
        s.count = kept;
    }
    return removed;
}

}  // namespace nvcoop::rebalance
