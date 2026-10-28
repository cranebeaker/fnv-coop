#include "social/escort.h"

#include "../../tools/mut/fnvcoop_mut.h"

namespace fnvcoop::social {

std::vector<FormRef> Escort::poll(uint64_t nowMs, const PlayerPlace& me, const std::vector<EscortSubject>& mine) {
    std::vector<FormRef> out;
    if (nowMs == 0) nowMs = 1;
    for (const EscortSubject& s : mine) {
        if (!s.loaded || !s.alive) { farSince_.erase(s.npc); continue; }
        if (s.waiting && !FNV_MUT(896)) { farSince_.erase(s.npc); continue; }   // BAD (896): a companion told to wait is dragged along anyway
        const bool far = cc::distance(s.pos, me.pos) > rules_.farDistance;
        if (!far) { farSince_.erase(s.npc); continue; }
        auto it = farSince_.find(s.npc);
        if (it == farSince_.end()) it = farSince_.emplace(s.npc, nowMs).first;
        if (nowMs - it->second < rules_.graceMs && !FNV_MUT(897)) continue;   // BAD (897): no grace, so a companion walking after its player is teleported at once
        auto last = lastBrought_.find(s.npc);
        if (last != lastBrought_.end() && nowMs - last->second < rules_.cooldownMs && !FNV_MUT(898)) continue;   // BAD (898): no cooldown, a failing bring-along repeats every poll
        lastBrought_[s.npc] = nowMs;
        farSince_.erase(s.npc);
        out.push_back(s.npc);
    }
    // forget companions that are no longer in the list (dismissed, dead, handed over)
    for (auto it = farSince_.begin(); it != farSince_.end();) {
        bool listed = false;
        for (const EscortSubject& s : mine) if (s.npc == it->first) { listed = true; break; }
        if (listed) ++it; else it = farSince_.erase(it);
    }
    return out;
}

}  // namespace fnvcoop::social
