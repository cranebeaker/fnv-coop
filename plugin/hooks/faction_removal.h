// RemoveFromAllFactions (G12): the factions an actor loses. Game-free; the engine reads (faction ids and ranks of the base record and of the
// runtime ExtraFactionChanges list) are in engine_hooks.cpp.
//
// The engine command (Cmd_RemoveFromAllFactions @0x5C8340) clears the actor's runtime ExtraFactionChanges list and marks the form modified. The
// effect list has no "leave everything" effect, so the capture spells it out: one SetFactionRank{actor, faction, -1} per faction the actor was
// in just before the command, which every machine applies with its own RemoveFromFaction (apply_lines.h removeFromFactionCmd).
//
// Effective membership = the base record's factions, overlaid by the runtime list (an entry in the runtime list replaces the base rank, a
// negative rank there means "removed").
#pragma once
#include <cstdint>
#include <vector>

namespace nvhooks {

struct FactionRankEntry {
    uint32_t faction = 0;   // runtime form id
    int rank = 0;           // engine byte rank, or negative = not a member
};

inline std::vector<uint32_t> factionsLeftBy(const std::vector<FactionRankEntry>& base, const std::vector<FactionRankEntry>& runtime) {
    std::vector<FactionRankEntry> eff;
    auto put = [&](const FactionRankEntry& e) {
        for (FactionRankEntry& x : eff)
            if (x.faction == e.faction) { x.rank = e.rank; return; }
        eff.push_back(e);
    };
    for (const FactionRankEntry& e : base) put(e);
#ifdef NVHOOKS_MUTANT
    // mutant: the runtime list is ignored, so a faction the script added (AddToFaction) before the clear is not announced and a removed one is
#else
    for (const FactionRankEntry& e : runtime) put(e);
#endif
    std::vector<uint32_t> out;
    for (const FactionRankEntry& x : eff)
        if (x.faction && x.rank >= 0) out.push_back(x.faction);
    return out;
}

}  // namespace nvhooks
