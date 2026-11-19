// Lines this machine only replays: another player's conversation, spoken here by the NPC (`Say` with the INFO forced, hooks_ui sayInfo) so the
// watcher hears it in sync. Saying an INFO runs its result scripts (begin, and end when the voice finishes): on a watcher they would apply the
// story a second time (a host watcher would set the stage again, hand out the reward again). The result-script hook asks isEcho() first.
// Game-free so the test runs it without the engine.
#pragma once
#include <cstdint>

#include "fnvcoop_mut.h"

namespace nvhooks {

class EchoLines {
public:
    static constexpr int kSlots = 8;
    void add(uint32_t info, uint32_t npc, uint64_t untilMs) {
        int slot = 0;
        for (int i = 0; i < kSlots; ++i) {
            if (e_[i].info == info && e_[i].npc == npc) { slot = i; break; }
            if (e_[i].until < e_[slot].until) slot = i;   // otherwise the oldest
        }
        e_[slot] = {info, npc, untilMs};
    }
    // True = skip the result script. `ownTalkWith` = the NPC of this player's own open conversation (0 = none): a line in it is always real.
    bool isEcho(uint32_t info, uint32_t npc, uint64_t nowMs, uint32_t ownTalkWith) {
        if (FNV_MUT(18) || !info || !npc || npc == ownTalkWith) return false;   // twin 18: replayed lines run their scripts again
        for (const auto& x : e_)
            if (x.info == info && x.npc == npc && nowMs <= x.until) { ++skipped_; return true; }
        return false;
    }
    uint64_t skipped() const { return skipped_; }

private:
    struct E { uint32_t info = 0, npc = 0; uint64_t until = 0; } e_[kSlots];
    uint64_t skipped_ = 0;
};

}  // namespace nvhooks
